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

No inspected API in the pinned SDK provides a verified periodic callback on
the app's owning UI thread. Consequently, there is no currently justified
way to call native note-on/note-off at future step times while satisfying the
SDK's pointer lifetime and thread contracts.

The next investigation should inspect the firmware's input/event queue and the
SDK's hook installation points for a supported enqueue/dispatch path. Any
candidate must be validated on the virtual MPC Sample emulator, including
re-entrancy, note-off matching, mode exit and unload. Do not add cached pointers,
new firmware offsets, or speculative cross-thread calls as an interim solution.

## Test status

Source/documentation inspection only. No emulator or physical MPC runtime test
has been performed as part of this investigation. No playback behavior is
claimed as working.
