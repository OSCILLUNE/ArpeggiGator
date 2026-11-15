# Recording investigation — 0.0.2.1

## Emulator comparison

On MPC Sample 1.3.0.12, two separate copies of `/data/ag-record-test`
were prepared with all existing clip events removed. Both retained the same
sample, two-bar sequence, 128 BPM and recorded quantisation settings. The
copies were mounted at `/data/TempProjects` one at a time. The original
directory was unmounted after testing and restored; both copies remain at
`/data/ag-record-0021-arp` and `/data/ag-record-0021-repeat`.

For each run, Sample and 16 Levels were selected, SEQ RECORD and PLAY were
pressed, then pads 1 and 9 were pressed followed by pad 16 about 0.65 s later.
All three were released after a further 1.1 s, then STOP was pressed. The
repeat test held NOTE REPEAT throughout the pad interaction; the ARP test
enabled ARP with SHIFT+B1 at its default rate and +0 setting. The button and
pad actions were sent with approximately 120 ms between individual transitions;
they were not synchronized to an exact transport position. Both recordings
were confirmed by reading the resulting copied project files. The output meter
showed signal in both runs; the audio waveform and transport phase were not
captured. The project wraps at two bars, so file order is not necessarily
chronological order of input.

Both saved clip event lists reported quantisation pulses 240, swing 0.5 and
strength 0.0. NOTE REPEAT saved 11 events at positions 0, 240, ..., 2400,
all length 180 pulses. ARP saved 10 events at positions 0, 240, ..., 2160:
the first seven had length 125, followed by lengths 53, 72 and 95. The
coarse-tuning modifiers differed across generated steps. The default ARP gate
is 50%, while the observed NOTE REPEAT length corresponds to 75% of this grid;
this difference alone is not evidence of a bug. The shorter events could also
be related to releasing pads, loop boundaries or quantisation; this experiment
does not distinguish these causes. The two engines' onset timing, gate and
phase have not been established as identical.

The original ARP scheduler anchors its clock to the first selected pad using
the monotonic clock, rather than a measured transport position. The pattern
engine resets its cursor when the held set changes. These are confirmed code
properties, but their contribution to the reported recording behaviour needs
controlled measurement. The SDK currently exposes no verified transport-phase
API. Do not infer a safe firmware offset or an overdub policy from this test.

## Candidate transport position (read-only research)

Static inspection of the firmware identified registration of the
`PlayheadPosition` property with the description “Returns the playhead
position in pulses”. In this exact firmware, its wrapper at `0x2afb8a0`
loads an object at wrapper + `0x98` and branches to `0x2113b10`. The latter
reads the object at + `0x790`, then a 64-bit field at + `0x130`, and
conditionally adds the value at + `0x668` if the field at + `0x418` is set.
The engine object already identified by the emulator audio adapter had the
expected engine vtable in this run. An external read-only `/proc/PID/mem`
probe of those fields showed a stable 0 at STOP, advancing values during
PLAY (1432, 2598, 3822, 5070, 6307, 7555), and a stable 178 after STOP on
the two-bar loop. This supports the candidate but is not a verified API:
playing state, seeks, tempo changes, loop unwrapping, object lifetime and
thread safety still need validation. No firmware function was called and no
new native hook was installed in this experiment.

A second read-only probe examined the separately registered `IsPlaying`
path. Its implementation checks a virtual method against firmware address
`0x1445640` and, on that branch, reads a byte at transport object + `0x1f0`.
The same transport object at engine + `0x790` matched that method address in
the test: the byte was 0 at STOP, 1 during PLAY, and 0 after STOP. During
PLAY the position wrapped from 6420 to 393 on the two-bar loop while the byte
remained 1. This validates only the observed state transitions in one setup;
the bounds of that loop, seek detection and realtime object lifetime are not
yet established.

`src/transport_playback.rs` contains a host-tested model for an externally
supplied unwrapped pulse clock. It queues pad changes until a grid boundary,
retains the pattern cursor, pairs generated events and handles stop/seek
handoff. An experimental connection exists only behind
`transport-sync-experiment`; it is not part of v0.0.2.1 or the normal
emulator audio build, and is not considered safe for user testing.

The experimental build produced one recording of 10 steps and 10 matching
Note Off events; the saved steps all had lengths of 124–125 pulses. A
longer test revealed that transport may report the exact loop endpoint before
wrapping; the unwrapping condition was corrected and a subsequent read-only
run crossed multiple loops without another reset. One longer pattern run
ended with SIGSEGV in `mod-repaint`; a run with that pump disabled ended in
SIGABRT and `corrupted double-linked list`. A separate run with the pump
disabled completed all eight patterns with 193 Note On and 193 Note Off events.
These observations do not establish a common cause or stable operation.
Test logs were temporarily saved as `/private/tmp/arpeggigator-sync-segv.log`;
do not assume temporary files survive a reboot or a new environment.

## Process failure and test-space limitation

Earlier attempts used copies under `/run`, a 387 MB tmpfs. During the ARP
attempt, the process exited with `SIGABRT`; the journal reported `corrupted
double-linked list` and systemd restarted it. In the subsequent NOTE REPEAT
attempt MPC displayed “Insufficient space on drive” during undo-history
handling. These observations cannot establish that the insufficient-space
condition caused the heap corruption. Another prior
abort reported `double free or corruption (!prev)` without a confirmed cause.
Use disk-backed copies for recordings. The relevant emulator journal was
saved on the host as `/private/tmp/arpeggigator-record-crash-journal.txt`.

On the disk-backed runs neither process restarted during the trial. This is
not a stability guarantee or a diagnosis of the earlier crashes.
