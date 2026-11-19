# Changelog

## 0.0.7.2

- Kosmetyczne poprawki napisów: `ACORDATOR` → `ACCORDATOR`, `MODE: ACORD` → `MODE: ACCORD`, oraz usunięcie podwójnego freeze w panelu MUTATOR (`SUBTLE FRZ OFF FRZ: OFF` → `SUBTLE FRZ:OFF`).
- Napisy wydzielone do testowalnych stałych/funkcji; dodane testy jednostkowe.
- Aktualizacja wszystkich publicznych powierzchni (README, manual, gh-pages) do nowych nazw. Bez zmian w audio, `Playback`, dropinach i guardzie firmware'u.

## 0.0.7.1

- Ported the verified 0.0.6.5 fix into the shipping line. `Playback::press()` now rejects presses while `EngineMode::Off` before latching the physical bit, and `clear()` resets `physical` on a mode jump through Off.
- Eliminates the intermittent "first PAD 4 press after SHIFT+B1 into Mutator/Acordator with HOLD: animation runs, no notes" bug. QA A/B with identical method: baseline `0/20` trials produced notes, fixed build `19/19` valid trials produced notes (`steps_delta` 9–29).
- Two regression tests added; the suite is now 84/84 (was 82/82). Both new tests were confirmed to FAIL on the unfixed code and PASS with the fix.

## 0.0.7.0

- Rebuilt from the clean `v0.0.6.0` source tree (`b912eed1028c9b9c54a166b36433cd349e782be0`); no fix is taken from a virtual machine, guest OS or a running `ac50` service. The repository source is the single source of truth.
- Version set to `0.0.7-0` in `Cargo.toml` and `Cargo.lock`.
- Renamed the misleading `emulator-audio` feature to `native-audio`. It is the main note engine, not an emulator detector: with it disabled the module is a UI-only overlay. The standard `scripts/build.sh` now sets `MPC_MOD_FEATURES=native-audio`, so the SD package contains the real arpeggiator. Firmware hooks remain gated by the exact 1.3.0.12 signature checks and the autorun firmware guard.
- Moved rotating diagnostics from `/tmp` to volatile `/run` (`/run/arpeggigator-6-events.log*`) so the mod leaves no file outside tmpfs.
- Added a stop flag for the read-only BPM monitor and a best-effort teardown in `Drop` (stop worker, panic the note state, extinguish pads). Committed firmware hooks cannot be unmapped by the SDK and are gone after the MPC restarts.
- Physical-safety contract: autorun copies the module to volatile `/run` and installs only `/run` systemd drop-ins with `ExecStopPost` pad cleanup. No change is made under `/etc`, `/data`, `/content` or `/factory`. The console returns to its pre-insertion state after a restart; pulling the card without a restart leaves the running mod loaded until power cycle.
- `scripts/verify-v070.py` performs 20 explicit PASS/FAIL integrity and safety checks; its report is `docs/VP-0.0.7.0-VERIFY-OUTPUT.md`.
- Full provenance table (source file → package entry → SHA-256) and line-by-line hardware-safety analysis are recorded in `docs/VP-0.0.7.0-*.md`.

## 0.0.6.0 UI fixes (0.0.1)

- Request a repaint when SHIFT+B1/B2/B3 or DATA/HOLD changes engine state, so the new screen and softkey captions appear while SHIFT is still held.
- Keep the overlay painted over the firmware playhead with a throttled (~30 Hz) repaint issued from the UI paint callback; HOLD state is now visible immediately.
- Serialize the rotating diagnostics file with a mutex because the heartbeat is emitted from both the UI and the tempo thread.

## 0.0.6.0

- Based on v0.0.4.9. No boot splash on the paint path (the old `draw_boot_splash` definition remains unused); no full-screen clear and no splash wake-up from audio.
- Removed the background `mod-repaint` pump after a proven SIGSEGV in that non-UI thread.
- Re-enabled generated notes in the emulator build by using `MPC_MOD_FEATURES=emulator-audio` (without it the audio hook is compiled out).
- MUTATOR: SHIFT+B3 toggles freeze; the caption reports `FRZ ON` / `FRZ OFF`, and the freeze state reaches both audio playback paths. The state is also visible as `FRZ:ON/OFF` in the overlay and `frz=` in the heartbeat.
- Added rotating diagnostics: atomics on the audio path, one line per second on journal plus `/tmp/arpeggigator-6-events.log` (4 files x 128 KiB). Heartbeat includes `mode/en/hold/l16/frz` and a startup-anchored `idle_ms`.
- The single-STOP-in-HOLD change was tested and reverted as a regression; STOP keeps v0.0.4.9 semantics.
- Staging-only feature gates `stage-arp-only` / `stage-melody-only` remain declared but are NOT enabled in the final build.
- Deferred: `map_surface_pad`, `PANIC_COUNT`, and the unload-time voice cleanup (see the live handoff).

