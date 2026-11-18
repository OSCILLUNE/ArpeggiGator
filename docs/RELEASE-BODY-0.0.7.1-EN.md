# ArpeggiGator v0.0.7.1 — first-pad fix

**Tag:** `v0.0.7.1`
**Version string:** `0.0.7-1`
**Target firmware:** stock Akai MPC Sample 1.3.0.12

ArpeggiGator is a standalone arpeggiator mod for the Akai MPC Sample. This release ports the verified `0.0.6.5` fix into the shipping line: `Playback::press()` now rejects presses while `EngineMode::Off` before latching the physical pad bit, and `clear()` resets `physical` when a mode jump passes through Off.

## What changed

- Fixed the intermittent issue where the first PAD press after SHIFT+B1 into Mutator/Acordator with HOLD ran the animation but produced no notes.
- Added two regression tests; the suite is now **84/84** (previously 82/82). Both new tests fail on the unfixed code and pass with the fix.
- No other mechanisms changed: the native audio engine, volatile `/run` loading, firmware guard, pad-off cleanup and all 20 integrity checks are identical to v0.0.7.0.

## Verification

- `cargo test`: **84/84 PASS**.
- `python3 scripts/verify-v070.py`: **20/20 PASS**.
- A/B check on the QA emulator with an identical method: the baseline build produced notes in **0/20** trials; the fixed build produced notes in **19/19** valid trials (one attempt was discarded as a harness error).
- Package integrity: the ZIP hash and its `.sha256` sidecar check out with `shasum -a 256 -c`.

## Downloads — one codebase, two delivery profiles

| Artifact | Use | SHA-256 |
|---|---|---|
| `arpeggigator-v0.0.7.1.zip` | SD card → physical MPC Sample 1.3.0.12. Unzip and copy the `autorun/` folder to the card. | `66846aaaf73b06b218ebc63c9ecea8606e97c14e7f9aecd677bc1aea8f4678e9` |
| `arpeggigator-v0.0.7.1-lab.so` | Emulator / QA-DEV via `python3 lab.py preload …`. **This is not an SD-card package.** | `66e4c6cbd5125c15af83da06b29d7f6cc94e157f1cdfaa0fe847f13899ab6df8` |

Both artifacts are published under this release. The `.sha256` sidecar for the SD-card ZIP is attached here too. Verify the package before inserting the card:

```
shasum -a 256 -c arpeggigator-v0.0.7.1.zip.sha256
```

## Safety notes

- The module is copied only to volatile `/run`; the installer adds only `/run` systemd drop-ins. Nothing is written under `/etc`, `/data`, `/content` or `/factory`.
- A firmware guard checks the exact 1.3.0.12 signature before the first copy.
- After a restart the console returns to its pre-insertion state (verified in the emulator). Pulling the card without a restart leaves the running module loaded until power cycle.
- Emulator and QA validation is complete; verification on physical hardware is still outstanding.

## Links

- User manual: https://oscillune.github.io/ArpeggiGator/
- Changelog: https://github.com/OSCILLUNE/ArpeggiGator/blob/main/CHANGELOG.md
- Build instructions: https://github.com/OSCILLUNE/ArpeggiGator#build
