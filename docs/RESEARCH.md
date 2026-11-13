# Technical research and verified contracts

## Target and SDK

- Target firmware: AKAI MPC Sample 1.3.0.12.
- SDK: ioma8/mpc-sample-sdk, pinned to commit 87d3ec20e6b8d46b0580a4fd7f0f39ca3fcd7caf (SDK 0.5.3 at repository bootstrap).
- Reference implementation and build wrapper: ioma8/mpc-minimod.
- Emulator workflow: orrpunk-art/virtual-mpc-sample.

## Hardware button codes

The SDK ButtonEvent.code is the event's raw key code. The control-surface map identifies:

| Control | Code |
|---|---:|
| SHIFT | 49 (0x31) |
| B1 | 12 (0x0c) |
| B2 | 13 (0x0d) |
| B3 | 14 (0x0e) |
| 16 LEVELS | 40 (0x28) |

The SDK exposes constants for SHIFT, B3 and 16 LEVELS; ArpeggiGator defines B1 and B2 from the control-surface map.

Mapping reference: https://github.com/gospodindilly-ops/mpc-sample-lab/blob/main/docs/control-surface-protocol.en.md, section 8.

## Native note trigger API

The SDK exposes sites::send_note_on and sites::send_note_off. The API contract requires live app input and properties objects on their owning UI thread. A physical pad callback exposes those pointers, but it does not establish that they remain valid after the callback returns. ArpeggiGator does not retain or use them yet.

References:
- https://github.com/ioma8/mpc-sample-sdk/blob/main/docs/03-pads.md
- https://github.com/ioma8/mpc-sample-sdk/blob/main/docs/06-midi.md
- https://github.com/ioma8/mpc-sample-sdk/blob/main/docs/16-status-and-time.md
- https://github.com/ioma8/mpc-sample-sdk/blob/main/docs/18-advanced.md

## Project tempo discovery

The MPC working project is exposed at /data/TempProjects/Temp-Project.xpj. The file is gzip-compressed; after decompression it contains an ACVS header followed by JSON.

The JSON document has a data object with masterTempoEnabled, masterTempo, currentSequence and a sequences array. Sequence entries are key/value pairs: the active sequence must be found by matching the entry key to currentSequence, not by using the array position. When masterTempoEnabled is true, masterTempo takes precedence; otherwise the active sequence's value.bpm is used.

ArpeggiGator now has a read-only parser and a 500 ms monitor for this file. It rejects missing, malformed or out-of-range values and shows BPM as unavailable instead of inventing a tempo. The remaining device-side question is freshness: confirm that the file reflects unsaved tempo edits immediately while the application is running.

Format reference: https://github.com/ioma8/mpc-minimod/blob/main/converter/docs/FORMAT.md, sections 1–4. Temporary project path reference: https://github.com/ioma8/mpc-minimod/blob/main/converter/docs/DEVICE-PROBE.md.

## Development safety

- Do not modify the firmware image or project file.
- Keep firmware fingerprint checks enabled in the generated autorun package.
- Do not use stored app pointers outside their verified lifetime.
- Send note-off for every generated note on disable, pad release, transport stop and unload.
- Do not publish an installable release until CI, emulator and hardware checks pass.