## 0.0.4.9

- Turn off all pads when resetting the engine, on double STOP and when the module drops.
- Add a systemd post-stop pad cleanup to the autorun package; shutdown and restart were verified in the emulator. Physical hardware is not yet verified.

## 0.0.2.2

- Fix ArpeggiGator SHIFT menu visibility and state tracking across view transitions (Sample, SEQ, Pad FX, Sample Select, Mute, shortcuts).
- Forward SHIFT+B1..B3 buttons when outside 16 Levels to restore native sample functions (Loop Lock, Normalize).
- Maintain active 16 Levels during SHIFT+16 Levels TYPE shortcut and native SAMPLE presses.
- Restore native pad LEDs when leaving 16 Levels and restore ArpeggiGator state on re-entry.

## 0.0.2.1

- Add up to three octave-transposed notes to emulator audio patterns; all eight patterns include them in their ordering.
- Prevent SHIFT+pad shortcuts from changing the HOLD selection and retry transient project-tempo read failures.
- Verify all patterns with two additional notes before and after an emulator reboot; generated Note On and Note Off counts matched.
- Audio remains experimental and is disabled in the standard hardware package. Transport alignment, recording behaviour, pitch limits and screen-dependent pad lighting remain under investigation.

## 0.0.2.0

- Add an opt-in emulator audio build with native pad playback, pattern and rate selection, and HOLD.
- Record generated notes with SEQ RECORD and play them alongside live arpeggios.
- Allow the emulator audio build to persist on the existing autorun card.
- Audio is experimental and disabled in the standard hardware package. An unavailable sequence tempo prevents playback; transport phase and additional notes remain unfinished.

## 0.0.1.6

- Add an optional real-time MIDI interceptor to the vendored SDK; ordinary MIDI retains its existing path.
- Verify isolated note interception while the emulator UI lock is occupied.
- Generated audio remains experimental and is not enabled in the normal build.

## 0.0.1.5

- Preserve an isolated emulator-only private MIDI audio proof of concept.
- Document repeated note-on/off, voice-release, source isolation and 16 Levels pitch observations.
- Generated audio is not yet connected to the production arpeggiator.

## 0.0.1.4

- Add a host-tested playback state machine for held pads, note pairing, BPM and rate changes.
- Record emulator-only MIDI/audio and voice-release investigation results.
- Audio output is not yet connected; the new state machine emits commands only.

## 0.0.1.3

- Select the arpeggiator step rate with the DATA encoder, with a visible rate label.
- Support 1/4 through 1/64 including quarter, eighth, sixteenth and thirty-second triplets.
- Calculate scheduler step deadlines from BPM; audio playback remains unconnected.

## 0.0.1.2

- Add DATA-controlled HOLD for the visual preview, with green active pads and individual pad toggles.
- Improve the control labels and replace the animated note with two red beamed notes.
- Show a two-second vaporwave ArpeggiGator splash when the app starts.
- Generated note playback remains disabled; HOLD currently controls the visual preview.

## 0.0.1.1

- Redesign the animated gator with red headphones and moving water.
- Synchronize the running emulator's matching autorun card after builds, with a warning when synchronization fails.
- Generated note playback remains disabled.

## 0.0.1.0

- Start the existing gator animation while physical pads are held and return to the idle pose after the last pad is released.
- Track pad state independently of skipped SDK callbacks; keep generated note playback disabled.

## 0.0.1

- Keep the native Sample and 16 Levels views visible by default.
- Add SHIFT+B1 ARP toggle, SHIFT+B2 pattern selection and SHIFT+B3 additional-note label (+0 to +3, default +0).
- Draw the compact gator over the native page only while ARP is enabled.
- Load the firmware-guarded build from SD autorun at every boot while the card is inserted.
- Keep generated note playback disabled; this version is a GUI prototype.
