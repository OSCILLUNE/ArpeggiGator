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
- [x] Inspect pinned SDK contracts for pad input/properties lifetime and thread ownership (see [PLAYBACK-RESEARCH.md](PLAYBACK-RESEARCH.md)); no safe deferred use established.
- [x] Add host-tested, platform-independent rate/gate timing engine (not yet connected to MPC).
- [ ] Find a reliable transport phase or beat clock, not just BPM.
- [ ] Validate whether the SDK repaint-to-paint path can safely dispatch on the app UI thread; measure callback thread identity and timing in the emulator.
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