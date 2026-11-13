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
- A pixel-art gator splash is drawn on the Sample page after mod startup.
- A read-only worker reads global or active-sequence BPM from the current temporary project file. It never modifies the project.

The physical pad strike is still forwarded to MPC. The current prototype does not yet emit scheduled retriggers.

## Project tempo

The current implementation reads /data/TempProjects/Temp-Project.xpj every 500 ms. It decompresses the gzip container, locates the JSON document after the ACVS header, and reads either masterTempo or the BPM of the keyed currentSequence.

This parser is host-testable. It still needs on-device confirmation that the temporary project file is refreshed as soon as the user changes BPM, without requiring a save/reload.

## Playback architecture

1. Capture the currently held 16 Levels pads.
2. Use the detected project BPM and a verified transport/clock phase.
3. Use a non-blocking scheduler to decide each step and gate time.
4. Trigger the target pad through the SDK's native note-on path.
5. Send the matching note-off before the next event or on any stop/disable/release path.

The SDK exposes native note-on/note-off functions, but they require live input/property objects on their owning UI thread. The lifetime of pointers captured during a physical pad callback is not yet proven. Do not store or dereference them from a timer until this is tested.

## UI

- SHIFT+B1: ARP on/off.
- SHIFT+B2: next pattern.
- B1/B2 are consumed only while 16 Levels is active and SHIFT is held.
- The current physical pad event is forwarded.
- The compact overlay shows pattern and detected BPM.

## Required safety behavior

- Every generated note-on must have a matching note-off.
- Disabling ARP, releasing a source pad, leaving 16 Levels, transport stop and unload must stop generated notes.
- Generated notes must not be re-captured as new physical pad presses.
- Unknown firmware remains fail-closed.
- The project, samples and device firmware remain unchanged.