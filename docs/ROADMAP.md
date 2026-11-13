# Roadmap

## Increment 0 — repository bootstrap
- [x] Pin the target firmware and SDK revision.
- [x] Add splash artwork and native pixel-art startup mark.
- [x] Implement the eight-pattern held-pad engine.
- [x] Bind SHIFT+B1 and SHIFT+B2 in 16 Levels.
- [x] Add read-only current-sequence/global BPM parser and host tests.
- [x] Host CI: formatting, unit tests and Clippy.

## Increment 1 — audible playback
- [ ] Verify that the temporary project file follows unsaved tempo edits on device.
- [ ] Prove the lifetime and thread contract for live pad input/properties objects.
- [ ] Find a reliable transport phase or beat clock, not just BPM.
- [ ] Add a non-blocking scheduler without NOTE REPEAT.
- [ ] Trigger and release pads through the native note path.
- [ ] Guarantee note-off on pad release, disable, transport stop and unload.

## Increment 2 — performance controls
- [ ] Rate: 1/4, 1/8, 1/8T, 1/16, 1/16T, 1/32.
- [ ] Gate length.
- [ ] Octave span where the active 16 Levels mapping supports it.
- [ ] Refine the compact status overlay.

## Increment 3 — release
- [ ] Build through the SDK's canonical script.
- [ ] Load in the virtual MPC Sample emulator.
- [ ] Test on physical MPC Sample 1.3.0.12.
- [ ] Publish a device-installable ZIP only after all checks pass.