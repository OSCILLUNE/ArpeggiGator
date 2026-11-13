# Changelog

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
