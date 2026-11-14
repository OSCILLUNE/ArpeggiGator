# Audio integration experiments

Status: research and a host-tested playback state machine; **generated audio is
not yet connected to ArpeggiGator**. Firmware and project data were not changed.
These emulator-only observations are not a hardware or release guarantee.

## 2026-11-13/14: MIDI dispatch and release

The active temporary project contained `RnB-Prc-Bang2 SFM001` on A01. Pressing
physical pad 1 produced a measured audio peak of approximately 0.44–0.46 in
the emulator's audio bridge. Attempts to subscribe a separate ALSA sequencer
client directly to the MPC's ports 128:0–2 were rejected. Sending events by
ALSA direct-address delivery succeeded, and the firmware's engine callback
observed each MIDI Note On. Sending notes 36 and 60 unchanged produced no
measurable output for the selected sample.

The scalar observations for physical pad 1 and direct MIDI note 36 differed:
the engine callback saw physical pad note 0, context 0, source `0x7ffffffa`,
and MIDI note 36, context 36, with a source specific to the destination port.
A temporary emulator-only adapter rewrote a single MIDI Note On to pad 0,
context 0 and the observed physical source while borrowing the live callback's
own event. It produced a measured audio peak around 0.40. No input, properties,
engine or event pointer was retained after the callback.

Ordinary MIDI Note Off and Note On with velocity zero did not reach the same
engine callback in these tests. A second temporary test used a *different
Note On as a marker*, rewrote the borrowed event to a zero-velocity pad event,
and forwarded it with the same physical source and context. That pair produced
audio and delivered two callbacks. Isolated tracefs uprobes at firmware offsets
`0x2118930`, `0x2118810`, and `0x2118460` observed the Note Off handler, voice
match, and downstream voice-release path. These observations establish a
promising single-note emulator proof of concept, not that audio gating,
recording, transport, project changes, or physical hardware are safe.

The marker technique is **not integrated**: direct events sent by two
independent ALSA clients to the same MPC port produced the same app-side source
ID, so source alone does not separate generated messages from a user's MIDI.
The physical source, context, pad and firmware offsets were observed only in
this setup. Other modes, banks and projects still need validation. Never use
reserved note markers to intercept unrelated user input. In addition, ordinary
firmware event hooks use a try-lock and may drop a callback; losing a release is
unacceptable. Pointer lifetime, voice cleanup on failure/unload, sequence
recording side effects and the realtime threading boundary remain open.

The transient diagnostic modules were loaded only in `/run`; the stable
0.0.1.3 module was restored afterward. Tracefs probes were removed.

## Project BPM

On the emulator's current sequence, the UI showed 128 BPM. With TAP TEMPO held,
the first DATA turn opened the tempo window and the next changed it to 129.
While the UI displayed 129, `/data/TempProjects/Temp-Project.xpj` also reported
129 for the current sequence. The inverse turn restored the file to 128 BPM.
This establishes freshness for that one sequence-tempo interaction, not for
other tempo modes, unsaved changes, transport phase or hardware.

## Pure playback state

`src/playback.rs` adds a host-testable state machine for physical versus held
pads, division, BPM changes, paired note events, and cleanup on HOLD/ARP off.
It emits commands only and is not connected to an audio endpoint. Tests cover
single and multiple pads, latched toggles, gate timing, rate/tempo changes,
invalid tempo and cleanup. A production adapter must guarantee that every
returned Note Off is actually delivered even on queue overflow or shutdown.
