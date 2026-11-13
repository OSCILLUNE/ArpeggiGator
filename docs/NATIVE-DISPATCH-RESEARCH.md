# Native note dispatch audit

## Scope and firmware identity

This is a static audit of the AM MOD reference and the exact MPC Sample
firmware 1.3.0.12 binary. No note was dispatched, no project was changed, and
no audio or physical-device behavior is claimed.

The extracted `/usr/bin/MPC Sample` is an AArch64 ELF64 shared object,
119,243,440 bytes. Its first 4 KiB SHA-256 is
`44507c7a7d3029c411b2e3780ea286ca3e08b7e3ffd7b5047d52f73bf5299d1f`, matching
the SDK build guard. The instruction words below come from that binary.

## Candidate event ABI

AM MOD defines a 56-byte `step_event` with a 16-byte header and 40-byte inline
payload. Its code constructs the event, sets its pitch, and passes it through
the engine's event dispatcher. The constructor uses an AArch64 hidden return
buffer in `x8`; the dispatcher receives the engine in `x0`, a selector in
`w1`, the event pointer in `x2`, and source in `w3`.

| Firmware offset | Reference use | First four instruction words |
|---:|---|---|
| `0x1e27360` | Pinned SDK `send_note_on` original | `d503233f d10403ff b00299e4 f941cc84` |
| `0x1e270d0` | Pinned SDK `send_note_off` original | `d503233f d10143ff b00299e3 f941cc63` |
| `0x27e54d0` | Construct `step_event` | `d503245f a9017d1f 53140c84 d28f0f07` |
| `0x27e4400` | Set event pitch | `d503233f a9bf7bfd 2a0103e4 2a0203e5` |
| `0x211b090` | Dispatch event into engine | `d503233f d10303ff a9067bfd 910183fd` |
| `0x2118930` | Engine note-off path | `d503233f a9ba7bfd 910003fd a90363f7` |
| `0x20e4da0` | Queue a recorder note-off | `d503233f d10503ff 90028404 f941cc84` |
| `0x213b130` | Finalize a recorder event | `d503233f d10203ff b0028148 f941cd08` |

Static disassembly shows that the event dispatcher checks the tag at `+0x08`.
For tag 3, a zero float at `+0x10` branches to a call of `0x2118930`; the
nonzero path goes through a different engine operation. The pinned SDK's MIDI
parser independently corroborates tag 3 with zero velocity as a Note Off
representation (see below). A deeper pass through the handler identifies a
per-voice source/note candidate match, while the downstream voice and recorder
effects still need verification before a call is safe to add.

The branch is visible at `0x211b388` (`ldr s0, [x20, #0x10]`; `fcmp s0, #0.0`;
`b.eq 0x211b430`). The zero path passes the engine, selector, byte `event+0x30`,
source, and header bits 12–15 to `0x2118930`; it then makes a follow-up call to
`0x211aeb0` with the event pointer.

The `0x2118930` handler walks the engine's voice collection at `engine+0x7b0`.
For each voice it selects a `0x218`-byte-stride table using the header bits and
calls `0x282e8c0`, which compares the incoming source with the slot selected by
the event's `+0x30` byte for ordinary note values. A match is passed to
`0x2118810`. That helper applies more state and mode checks; a conditional
branch can queue a recorder event through `0x20e4da0`, and an eligible path
continues through `0x2118460`, which updates per-voice state and submits a
24-byte event to callbacks through `0x212cba0`. This establishes candidate
selection and the presence of conditional recorder/event paths in this binary.
It does not establish that a call from this module would release every intended
voice, nor what downstream callbacks do to audio or sequence recording.

The pinned SDK's [`midi.rs`](https://github.com/ioma8/mpc-sample-sdk/blob/87d3ec20e6b8d46b0580a4fd7f0f39ca3fcd7caf/src/midi.rs)
independently documents the same event layout: type at `+0x08`, velocity at
`+0x10`, and note at `+0x30`. It defines type 3 as a pad event and decodes a
type-3 event with zero velocity as Note Off; type 2 is also a Note Off event.
That corroborates the zero-float representation in the engine dispatcher. It
does not prove that a particular source/note pair releases the intended voice
or that recorder side effects are correct.

