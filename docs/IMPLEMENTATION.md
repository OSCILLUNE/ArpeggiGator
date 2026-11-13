# ArpeggiGator — implementation notes

## Target

- Device: AKAI MPC Sample
- Target firmware: 1.3.0.12
- Runtime model: standalone add-in loaded by the device's supported add-in mechanism
- Host integration: MPC SDK hooks, validated against the exact SDK revision used for each build

## Input model

Maintain a set of physically held 16 Levels pads. A pad enters the held-note set on key-down and leaves on key-up. Duplicate transitions must be ignored. Pad identity must be mapped through the active 16 Levels layout rather than assuming raw MIDI note numbers are fixed.

## Clock and scheduler

The desired design is an independent scheduler synchronized to the MPC transport/project tempo. It must not require NOTE REPEAT. Prefer a host-provided beat/clock callback if the SDK exposes one. Otherwise validate a safe transport/tempo timing source before implementing a fallback. Do not guess private memory offsets or hook undocumented timing functions.

Use a monotonic clock for scheduling and derive tick duration from the verified tempo and selected rate. Handle transport start/stop, tempo changes, and host suspend/resume without leaving stuck notes.

## MIDI safety

- Emit note-off for every note-on on disable, pad release, transport stop, panic and unload.
- Track active notes by channel and note.
- Preserve velocity from the source pad unless user-configurable velocity behavior is later added.
- Avoid feedback loops by distinguishing generated events from physical pad events.
- Do not forward generated MIDI back into the input-held-pad set.

## UI

In 16 Levels context only:
- SHIFT + B1 toggles ARP.
- SHIFT + B2 cycles pattern.
- Do not consume the button combination outside 16 Levels.
- When ARP is disabled, preserve stock behavior.
- Render compact status in the screen's supported draw layer: ARP state and pattern, without replacing the device's normal UI.

## Initial pattern set

UP, DOWN, UP_DOWN, DOWN_UP, CONVERGE, DIVERGE, RANDOM, ORDER.

Patterns must have deterministic behavior for a fixed held-pad set except RANDOM. RANDOM should avoid immediate repeats when more than one note is held.

## Development stages

1. Confirm SDK API and firmware compatibility.
2. Implement pure Rust pattern engine and unit tests on desktop.
3. Implement host input capture and MIDI output hooks.
4. Implement safe transport-synced scheduler.
5. Add top soft-key bindings and compact overlay.
6. Add cross-compilation, packaging and checksum.
7. Test on hardware with recovery instructions before public release.
