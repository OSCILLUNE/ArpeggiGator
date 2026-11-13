# Roadmap

## Increment 0 — repository bootstrap
- [x] Pin the target firmware and SDK revision.
- [x] Create reusable pixel-art gator brand artwork.
- [x] Implement the eight-pattern held-pad engine.
- [x] Bind SHIFT+B1/B2/B3 in 16 Levels.
- [x] Add read-only current-sequence/global BPM parser and host tests.
- [x] Host CI: formatting, unit tests and Clippy.

## Increment 1 — audible playback
- [ ] Verify that the temporary project file follows unsaved tempo edits on device.
- [x] Inspect pinned SDK contracts for pad input/properties lifetime and thread ownership (see [PLAYBACK-RESEARCH.md](PLAYBACK-RESEARCH.md)); no safe deferred use established.
- [x] Add host-tested, platform-independent rate/gate timing engine (not yet connected to MPC).
- [ ] Find a reliable transport phase or beat clock, not just BPM.
- [x] Measure repaint callback timing and thread IDs in the emulator; `on_pad` ran on the audio-processing thread and `on_paint` on the process leader (see [PLAYBACK-RESEARCH.md](PLAYBACK-RESEARCH.md)).
- [x] Audit the AM MOD native event and Note Off paths against firmware 1.3.0.12; record the unresolved voice matching, recording, pointer-lifetime and audio-lock risks (see [NATIVE-DISPATCH-RESEARCH.md](NATIVE-DISPATCH-RESEARCH.md)).
- [ ] Verify a supported native note dispatch path and safe context handoff from the audio-processing thread.
- [ ] Trigger and release pads through the native note path.
- [ ] Guarantee note-off on pad release, disable, transport stop and unload.

## Increment 2 — arp controls and performance
- [ ] Rate: 1/4, 1/8, 1/8T, 1/16, 1/16T, 1/32.
- [ ] Gate length.
- [ ] Octave span where the active 16 Levels mapping supports it.
- [x] Keep native Sample and 16 Levels pages visible unless the user explicitly enables the arp.
- [x] Show ARP, pattern and +0/+1/+2/+3 (default +0) on the native softkey row while SHIFT is held in 16 Levels.
- [ ] Add a HOLD latch mode for source pads after note output is connected.
- [ ] Drive the gator's chewing animation from generated arpeggio note events.

## Increment 3 — release
- [ ] Build through the SDK's canonical script.
- [ ] Load in the virtual MPC Sample emulator.
- [ ] Test on physical MPC Sample 1.3.0.12.
- [ ] Publish a device-installable ZIP only after all checks pass.
