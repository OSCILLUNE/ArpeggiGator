# ArpeggiGator 🐊

**A standalone arpeggiator mod for the AKAI MPC Sample.**

ArpeggiGator is an independent project targeting stock MPC Sample firmware **1.3.0.12**. It is not a SampQOL feature or extension.

> Early development: this repository currently contains project scaffolding and design notes. No installable binary is published yet.

## Planned interaction

In **16 Levels** mode:

- Hold **SHIFT + B1** (top soft key 1) to toggle the arpeggiator.
- Hold **SHIFT + B2** (top soft key 2) to cycle the arpeggiator pattern.
- Hold two or more pads to define the active note set.
- The arpeggiator advances automatically, synchronized to the MPC project tempo. **NOTE REPEAT is not required.**
- Releasing the held pads stops the sequence.

Planned patterns:

- Up
- Down
- Up / Down
- Down / Up
- Converge
- Diverge
- Random
- Played order

Planned parameters: rate, octave span, gate and swing behavior, subject to what the MPC host APIs expose safely.

## Design goals

1. Standalone operation on stock firmware 1.3.0.12.
2. No dependency on SampQOL.
3. Non-destructive: do not alter projects, samples or user data.
4. Fail closed on unknown firmware/API versions.
5. Keep the normal 16 Levels behavior unchanged when the mod is disabled.
6. Publish source and reproducible build instructions before distributing binaries.

## Current status

This is the initial repository bootstrap. The next engineering milestone is validating the MPC Sample SDK event and MIDI APIs on the target firmware, then implementing a clocked event scheduler with reliable note-on/note-off handling.

Until that work is complete, **do not install this project on a device**.

## Repository layout

- `assets/` — project artwork
- `docs/` — design and implementation notes
- `src/` — Rust implementation (to be added after API validation)
- `scripts/` — reproducible build and packaging scripts (to be added)

## License

TBD. A license will be selected before accepting external contributions.