The SDK's high-level input hooks at `0x1e27360` / `0x1e270d0` are a different
layer from the AM MOD direct engine dispatcher at `0x211b090`. The pinned
[SDK `sites.rs`](https://github.com/ioma8/mpc-sample-sdk/blob/87d3ec20e6b8d46b0580a4fd7f0f39ca3fcd7caf/src/sites.rs)
declares that Note On entry with `input`, `source`, `pad`, `properties`, and
velocity; the exact firmware code reads `properties + 0x158` during the call
and also reads state from the input object. Do not substitute one API for the
other or treat the SDK's copied raw addresses as lasting capabilities. Its
wrapper contract allows them only while both objects are live and the caller
is on the owning UI thread.

Every offset would need its own runtime guard in ArpeggiGator. A prologue
match alone would not establish object lifetime, valid arguments, lock order,
re-entrancy, or teardown safety.

## Problems in the reference arpeggiator path

The reference [poly_record.c](https://github.com/AlekseyMalykh/am-mod/blob/main/sd-mod-session/src/poly_record.c)
constructs an event with a nonzero float for both `on` states, but calls the
engine dispatcher only when `on` is true. Its `on == false` branch calls
`chord_record_off`, which only queues a `SequenceRecorder` event when recording
is active. It does not call the engine dispatcher or the engine note-off
function. The reference's `arp_silence()` therefore has no explicit audio
Note Off in this path. Whether the engine applies an automatic duration is
unverified, so this cannot prove that voices release at the requested gate.

The arpeggiator stores `engine` and `track` pointers in global state. Its source
comment says those pointers are cleared when Chords closes, but
[`chords_close()`](https://github.com/AlekseyMalykh/am-mod/blob/main/sd-mod-session/src/chords.c)
only disables the Chords state. `poly_record_arp_emit()` allows an off event
after Chords becomes inactive, and the timer stops only after its next tick.
This leaves a possible stale-pointer path during screen, program, or project
changes. The pointer lifetime has not been proved.

The timer callback takes `voice_guard_lock()` before entering the engine. In
the reference, that lock serializes timer operations with audio rendering, but
it is a blocking mutex on the message thread. The release-all path waits 250 ms
for the same mutex and then calls the original release function without the
lock if the wait expires. That fallback does not guarantee race-free voice
release. Lock ordering and teardown remain unverified. See
[`voice_guard.c`](https://github.com/AlekseyMalykh/am-mod/blob/main/sd-mod-session/src/voice_guard.c).

The reference files inspected on 2026-11-13 identify these Git blob snapshots:

- `poly_record.c`: `526c3fa44bdf6df5dac74a561e3ed5072b44fc01`
- `chords_ui.c`: `fab7ece37f9a51e13fd949bc06d899f4604ff685`
- `chords.c`: `4776af238360ad90af04cc790b3e8809acc5bd96`
- `voice_guard.c`: `2c6c802c7b71a1291d8a08a288dcdde9a7a73d86`

## SDK route and current decision

The pinned SDK's `send_note_on/off` wrappers call the app's original input
handlers. Their safety contracts require a live input object on its owning UI
thread; note-on also needs live properties. `PadEvent` exposes these as raw
addresses, but does not extend their lifetime. The Timer observer found the
JUCE Timer and `on_paint` on the same thread in one emulator run. That
observation does not prove that a cached `input`, `properties`, `engine`, or
`track` pointer remains valid until a later timer callback.

The 4 ms JUCE Timer also measured about 10.52 Hz on average in the virtual MPC,
with 95.016 ms mean and 574.658 ms maximum intervals. It is not a usable
arpeggiator clock under the measured conditions.

A separate safe repaint-cadence probe recorded Linux thread IDs without
retaining any `PadEvent` pointers or calling native note hooks. In that run,
`on_pad` ran on TID 1188 (`Audio Processin`), `on_paint` on TID 1150 (`MPC
Sample Main`), and the repaint worker on TID 1288. The paint TID matched the
process leader. This virtual run places the pad callback on a different thread
from the SDK's UI paint callback. The repaint request also did not invoke paint
inline on its worker. It therefore provides no basis for carrying `input` or
`properties` from `PadEvent` to a later UI callback; their lifetime and
ownership remain unverified.

The pinned SDK provides no general UI callback queue. `Task<T>::take()` polls a
worker result, while `repaint_painted()` requests drawing. The measured paint
callback can be a UI-thread wakeup in this emulator, but it is not a future
event API or a transport clock.

No native Note On/Off call is added. Before a playback experiment, resolve:

1. The complete `step_event` pitch, source, and duration contract.
2. Confirm the per-voice source/note table interpretation against a known Note
   On, then verify voice release and recorder/listener effects for both the
   `0x211aeb0` and `0x212cba0` callback paths.
3. Engine and track lifetime across page, instrument, project, and module
   changes.
4. Audio-render synchronization and lock ordering for the exact call path.
5. A transport-aligned clock with timing adequate for the selected rate.
6. Guaranteed voice release on pad release, disable, transport stop, context
   change, and unload.

The reference's native C harness redirects `0x2118930` to a fake release
function and checks counts and event data; it does not exercise the firmware
audio engine. Its QEMU live regression also explicitly reports
`physical_device_tested=False` and `audio_asserted=False` ([test summary](https://github.com/AlekseyMalykh/am-mod/blob/main/sd-mod-session/lab/check_chords_poly_live.py#L79-L82),
[stubbed native harness](https://github.com/AlekseyMalykh/am-mod/blob/main/sd-mod-session/check_chords_native.c#L27-L30)).
The AM MOD implementation remains a research reference; the Chords path is
marked trial and does not prove physical-device or audio behavior.
