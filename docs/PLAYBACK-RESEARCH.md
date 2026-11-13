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

The SDK repaint pump runs on its own worker thread and invokes the app's
repaint function for a component. It is specifically a repaint mechanism, not
a general UI callback dispatcher. It must not be repurposed to call
`send_note_on/off`.

### Repaint-to-paint handoff candidate

The SDK repaint pump runs a worker that periodically calls the app repaint function. The SDK drawing documentation says `on_paint` receives a live Graphics object on the UI thread, and the Task example requests a repaint from a worker after its job completes. This suggests a possible handoff: a timer worker publishes due events through atomics, requests a repaint, and the native paint callback consumes the events on the app thread. However, the inspected source does not establish that the repaint function always marshals painting to the owning UI thread, nor does it establish callback timing or maximum useful frequency. This is a candidate for an emulator probe, not a verified scheduler.

One safe emulator run issued 128 repaint requests at 4 ms intervals, then observed callbacks for a 750 ms drain period. It recorded 239 `on_paint` calls and 238 intervals: 327 us minimum, 2,216.8 us mean, and 9,896 us maximum. The callback counter includes every paint during the active window and can exceed the request count, so this does not show a one-to-one request/callback mapping. The `on_pad` and `on_paint` thread fingerprints differed in this run. The test did not retain native pointers or call either note hook.

These measurements show repeated paint callbacks in this emulator session, but they do not identify a 4 ms clock, establish phase lock, or prove that the paint callback is the thread required by the native note hooks. The built-in repaint pump ticks every 60 ms, which is too coarse to assume accurate 1/16 or faster playback. A higher-rate notifier would still need testing for repaint coalescing, UI load and jitter, with its native dispatch thread identified independently.

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

The pinned SDK provides repaint callbacks, but the observed thread fingerprints differ from the pad hook and callback timing is variable. The safe probe does not retain pointers or call native note hooks. Consequently, there is not yet a verified way to dispatch native note-on/note-off at future step times while satisfying the SDK's pointer lifetime and thread contracts.

The next investigation should identify the owning UI thread independently and inspect the firmware's supported input/event dispatch path. Any candidate must verify pointer lifetime, re-entrancy, matched note-off, mode exit and unload. Do not add speculative cross-thread native calls or new firmware offsets as an interim solution.

## Test status

Source/documentation inspection, host-side scheduler unit tests, and one safe repaint-cadence run on the virtual MPC Sample emulator. No physical MPC or audio assertion was part of the investigation. The timing engine is not connected to native playback; no audible arpeggiator behavior is claimed as working.
