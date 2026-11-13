# Technical research and verified contracts

## Target and SDK

- Target firmware: AKAI MPC Sample 1.3.0.12.
- SDK: ioma8/mpc-sample-sdk, pinned to commit 87d3ec20e6b8d46b0580a4fd7f0f39ca3fcd7caf (SDK 0.5.3 at repository bootstrap).
- Reference implementation and build wrapper: ioma8/mpc-minimod.
- Emulator workflow: orrpunk-art/virtual-mpc-sample.

## Hardware button codes

The SDK ButtonEvent.code is the event's raw key code. The control-surface button map identifies:

| Control | Code |
|---|---:|
| SHIFT | 49 (0x31) |
| B1 | 12 (0x0c) |
| B2 | 13 (0x0d) |
| B3 | 14 (0x0e) |
| 16 LEVELS | 40 (0x28) |

The SDK exposes constants for SHIFT, B3 and 16 LEVELS; ArpeggiGator defines B1 and B2 from the control-surface map.

Primary mapping reference: https://github.com/gospodindilly-ops/mpc-sample-lab/blob/main/docs/control-surface-protocol.en.md, section 8.

## Native note trigger API

The SDK exposes sites::send_note_on and sites::send_note_off. These call the app's displaced native note handlers. The API contract requires the input and properties objects to be live and used on their owning UI thread. The SDK also exposes a monotonic clock and a 60 ms repaint pump.

This is not sufficient proof that pointers captured from a physical pad callback remain valid on later paint callbacks. The runtime must validate their lifetime and thread affinity before scheduled retriggers are enabled.

References:
- https://github.com/ioma8/mpc-sample-sdk/blob/main/docs/03-pads.md
- https://github.com/ioma8/mpc-sample-sdk/blob/main/docs/06-midi.md
- https://github.com/ioma8/mpc-sample-sdk/blob/main/docs/16-status-and-time.md
- https://github.com/ioma8/mpc-sample-sdk/blob/main/docs/18-advanced.md

## Tempo source

The SDK's public API currently documents a monotonic clock, but no typed getter for the active sequence BPM or sequencer tick position. ArpeggiGator must not claim project-tempo synchronization until a stable, verified source is identified. The target behavior is sync to the active sequence tempo without requiring NOTE REPEAT.

## Development safety

- Do not modify the firmware image on disk.
- Keep firmware fingerprint checks enabled in the generated autorun package.
- Do not use stored app pointers outside their verified lifetime.
- Send note-off for every generated note on disable, pad release, transport stop and unload.
- Do not publish an installable release until emulator checks and hardware tests pass.
