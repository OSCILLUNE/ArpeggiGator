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

B1 and B2 are the two left soft keys above the display. The implementation uses the verified control-surface codes B1=12 and B2=13.

## Patterns

UP · DOWN · UP/DOWN · DOWN/UP · CONVERGE · DIVERGE · RANDOM · PLAYED

The pure Rust engine already tracks held pads and velocity and has host-side unit tests for the pattern order.

## Current development status

**Early prototype — no automatic retriggering yet.**

Committed now:
- MPC SDK integration pinned to a specific revision.
- SHIFT+B1 / SHIFT+B2 handling, scoped to 16 Levels.
- Startup toast and a small pixel-art gator splash rendered through the MPC graphics API.
- Eight-pattern engine with tests.
- Firmware-guarded build/package script and CI workflow.

Not yet implemented:
- Automatic note-on/note-off scheduling. The SDK exposes native note trigger functions, but their input/property pointers must remain valid on the UI thread.
- Reading the active sequence BPM / sequencer clock. The public SDK currently exposes a monotonic clock, not a typed project-tempo getter.

The goal remains automatic arpeggiation without holding NOTE REPEAT, synchronized to the active project tempo. The runtime playback path will not be enabled until pointer lifetime, clock source and note-off cleanup are validated.

**Do not install this prototype on the MPC yet.** It is not an audible arpeggiator release.

## Build

The SDK requires Rust 1.97+, Zig, LLVM binutils, Python 3 and zip. The canonical SDK build script is wrapped by:

    sh scripts/build.sh

The generated package is not considered device-ready until the playback milestone and hardware test pass.

## Technical notes

- Research and verified API contracts: docs/RESEARCH.md
- Implementation design: docs/IMPLEMENTATION.md
- Roadmap: docs/ROADMAP.md

## License

The project license is still to be selected.