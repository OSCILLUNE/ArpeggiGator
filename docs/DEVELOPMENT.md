# Development checklist

## Prototype
- [x] Pin the MPC Sample SDK revision.
- [x] Record the firmware 1.3.0.12 target and fingerprint.
- [x] Map B1/B2 and SHIFT hardware codes.
- [x] Implement the eight-pattern held-pad engine.
- [x] Add native controls and reusable gator artwork.
- [ ] Run CI formatting, unit tests and Clippy.

## Audible playback
- [ ] Verify the lifetime of input and per-pad properties pointers.
- [ ] Identify the active sequence BPM / transport tick source.
- [ ] Implement a non-blocking scheduler without NOTE REPEAT.
- [ ] Trigger pads through the native note path.
- [ ] Track active notes and guarantee note-off cleanup.
- [ ] Add rate and gate parameters.
- [ ] Test pattern changes while pads are held.

## Release
- [ ] Build through the SDK script.
- [ ] Load in the virtual MPC Sample emulator.
- [ ] Confirm no crash, stuck notes or change to project data.
- [ ] Test on physical firmware 1.3.0.12.
- [ ] Publish a device-installable ZIP only after all checks pass.

## Animation input investigation

The pinned SDK originally dispatched surface events, pad events, and paint
callbacks through the same non-blocking module lock. In the emulator, releasing
the last of two physical pads reached the surface event path but did not reach
`on_pad`, leaving the old animation state active. Observing the raw note-off
hook was also insufficient in this 16 Levels scenario.

`vendor/mpc-sample-sdk` retains the original SDK revision and adds an optional
lock-free surface observer. The animation tracks the physical pad events there
with an atomic bit mask, including a repaint after the final release. The
observer uses no retained pointers, allocations, or blocking operations. The
emulator's observed pad protocol uses MIDI channel 10 and codes 0–15; this
assumption still needs confirmation on physical hardware. This fixes the visual
preview state only; the existing arpeggiator engine and audio path require
separate input-lifetime and event-delivery work.

The diagnostic emulator run showed physical pad state 2 → 1 → 0 after the
change, including a case where the normal pad callback missed the final
release. The user confirmed the animation works on the emulator. Physical hardware
validation remains outstanding.
