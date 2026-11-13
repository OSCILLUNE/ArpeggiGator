# ArpeggiGator 🐊

![ArpeggiGator mascot](assets/arpeggigator-logo.svg)

**Standalone arpeggiator mod for the AKAI MPC Sample.**

**Version 0.0.1 — GUI prototype.**

ArpeggiGator targets stock MPC Sample firmware **1.3.0.12**. It is independent of SampQOL.

## Controls

In 16 Levels mode:

| Shortcut | Action |
|---|---|
| SHIFT + B1 | ARP on/off |
| SHIFT + B2 | Cycle pattern |
| SHIFT + B3 | Cycle additional-note setting (+0, +1, +2, +3; default +0) |

B1, B2 and B3 are the three soft keys above the display. Their custom labels appear only while SHIFT is held in 16 Levels. The ordinary Sample and 16 Levels screens remain visible by default.

## Patterns

UP · DOWN · UP/DOWN · DOWN/UP · CONVERGE · DIVERGE · RANDOM · PLAYED

The pure Rust engine tracks the held-pad set and strike velocities. Unit tests cover all main pattern directions and edge cases.

## Current development status

**GUI prototype — automatic note retriggering is not enabled yet.**

Committed now:
- SDK integration pinned to a specific revision.
- SHIFT+B1/B2/B3 controls scoped to 16 Levels, with temporary custom softkey captions.
- An on-demand pixel gator overlay when ARP is enabled; ordinary Sample and 16 Levels views remain native.
- Eight-pattern engine and host tests.
- Host-testable timing engine for 1/4, 1/8, 1/8T, 1/16, 1/16T and 1/32 rates, with gate control and paired note events.
- Read-only BPM discovery from the active working project file; tempo is not currently shown over the MPC controls.
- Firmware-guarded build/package script and GitHub Actions CI.

Still pending:
- Connecting the chewing animation to actual arpeggiated note events. The current motion is a visual preview while a source pad is held; this build does not generate arpeggio notes yet.
- Applying the +0/+1/+2/+3 setting to generated notes; the setting currently changes its softkey label only.
- Adding the HOLD mode later, after the core arp output works.
- Connecting the timing engine to native note-on/note-off. Native note trigger functions require live input/property objects on their owning UI thread; deferred properties lifetime and the repaint-to-UI-thread handoff still need emulator validation.
- Validation that the temporary project file updates immediately when unsaved sequence or global tempo changes are made.
- Hardware verification of the clock and note lifecycle.

The goal remains an automatic arpeggiator that works without NOTE REPEAT and follows the active project tempo. No generated notes are emitted until pointer lifetime, tempo freshness and note-off cleanup are proven.

This version is a GUI prototype, not an audible arpeggiator release: it does not yet emit generated notes.

## Build

The SDK requires Rust 1.97+, Zig, LLVM binutils, Python 3 and zip. Run:

    sh scripts/build.sh

This produces `build/arpeggigator.zip`, a firmware-guarded SD autorun package. The MPC
loads it on each boot while that card remains inserted. The library itself is copied to
`/run`, so it does not modify the verified firmware.

## Technical notes

- docs/RESEARCH.md — verified API contracts
- docs/IMPLEMENTATION.md — runtime design
- docs/ROADMAP.md — milestones

## License

The project license is still to be selected.
