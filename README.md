<div align="center">

![ArpeggiGator — purple and gold gator banner](assets/readme-hero.svg)

**Five musical engines for the AKAI MPC Sample, right inside 16 Levels.**

`ARP` · `MELODY` · `MUTATOR` · `ACORDATOR` · `ORBITATOR`

[![Download here — latest version](assets/readme-download.svg)](https://github.com/OSCILLUNE/ArpeggiGator/releases/latest)

**The download link is ready, but a GitHub Release with an installable ZIP has not been published yet.**

[Installation](#installation) · [How to use](#how-to-use) · [Modes](#the-five-modes) · [Preview](#preview) · [Development notes](#development-notes)

</div>

---

## Make your pads come alive

ArpeggiGator is an unofficial, performance-focused mod for **AKAI MPC Sample**. Turn on **16 Levels**, select a musical engine and play evolving arpeggios, melodies, chords or orbital patterns from your samples. Change patterns, rate and HOLD while playing — without leaving your MPC workflow.

- **Five engines:** classic patterns, melodic phrases, evolving mutations, expressive chords and generative ambient movement.
- **Hands-on controls:** three soft keys, the DATA encoder and your pads.
- **One-card install:** loads from an SD card `autorun/` folder; no firmware flashing.
- **Original visuals:** a pixel gator and mode-specific scenes.

## Installation

> **Compatibility:** designed for AKAI MPC Sample firmware **1.3.0.12**. This is an early-stage mod tested primarily in the emulator. Physical-device compatibility has not yet been fully verified. Do not install until an official downloadable ZIP is published.

1. **Back up your SD card.** If it already contains an `autorun/` folder, copy that entire folder to your computer before changing anything.
2. **Download the latest release ZIP** using the purple button above, once a release is available.
3. **Extract the ZIP.** You should see an `autorun/` folder inside.
4. **Copy `autorun/` to the root of the SD card**, preserving all the files and subfolders. The card should contain `autorun/autorun.sh` — not a ZIP file placed on the card.
5. **Safely eject the SD card**, power off the MPC Sample, insert the card and power it on.
6. **Open Sample → 16 Levels** and use the controls below.

**Already using another mod?** Back up its `autorun/` folder first. Do not merge or overwrite another mod’s autorun files blindly; compatibility is not guaranteed.

**Uninstall / restore:** power off the MPC, remove the ArpeggiGator card or restore your original backed-up `autorun/` folder, then restart.

## How to use

1. Open the **Sample** page and enable **16 Levels**.
2. Hold **SHIFT** and press the soft key **B1** (first key above the screen) to select a mode.
3. Use **SHIFT + B2** and **SHIFT + B3** to change that mode’s options.
4. **Turn DATA** to choose the rhythmic rate; **press DATA** to toggle `HOLD: ON / OFF`.
5. Play your pads. In **ARP**, hold multiple pads; the other generative engines use a **single root pad**.
6. **Double-tap STOP** (within approximately 800 ms) to silence the engine and clear active pad states.

### Quick control reference

| Control | Action |
|:--|:--|
| `SHIFT + B1` | Cycle `OFF → ARP → MELODY → MUTATOR → ACORDATOR → ORBITATOR → OFF` |
| `SHIFT + B2` | Change the first parameter for the current mode |
| `SHIFT + B3` | Change the second parameter (or toggle FREEZE in MUTATOR) |
| `Turn DATA` | Set rate / rhythmic subdivision |
| `Press DATA` | Toggle HOLD (latch playback after releasing a pad) |
| `Double STOP` | Panic stop: silence notes and reset pad states |

The three **B** buttons are the soft keys immediately above the MPC display. ArpeggiGator shortcuts are scoped to **16 Levels**.

## The five modes

![Four illustrated ArpeggiGator modes](assets/readme-modes.svg)

| Mode | What it does | `SHIFT + B2` | `SHIFT + B3` |
|:--|:--|:--|:--|
| **ARP** | Plays notes from held pads in patterns | Pattern: UP, DOWN, UP/DOWN, DOWN/UP, CONVERGE, DIVERGE, RANDOM, PLAYED | Additional octaves: `+0` … `+3` |
| **MELODY** | Plays melodic phrases from one root pad | Phrase length | Melody variant (`MEL1` … `MEL9`) |
| **MUTATOR** | Evolves a melody as it repeats | Intensity: SUBTLE, WILD, CHAOS | Toggle FREEZE |
| **ACORDATOR** | Turns one pad into flowing chord voicings | Mood: WARM, SAD, SPACE, CINEMA | Style: STRUM, PULSE, WAVE |
| **ORBITATOR** | Creates drifting, scale-based generative notes | Atmosphere: AMBIENT, SPACE, DARK, RHYTHM | Dynamics: CALM, FLOW, WILD |

**Tip:** Start with **ORBITATOR → AMBIENT → CALM** for evolving soundscapes, or **ACORDATOR → WARM → WAVE** for lush harmonic movement.

## Preview

<div align="center">

**Running in the MPC Sample emulator**

<img src="docs/images/led-1-running.jpg" alt="Real emulator capture showing ArpeggiGator running on the MPC Sample" width="760">

</div>

**ARP, MUTATOR, ACORDATOR and ORBITATOR:** the artwork above shows illustrative mode cards. Actual per-mode emulator screenshots are not yet stored in this repository and will replace the illustrations when available.

## Download and compatibility status

- **Latest stable / verified version:** **`v0.0.5-1`** (0.0.5.1) — see the [release verification report](docs/RELEASE-REPORT-v0.0.5-1.md).
- **Development version:** `0.0.5-2` — work in progress to fix another bug; **not recommended for download or installation**.
- **Downloadable GitHub Release:** not published yet. The main download button points to the future latest release.
- **Verified environment:** firmware-targeted aarch64 MPC Sample emulator. **Physical hardware testing is still required.**
- **Independent project:** not affiliated with, endorsed by or supported by AKAI Professional.

## Development notes

Interested in the implementation? See [release verification](docs/RELEASE-REPORT-v0.0.5-1.md), [implementation notes](docs/IMPLEMENTATION.md), [development setup](docs/DEVELOPMENT.md) and the [roadmap](docs/ROADMAP.md).

Built by **OSCILLUNE** · [Changelog](CHANGELOG.md) · [License](LICENSE)
