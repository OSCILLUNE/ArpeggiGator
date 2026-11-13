# ArpeggiGator implementation

## Target

- AKAI MPC Sample, firmware 1.3.0.12.
- Standalone add-in; no SampQOL dependency.
- Rust SDK pinned in Cargo.toml to commit 87d3ec20e6b8d46b0580a4fd7f0f39ca3fcd7caf.

## Implemented

- SHIFT+B1 toggles ARP state in 16 Levels.
- SHIFT+B2 cycles eight patterns.
- Pad press/release callbacks update the held-pad set and preserve velocity.
- The pure Rust engine implements UP, DOWN, UP/DOWN, DOWN/UP, CONVERGE, DIVERGE, RANDOM and PLAYED.
- A platform-independent timing engine implements the six intended rates, gate percentage, paired NoteOn/NoteOff events, rational step deadlines and missed-step skipping. It is host-testable but not connected to MPC callbacks.
- The Sample page is left native; there is no startup overlay.
- A read-only worker reads global or active-sequence BPM from the current temporary project file. It never modifies the project.

The physical pad strike is still forwarded to MPC. The current prototype does not yet emit scheduled retriggers.

## Project tempo

The current implementation reads /data/TempProjects/Temp-Project.xpj every 500 ms. It decompresses the gzip container, locates the JSON document after the ACVS header, and reads either masterTempo or the BPM of the keyed currentSequence.

This parser is host-testable. It still needs on-device confirmation that the temporary project file is refreshed as soon as the user changes BPM, without requiring a save/reload.

## Timing engine

`src/scheduler.rs` is a pure Rust state machine. It accepts a monotonic microsecond timestamp and a validated BPM, then returns at most one NoteOn per poll and its matching NoteOff. Step deadlines are derived from a rational grid (milli-BPM × subdivisions), so integer rounding does not accumulate. A late poll skips expired deadlines instead of emitting a burst. The engine defaults to 1/16 and 50% gate. It has no firmware calls, does not read the project file and is not yet responsible for transport-phase alignment.

## Playback architecture

1. Capture the currently held 16 Levels pads.
2. Use the detected project BPM and a verified transport/clock phase.
3. Use a non-blocking scheduler to decide each step and gate time.
4. Trigger the target pad through the SDK's native note-on path.
5. Send the matching note-off before the next event or on any stop/disable/release path.

The SDK exposes native note-on/note-off functions, but they require live input/property objects on their owning UI thread. The Minimod reference stores the live input pointer to pair a later note-off, but no inspected implementation establishes that the properties pointer remains valid for a delayed note-on. The SDK repaint pump is a possible wake-up path: a worker requests repaint and the native paint hook may run on the app thread. The thread handoff must be verified in the emulator before dispatching notes from `on_paint`. Do not dereference cached properties from a worker or assume the repaint callback thread without that test.

## UI

- SHIFT+B1: ARP on/off.
- SHIFT+B2: next pattern.
- B1/B2 are consumed only while 16 Levels is active and SHIFT is held.
- The current physical pad event is forwarded.
- The native Sample and 16 Levels pages remain visible by default.
- In 16 Levels, holding SHIFT replaces the three softkey captions with ARP, the current pattern, and +0/+1/+2/+3 (default +0). B1 toggles ARP, B2 changes pattern, and B3 cycles the additional-note setting.
- When ARP is enabled, a compact gator overlay appears. Holding source pads animates a note toward its mouth as a visual preview; generated notes are not connected yet.

## Required safety behavior

- Every generated note-on must have a matching note-off.
- Disabling ARP, releasing a source pad, leaving 16 Levels, transport stop and unload must stop generated notes.
- Generated notes must not be re-captured as new physical pad presses.
- Unknown firmware remains fail-closed.
- The project, samples and device firmware remain unchanged.

## Playback architecture investigation

See [PLAYBACK-RESEARCH.md](PLAYBACK-RESEARCH.md) for the pinned-SDK source audit. No verified periodic UI-thread scheduler is currently exposed by the inspected API; do not implement playback by retaining input/property pointers in a worker or audio callback.
