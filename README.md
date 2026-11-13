# ArpeggiGator 🐊

![ArpeggiGator mascot](assets/arpeggigator-splash.svg)

**Standalone arpeggiator mod for the AKAI MPC Sample.**

ArpeggiGator targets stock MPC Sample firmware **1.3.0.12**. It is independent of SampQOL.

## Controls

In 16 Levels mode:

| Shortcut | Action |
|---|---|
| SHIFT + B1 | ARP on/off |
| SHIFT + B2 | Cycle pattern |

B1 and B2 are the two left soft keys above the display. The implementation uses the control-surface codes B1=12 and B2=13.

## Patterns

UP · DOWN · UP/DOWN · DOWN/UP · CONVERGE · DIVERGE · RANDOM · PLAYED

The pure Rust engine tracks the held-pad set and strike velocities. Unit tests cover all main pattern directions and edge cases.

## Current development status

**Early prototype — automatic note retriggering is not enabled yet.**

Committed now:
- SDK integration pinned to a specific revision.
- SHIFT+B1 / SHIFT+B2 handling, scoped to 16 Levels.
- Startup toast and compact pixel-art gator splash rendered through the MPC graphics API.
- Eight-pattern engine and host tests.
- Host-testable timing engine for 1/4, 1/8, 1/8T, 1/16, 1/16T and 1/32 rates, with gate control and paired note events.
- Read-only BPM discovery from the active working project file. The overlay shows the detected BPM when the file is available.
- Firmware-guarded build/package script and GitHub Actions CI.

Still pending:
- Connecting the timing engine to native note-on/note-off. Native note trigger functions require live input/property objects on their owning UI thread; deferred properties lifetime and the repaint-to-UI-thread handoff still need emulator validation.
- Validation that the temporary project file updates immediately when unsaved sequence or global tempo changes are made.
- Hardware verification of the clock and note lifecycle.

The goal remains an automatic arpeggiator that works without NOTE REPEAT and follows the active project tempo. No generated notes are emitted until pointer lifetime, tempo freshness and note-off cleanup are proven.

**Do not install this prototype on the MPC yet.** It is not an audible arpeggiator release.

## Build

The SDK requires Rust 1.97+, Zig, LLVM binutils, Python 3 and zip. Run:

    sh scripts/build.sh

## Technical notes

- docs/RESEARCH.md — verified API contracts
- docs/IMPLEMENTATION.md — runtime design
- docs/ROADMAP.md — milestones

## License

The project license is still to be selected.