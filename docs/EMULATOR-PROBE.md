# Emulator-only repaint dispatch probe

> **Do not install this branch on a physical MPC Sample.**  
> This experiment deliberately reuses a captured properties pointer 250 ms
> after its source pad callback. It exists only to validate the hypothesis in
> the virtual MPC Sample lab.

Branch: experiment/repaint-dispatch-probe

## What it measures

1. on_pad records its hashed std::thread::ThreadId, the live input and
   properties addresses, source, pad and velocity.
2. A worker waits 250 ms. It does not call MPC's native note hooks.
3. The worker requests a repaint using the SDK's supported repaint helper.
4. The SDK's on_paint callback records its thread fingerprint and attempts
   native send_note_on using the captured input/properties.
5. After 120 ms, the worker requests another repaint and on_paint calls the
   matching native send_note_off.
6. The script verifies thread identity, non-null context, both native call
   results, worker completion, process identity and the systemd restart counter.

The probe uses the same physical pad as the generated note, so the source
pad's ordinary release also provides a final release path if the experiment
fails to deliver its generated note-off.

## Build and run

Use a local checkout of this branch and the
[virtual MPC Sample](https://github.com/orrpunk-art/virtual-mpc-sample)
repository. Follow that repository's own firmware-input requirements.

```sh
# Build the emulator once, then start it.
cd /path/to/virtual-mpc-sample
python3 lab.py build
python3 lab.py run --headless
```

In a second terminal, from the ArpeggiGator experiment-branch checkout:

```sh
sh scripts/build.sh
python3 /path/to/virtual-mpc-sample/lab.py preload build/arpeggigator.so
python3 scripts/emulator_repaint_probe.py --lab /path/to/virtual-mpc-sample
```

Open the emulator panel at http://127.0.0.1:8765/ if it is not already open.
The script assumes 16 Levels is initially OFF; start from a fresh app session.
It navigates to Sample, enters 16 Levels, enables ArpeggiGator with SHIFT+B1,
holds PAD 1 for two seconds and reads g_probe from the running process.

## PASS criteria

- on_pad and on_paint thread fingerprints are equal.
- Both captured pointers are nonzero.
- Exactly one delayed native note-on attempt returns true.
- Exactly one matching native note-off is dispatched.
- The probe worker finishes without timeout.
- MPC Sample keeps the same PID and NRestarts=0.

A PASS establishes only that this particular delayed call sequence worked in
this emulator session. Repeat it across fresh boots and other pads before
treating the pointer lifetime as established. Then test timing under CPU load,
page changes, pad release races and transport stop. It is not hardware evidence
and does not establish phase-locked transport synchronisation.

A FAIL is diagnostic, not permission to weaken the safety checks. Record the
printed pointers, thread fingerprints, counter deltas, process PID and logs.
