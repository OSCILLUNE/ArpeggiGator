# Safe emulator repaint cadence observer

This branch contains a deliberately limited runtime probe.

## What it tests

- A panel-driven pad press reaches the mod callback.
- A worker can request 128 repaints through the SDK helper at 4 ms intervals.
- The Sample-page on_paint callback runs repeatedly during the request burst.
- First callback latency, inter-paint intervals and thread fingerprints can be recorded.
- The MPC process and ac50 service remain alive during the observation.

The callback counter records every on_paint call during the active measurement
window. It can exceed the 128 repaint requests and does not establish a one to
one mapping between a request and a callback. First callback latency is shown
in whole milliseconds; interval statistics use microseconds.

The observer does not call sites::send_note_on or sites::send_note_off.
It does not copy, store, dereference or reuse PadEvent.input or
PadEvent.properties. It therefore cannot produce an arpeggiated note.

Thread fingerprints are diagnostic values, not a proof of native object
ownership. A matching value would not prove that a cached pointer remains valid
after its callback.

## Run on the virtual MPC

With the virtual MPC repository available at
~/Downloads/virtual-mpc-sample, run:

    sh scripts/emulator_repaint_observer.sh

The script builds the mod, installs the package through the emulator's SD-card
autorun workflow, presses one pad for two seconds and reports repaint counts,
callback latency and inter-paint interval statistics. The request burst is
fixed at 128 calls, followed by a 750 ms drain period. The Python runner can
read either the autorun path (`/run/arpeggigator.so`) or the emulator preload
path (`/run/mpclab-preload/arpeggigator.so`).

## Recorded run

On 2026-11-13, one virtual MPC run received 128 repaint requests and recorded
239 paint callbacks with 238 intervals: 327 us minimum, 2,216.8 us mean and
9,896 us maximum. The pad and paint thread fingerprints differed. The MPC PID
stayed 417, the service remained active, and the systemd restart count stayed
at zero during the measurement. This run does not attribute each callback to
one request; it also provides no evidence of native note playback.

This is an emulator-only diagnostic. Do not install this branch on physical
hardware. A passing result is evidence only for repaint callback cadence in
this emulator session and process stability; it is not evidence that native
note playback is safe.
