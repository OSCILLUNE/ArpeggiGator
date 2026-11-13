# Roadmap

## Increment 0 — repository bootstrap
- [x] Pin the target firmware and SDK revision.
- [x] Add the first splash artwork and native pixel-art startup mark.
- [x] Implement the host-testable eight-pattern engine.
- [x] Bind SHIFT+B1 and SHIFT+B2 in 16 Levels.
- [ ] Host CI and compile verification.

## Increment 1 — playback
- [ ] Prove the lifetime and thread contract for the live pad input/properties objects.
- [ ] Identify a project-tempo / sequencer-clock source on firmware 1.3.0.12.
- [ ] Add a non-blocking scheduler driven by the verified clock.
- [ ] Trigger and release pads through the native note path.
- [ ] Handle release, disable, transport stop and mod unload without stuck notes.

## Increment 2 — performance controls
- [ ] Rate: 1/4, 1/8, 1/8T, 1/16, 1/16T, 1/32.
- [ ] Gate length.
- [ ] Octave range (only if the selected 16 Levels instrument supports it safely).
- [ ] Display active state and pattern in a compact native overlay.

## Increment 3 — release
- [ ] Run host tests and Clippy.
- [ ] Build with the SDK's canonical script.
- [ ] Load in the virtual MPC Sample emulator.
- [ ] Test on physical MPC Sample 1.3.0.12.
- [ ] Publish an SD-card ZIP only after all checks pass.
