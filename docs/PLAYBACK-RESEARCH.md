# Playback architecture investigation

## Confirmed SDK contracts

This note records a source-level investigation of the pinned SDK revision
`87d3ec20e6b8d46b0580a4fd7f0f39ca3fcd7caf`.

### Native pad trigger hooks

The SDK exposes `sites::send_note_on` and `sites::send_note_off`.
Their safety contracts require:

- the `input` object to be live;
- the `properties` object passed to note-on to be live;
- calls to run on the app's owning UI thread;
- source and pad values to identify a valid note.

The pad hook delivers both pointers on press. The release hook only supplies
`input`, `source`, and `pad`; SDK code constructs the release event with
`properties: 0`. A press callback therefore does not establish a durable
context for later scheduler calls.

Do not cache these addresses for use from a worker or audio callback unless
their lifetime and thread ownership are separately proven on the emulator and
device.

### Task and audio callbacks

The SDK `Task<T>` API runs an owned closure on a worker. Its `take()` method
is a non-blocking result poll; it is not a periodic UI-thread scheduler.

The SDK's `AudioHooks` are invoked on the audio thread without the module
mutex. The audio contract allows atomics and lock-free state, but prohibits
allocation, locks, drawing, toasts and blocking. It does not authorize calling
the native UI pad hooks from the audio thread.

### UI scheduling boundary

The pinned SDK's `Task<T>` runs work on a worker and exposes `take()` as a
non-blocking poll; it does not post a closure to the UI thread. The drawing
helper `repaint_painted()` requests a repaint of the last painted component.
The SDK exposes no general `callAsync`, message-queue, or arbitrary UI-callback
dispatcher. In the emulator, a repaint request did reach `on_paint` on the main
thread, but this repaint-specific wakeup has no transport timing guarantee.

The SDK repaint pump runs on its own worker thread and invokes the app's
repaint function for a component. It is specifically a repaint mechanism, not
a general UI callback dispatcher. It can wake `on_paint` in the measured
emulator, but does not supply a future-event queue or a transport clock.

### Repaint-to-paint handoff candidate

The SDK repaint pump runs a worker that periodically calls the app repaint function. The SDK drawing documentation says `on_paint` receives a live Graphics object on the UI thread, and the Task example requests a repaint from a worker after its job completes. This suggests a possible handoff: a timer worker publishes due events through atomics, requests a repaint, and the native paint callback consumes the events on the app thread. However, the inspected source does not establish that the repaint function always marshals painting to the owning UI thread, nor does it establish callback timing or maximum useful frequency. This is a candidate for an emulator probe, not a verified scheduler.

Safe emulator runs issued 128 repaint requests at 4 ms intervals, then observed callbacks for a 750 ms drain period. The first recorded 239 `on_paint` calls and 238 intervals: 327 us minimum, 2,216.8 us mean, and 9,896 us maximum. A later run recorded 248 calls and 247 intervals: 580 us minimum, 2,145 us mean, and 5,004 us maximum. The callback counter includes every paint during the active window and can exceed the request count, so this does not show a one-to-one request/callback mapping.

The later run recorded `on_pad` on TID 1188 (`Audio Processin`), `on_paint` on TID 1150 (`MPC Sample Main`), and the repaint worker on TID 1288. The paint TID matched the process leader, consistent with the SDK's UI-thread paint contract; the `on_pad` callback came from a different, audio-processing thread. The worker also differed from the paint thread, so the repaint request did not invoke `on_paint` inline on its caller in this run. The probe did not retain native pointers or call either note hook.

These measurements show that repaint requests reach the main/UI paint thread in this emulator session. They do not identify a 4 ms clock or establish phase lock. Since the input hook ran on the audio-processing thread while the paint hook ran on the process leader, the `PadEvent.input` and `properties` addresses must not be carried across to `on_paint` without a separate lifetime and ownership contract. The built-in repaint pump ticks every 60 ms, which is too coarse to assume accurate 1/16 or faster playback. A higher-rate notifier would still need testing for repaint coalescing, UI load and jitter.

### MIDI replay

`NoteEvent::replay_as_pad` rewrites an existing app MIDI event during its
borrowed callback. The SDK forwards that same event and restores its original
bytes. It does not create a future or independently scheduled MIDI event.

The Minimod implementation in `rust/crates/minimod/src/midi.rs` is a useful
reference for tracking note-on/off pairs and replaying external MIDI notes as
pads. It is not an autonomous arpeggiator scheduler.

### Screen/context detection

The SDK's paint hook reports `PaintEvent.sample_page` and tracks the live
Sample page component. `platform::page::visible(screen)` checks that page's
visibility, but neither this predicate nor the paint callback documents a
specific 16 Levels screen-state query.

The current ArpeggiGator `in_16_levels` flag is toggled from the 16 LEVELS
button event and forced true by `on_pitch`. This is only a heuristic; it can
become stale when navigation happens through another route. Do not treat it as
verified screen detection. A reliable mode detector still needs emulator
validation and, if required, SDK/firmware research.

## Current conclusion

The pinned SDK provides a route from a worker repaint request to an `on_paint` callback on the main/UI thread in the virtual MPC. The pad hook ran on a different audio-processing thread in the measured run, and callback timing varied. The safe probe does not retain pointers or call native note hooks. Consequently, there is not yet a verified way to dispatch native note-on/note-off at future step times while satisfying the SDK's pointer lifetime and thread contracts.

The next investigation should inspect the firmware's supported input/event dispatch path and determine whether audio-thread input context can be represented without retaining raw `PadEvent` pointers. Any candidate must verify pointer lifetime, re-entrancy, matched note-off, mode exit and unload. Do not add speculative cross-thread native calls or new firmware offsets as an interim solution.

## Test status

Source/documentation inspection, host-side scheduler unit tests, and one safe repaint-cadence run on the virtual MPC Sample emulator. No physical MPC or audio assertion was part of the investigation. The timing engine is not connected to native playback; no audible arpeggiator behavior is claimed as working.
