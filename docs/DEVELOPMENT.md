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
