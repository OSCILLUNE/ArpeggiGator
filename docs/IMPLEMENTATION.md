# ArpeggiGator implementation

## Target

- AKAI MPC Sample, firmware 1.3.0.12.
- Standalone add-in; no SampQOL dependency.
- Rust SDK pinned in Cargo.toml to commit 87d3ec20e6b8d46b0580a4fd7f0f39ca3fcd7caf.

## Implemented

- SHIFT+B1 toggles ARP state in 16 Levels.
- SHIFT+B2 cycles eight patterns.
- Pad press/release events update the held-pad set and preserve each pad's velocity.
- The pattern engine is independent of firmware APIs and can be tested on the host.
- A small pixel-art gator splash is rendered on the Sample page after mod startup.

The physical pad strike is still forwarded to MPC. The current prototype does not yet emit scheduled retriggers.

## Playback architecture

The intended runtime path is:

1. Capture the pad set from pad press/release callbacks.
2. Obtain the current sequence tempo or transport tick from a verified source.
3. Use a non-blocking scheduler to decide each step and gate time.
4. Trigger the target pad through the SDK's native note-on path.
5. Send the matching note-off before the next event or on any stop/disable/release path.

The SDK exposes sites::send_note_on and sites::send_note_off. Both require live app input objects on their owning UI thread. A pad callback exposes an input pointer and property pointer, but their lifetime beyond the callback is not yet established. This is a hard safety gate, not a detail to work around with guessed pointers.

The public SDK also exposes CLOCK_MONOTONIC time, but no typed getter for active project BPM or sequencer position. Do not claim tempo sync or ship a timer based on a guessed memory offset.

## UI

- SHIFT+B1: ARP on/off.
- SHIFT+B2: next pattern.
- B1/B2 shortcuts are consumed only while 16 Levels is active and SHIFT is held.
- The normal physical pad event is currently forwarded.
- The small status overlay shows ARP state, selected pattern and held-pad count.

## Required safety behavior

Before the first audible build:
- Every generated note-on must have a matching note-off.
- Disabling ARP, releasing its source pad, leaving 16 Levels, transport stop and unload must stop generated notes.
- Generated notes must not be re-captured as new physical pad presses.
- Unknown firmware must remain fail-closed.
- The original project, samples and device firmware must remain unchanged.