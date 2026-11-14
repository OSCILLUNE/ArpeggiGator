# Private MIDI audio proof of concept — emulator only

This intentionally limited experiment verifies four repetitions of A01 in one
known virtual MPC configuration. It is **not** the arpeggiator, a release build,
or safe for installation on physical hardware. It hardcodes firmware-specific
source `0x7ffffffa`, pad/context 0, private-input source `0x60ec0937`, and MIDI
markers 36/35. It uses an experimental real-time interceptor and remains unfit for production;
marker delivery and cleanup are not yet guaranteed. It does not validate project
changes, transport, recording, or generic pitch mapping. Before restarting the
mod, remove any tracefs probes on firmware hook sites; otherwise the SDK guard
can refuse installation even though the library remains mapped.

`python3 experiments/private-midi/build.py` copies source and the pinned SDK to
a temporary directory, applies the experiment only to that copy, and writes
`build/private-midi-poc/arpeggigator.so` plus `seq_probe`. It does not modify
or install the production build or autorun card. The encoder waits for a reader
on its dedicated ALSA source port; it sends four on/off marker pairs at 120 BPM,
then explicit MIDI note-offs for both markers. It assumes a loaded, audible
A01 sample. Do not run during recording, on a project with unsaved work, or
while another user uses the emulator. Passing `--contention` additionally builds
a test-only variant that blocks the UI for five seconds on SHIFT+B3; never use
that variant outside this isolated test.

The code is preserved to reproduce the measurement, not to claim that a safe
native output adapter exists. See `docs/AUDIO-EXPERIMENTS.md` for results.
