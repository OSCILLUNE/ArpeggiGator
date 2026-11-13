# Safe emulator repaint observer

This branch contains a deliberately limited runtime probe.

## What it tests

- A physical pad press reaches the mod callback.
- A worker can request a repaint through the SDK helper.
- The Sample-page on_paint callback observes that request.
- The callback latency and thread fingerprints can be recorded.
- The MPC process and ac50 service remain alive during the observation.

The observer does not call sites::send_note_on or sites::send_note_off.
It does not copy, store, dereference or reuse PadEvent.input or
PadEvent.properties. It therefore cannot produce an arpeggiated note.

Thread fingerprints are diagnostic values, not a proof of native object
ownership. Even when the pad and paint fingerprints match, this test does not
prove that a cached pointer remains valid after its callback.

## Run on the virtual MPC

With the virtual MPC repository available at
~/Downloads/virtual-mpc-sample, run:

    sh scripts/emulator_repaint_observer.sh

The script builds the mod, installs the package through the emulator's SD-card
autorun workflow, presses one pad for two seconds and reports the observer
counters.

This is an emulator-only diagnostic. Do not install this branch on physical
hardware. A passing result is evidence only for repaint callback delivery and
process stability; it is not evidence that native note playback is safe.
