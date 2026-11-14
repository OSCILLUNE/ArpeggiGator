# Changelog

## 0.0.2.0

- Add an opt-in emulator audio build with native pad playback, pattern and rate selection, and HOLD.
- Record generated notes with SEQ RECORD and play them alongside live arpeggios.
- Allow the emulator audio build to persist on the existing autorun card.
- Audio is experimental and disabled in the standard hardware package. An unavailable sequence tempo prevents playback; transport phase and additional notes remain unfinished.

## 0.0.1.6

- Add an optional real-time MIDI interceptor to the vendored SDK; ordinary MIDI retains its existing path.
- Verify isolated note interception while the emulator UI lock is occupied.
- Generated audio remains experimental and is not enabled in the normal build.

## 0.0.1.5

- Preserve an isolated emulator-only private MIDI audio proof of concept.
- Document repeated note-on/off, voice-release, source isolation and 16 Levels pitch observations.
- Generated audio is not yet connected to the production arpeggiator.

## 0.0.1.4

- Add a host-tested playback state machine for held pads, note pairing, BPM and rate changes.
- Record emulator-only MIDI/audio and voice-release investigation results.
- Audio output is not yet connected; the new state machine emits commands only.

## 0.0.1.3

- Select the arpeggiator step rate with the DATA encoder, with a visible rate label.
- Support 1/4 through 1/64 including quarter, eighth, sixteenth and thirty-second triplets.
- Calculate scheduler step deadlines from BPM; audio playback remains unconnected.

## 0.0.1.2

- Add DATA-controlled HOLD for the visual preview, with green active pads and individual pad toggles.
- Improve the control labels and replace the animated note with two red beamed notes.
- Show a two-second vaporwave ArpeggiGator splash when the app starts.
- Generated note playback remains disabled; HOLD currently controls the visual preview.

## 0.0.1.1

- Redesign the animated gator with red headphones and moving water.
- Synchronize the running emulator's matching autorun card after builds, with a warning when synchronization fails.
- Generated note playback remains disabled.

## 0.0.1.0

- Start the existing gator animation while physical pads are held and return to the idle pose after the last pad is released.
- Track pad state independently of skipped SDK callbacks; keep generated note playback disabled.

## 0.0.1

- Keep the native Sample and 16 Levels views visible by default.
- Add SHIFT+B1 ARP toggle, SHIFT+B2 pattern selection and SHIFT+B3 additional-note label (+0 to +3, default +0).
- Draw the compact gator over the native page only while ARP is enabled.
- Load the firmware-guarded build from SD autorun at every boot while the card is inserted.
- Keep generated note playback disabled; this version is a GUI prototype.
