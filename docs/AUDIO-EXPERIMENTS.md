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

## 2026-11-14: private MIDI port and repeated notes

A temporary ALSA source port named `ArpeggiGator Private Test / Internal
ArpeggiGator` was automatically discovered and subscribed by the running MPC.
Its app-side source was `0x60ec0937`, distinct from the ordinary External MIDI
input `0x90b6e9eb` in this session. The private source identifier remained the
same when another ALSA client occupied the preceding client number. Two
independent senders into the *same* external destination had previously shared
a source ID, so the dedicated port matters. It is not yet proven stable across
all boots, devices, or naming collisions.

With an isolated temporary adapter filtering only the private source, a test
sent four on/off-marker pairs at 250 ms intervals. The meter recorded audio
(peak 0.364) and tracefs recorded four voice matches and four downstream voice
releases. The identical note sent to External MIDI did not enter the private
adapter. These are emulator observations, not a general playback or recording
proof. Source detection and hardcoded pad/context remain unresolved, and the
normal SDK note hook can still drop an event due to its try-lock.

During physical 16 Levels Tune testing, pads 1, 2, 8, 9 and 16 produced pitch
setter values -3, -2, 4, 5 and 12 respectively in the current project. Tracefs
showed the physical pad callback, pitch setter and engine dispatcher in that
order on the Audio Processing thread. This suggests a way to associate pitch
with a pad without retaining pointers, but the values are contextual and must
not be hardcoded. `experiments/private-midi/` preserves the restricted POC source.


### Reproducibility and instrumentation cleanup

The standalone experiment in `experiments/private-midi/` was built from a
scratch copy of the pinned source. After removing all tracing probes and
restarting the application, the rebuilt experiment again registered eight
private marker events and an audio peak of 0.364 with Pad MIDI In Off. An
earlier zero result was invalid: a previous uprobe remained installed at a
firmware hook site, so the SDK firmware guard declined to install the mod.
Future experiments must remove tracefs uprobes before restarting the SDK mod,
and verify the hook actually installed rather than relying on a mapped library.
In a separate isolated run, four of the eight markers were observed entering
the voice-release function. The app's original `Pad MIDI In` setting was
restored to Off after comparison.

An idle trace of `snd_rawmidi_read` and readiness helpers recorded no calls;
the controller uses `snd_rawmidi_tread` on an `MPC Hardware` thread. This is
not yet a verified injection or scheduling interface. A private MIDI input
remains an experiment, and the firmware path must not be shipped until source
identity, note-off delivery, queue overload, cleanup and mode changes are
proven under contention.


## 2026-11-14: real-time note interception under UI contention

The vendored SDK now has an optional `NoteInterceptor`. It is copied once at
installation, receives only the current borrowed note event, and runs before
attempting the module mutex. Returning `None` preserves the previous SDK path.
Its contract prohibits blocking, allocation, pointer retention and module
re-entry; the production ArpeggiGator does not install an interceptor yet.

For an emulator-only test, `experiments/private-midi/build.py --contention`
compiled a disposable mod which blocks its own paint callback for five seconds
on SHIFT+B3. While this held the module mutex, all eight private marker events
(four on and four off) reached the interceptor during the block. The experiment
is not safe to install on hardware and does not establish bounded MIDI
transport latency, audio cleanup on disconnect, or correct pitch mapping.
A subsequent test of the ordinary no-interceptor build passed one and two pad
press/release, five taps, and ARP-off-while-held. The private diagnostic module
was removed from the running emulator afterward.
