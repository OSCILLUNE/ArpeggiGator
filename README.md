<div align="center">

<img src="assets/readme-user-banner.webp" alt="ArpeggiGator — original gator artwork in purple and gold" width="500">

# ArpeggiGator 🐊

**Five musical engines for the AKAI MPC Sample — all hidden inside 16 Levels.**

`ARP` · `MELODY` · `MUTATOR` · `ACCORDATOR` · `ORBITATOR`

**Latest published release: [v0.0.7.4](https://github.com/OSCILLUNE/ArpeggiGator/releases/tag/v0.0.7.4)** — native audio engine, first-pad fix, STOP repaint fix, and native pad LED restoration.

**Current release: 0.0.7.4** — restores native pad LED colours on screen/mode exits (no blackout in MUTE) on top of the STOP repaint fix. Both artifacts are attached to the release linked below.

❤️ SPONSOR: <a href="https://github.com/sponsors/OSCILLUNE"><img src="assets/github-red.svg" alt="GITHUB" width="62" height="16"></a> / <a href="https://ko-fi.com/arpeggigator"><img src="assets/kofi-red.svg" alt="KOFI" width="39" height="16"></a> ☕

**<span style="color:#2563eb">DOWNLOAD</span>** · [⬇️ SD-card ZIP for MPC](https://github.com/OSCILLUNE/ArpeggiGator/releases/download/v0.0.7.4/arpeggigator-v0.0.7.4.zip) · [⬇️ Emulator `.so` for QA/DEV](https://github.com/OSCILLUNE/ArpeggiGator/releases/download/v0.0.7.4/arpeggigator-v0.0.7.4-lab.so)

❤️ SPONSOR: <a href="https://github.com/sponsors/OSCILLUNE"><img src="assets/github-red.svg" alt="GITHUB" width="62" height="16"></a> / <a href="https://ko-fi.com/arpeggigator"><img src="assets/kofi-red.svg" alt="KOFI" width="39" height="16"></a> ☕

<!-- YT-WIDGET:START -->
**Latest videos — [OSCILLUNE on YouTube](https://www.youtube.com/@oscillune)**

<table><tr>
<td align="center"><a href="https://www.youtube.com/watch?v=mTeVYvCqEWQ"><img src="https://i.ytimg.com/vi/mTeVYvCqEWQ/hqdefault.jpg" alt="This Mod Makes AKAI MPC Sample Play Music for You — ArpeggiGator Explained #mpc #Arpeggiator #akai" width="180"><br><sub>This Mod Makes AKAI MPC Sample Play Music for You — Ar</sub></a></td>
<td align="center"><a href="https://www.youtube.com/watch?v=QfljdW24p6g"><img src="https://i.ytimg.com/vi/QfljdW24p6g/hqdefault.jpg" alt="NEW! ArpeggiGator — MOD for AKAI MPC Sample 🧩" width="180"><br><sub>NEW! ArpeggiGator — MOD for AKAI MPC Sample 🧩</sub></a></td>
<td align="center"><a href="https://www.youtube.com/watch?v=d4QMyxIAiHQ"><img src="https://i.ytimg.com/vi/d4QMyxIAiHQ/hqdefault.jpg" alt="NEW! ArpeggiGator ARP — MOD for AKAI MPC Sample 🔥" width="180"><br><sub>NEW! ArpeggiGator ARP — MOD for AKAI MPC Sample 🔥</sub></a></td>
</tr></table>
<!-- YT-WIDGET:END -->
</div>

[Installation](#installation) · [Interface](#interface-tree) · [Shortcuts](#shortcuts) · [Parameters](#parameters) · [Modes](#modes) · [Procedures](#procedures) · [Download](#download)

---

## ⚠️ BETA SOFTWARE — USE AT YOUR OWN RISK

> **ArpeggiGator is BETA software.** It is unofficial, experimental and provided **“AS IS”**, without warranties of any kind. You install and use it **entirely at your own risk**.
>
> Bugs or incompatibilities may cause crashes, unexpected behaviour, loss of work, damaged or corrupted project files, SD-card data loss, or other software or device problems. **Back up your projects and the entire original SD card (including any existing `autorun/` folder) before installation.**
>
> **To the maximum extent permitted by applicable law, OSCILLUNE and the project's authors and contributors accept no liability for any loss, damage, data loss, device malfunction or other consequences arising from installing or using ArpeggiGator.** Nothing in this notice excludes rights or liabilities that cannot legally be excluded.

**This mod does not replace or flash the stock MPC Sample firmware.** It adds optional functionality within **16 Levels** while preserving the normal MPC workflow. However, that does not eliminate all risks of running experimental software.

---

## What it is

ArpeggiGator is an unofficial, performance-focused mod for **Akai MPC Sample** firmware **1.3.0.12**. It adds five optional musical engines **inside the existing 16 Levels mode**. Play evolving arpeggios, melodies, chords or orbital patterns using your own samples, with hands-on controls for rate, patterns and HOLD.

**Your MPC stays your MPC.** The mod loads from the SD card at startup, copies its library to volatile `/run` and leaves the verified firmware untouched. Outside the mod's controls you can keep using the MPC Sample as usual.

- **Five engines:** classic patterns, melodic phrases, evolving mutations, expressive chords and generative ambient movement.
- **Hands-on controls:** three soft keys, the DATA encoder and your pads.
- **Original visuals:** a pixel gator and a different scene for every mode.

## Interface tree

```
MPC Sample (firmware)
└── SAMPLE screen
    └── 16 LEVELS
        └── [SHIFT held OR engine enabled] → ArpeggiGator overlay
            ├── left: animation (a different gator for each mode)
            ├── top right: MODE + parameters + RATE + HOLD
            └── bottom: B1 / B2 / B3 captions (only while SHIFT is held)
```

## Shortcuts

All shortcuts work **only in 16 Levels** (exceptions: STOP and PLAY).

| Button | Without SHIFT | With SHIFT |
|:--|:--|:--|
| **B1** (SCREEN 1) | — | **Change mode**: Off → ARP → MELODY → MUTATOR → ACCORDATOR → ORBITATOR → Off |
| **B2** (SCREEN 2) | — | **Parameter 1** — depends on the mode |
| **B3** (SCREEN 3) | — | **Parameter 2** — depends on the mode |
| **DATA** (knob) | turn = **RATE**, press = **HOLD on/off** | no action |
| **16 LEVELS** | enter/exit the view | does not exit the view |
| **PAD 1…16** | feeds notes into the engine | firmware shortcut, not a note |
| **STOP** | single press = native behaviour; **double press within ≤650 ms** = full reset | — |
| **PLAY** | the mod only reads the firmware transport | — |

The three **B** buttons are the soft keys immediately above the MPC display.

## Parameters

| Mode | B2 — parameter 1 | B3 — parameter 2 |
|:--|:--|:--|
| **ARP** | Pattern: UP, DOWN, UP/DOWN, DOWN/UP, CONVERGE, DIVERGE, RANDOM, PLAYED | Additional octaves: `+0` → `+1` → `+2` → `+3` |
| **MELODY** | Phrase length: `2BAR` → `4BAR` → `8BAR` → `16BAR` | Preset phrase: `MEL1` … `MEL9` |
| **MUTATOR** | Mutation intensity: SUBTLE → WILD → CHAOS | Freeze: `FRZ OFF` ⇄ `FRZ ON` |
| **ACCORDATOR** | Chord mood: WARM → SAD → SPACE → CINEMA | Style: STRUM → PULSE → WAVE |
| **ORBITATOR** | Atmosphere: AMBIENT → SPACE → DARK → RHYTHM | Dynamics: CALM → FLOW → WILD |

## Modes

| Mode | What it does |
|:--|:--|
| **ARP** | Accumulates held pads and plays them back in the selected pattern. HOLD toggles individual pad bits (XOR). |
| **MELODY** | Plays ready-made melodic phrases from a single root pad; a new pad takes over the voice. |
| **MUTATOR** | Evolves the buffer every phrase pass; Freeze keeps the next loops identical. |
| **ACCORDATOR** | Turns one pad into flowing chord voicings; mood sets the intervals, style sets the articulation. |
| **ORBITATOR** | Generative notes drifting across a scale; dynamics set the wandering radius. |

**Tip:** start with **ORBITATOR → AMBIENT → CALM** for evolving soundscapes, or **ACCORDATOR → WARM → WAVE** for lush harmonic movement.

| Mode | Without SHIFT | With SHIFT |
|:--|:--|:--|
| **ARP** | <img src="docs/screens/01_ARP_plain.png" width="300" alt="ARP without SHIFT"> | <img src="docs/screens/01_ARP_shift.png" width="300" alt="ARP with SHIFT"> |
| **MELODY** | <img src="docs/screens/02_MELODY_plain.png" width="300" alt="MELODY without SHIFT"> | <img src="docs/screens/02_MELODY_shift.png" width="300" alt="MELODY with SHIFT"> |
| **MUTATOR** | <img src="docs/screens/03_MUTATOR_plain.png" width="300" alt="MUTATOR without SHIFT"> | <img src="docs/screens/03_MUTATOR_shift.png" width="300" alt="MUTATOR with SHIFT"> |
| **ACCORDATOR** | <img src="docs/screens/04_ACCORDATOR_plain.png" width="300" alt="ACCORDATOR without SHIFT"> | <img src="docs/screens/04_ACCORDATOR_shift.png" width="300" alt="ACCORDATOR with SHIFT"> |
| **ORBITATOR** | <img src="docs/screens/05_ORBITATOR_plain.png" width="300" alt="ORBITATOR without SHIFT"> | <img src="docs/screens/05_ORBITATOR_shift.png" width="300" alt="ORBITATOR with SHIFT"> |

Screenshots: DEV, build `34a9ec26…`, 18 Nov 2026. The screenshots show the original interface layout; 0.0.7.2 changes only the chord-mode label to `ACCORDATOR` / `MODE: ACCORD`.

## Procedures

1. **Enable the mod:** open **Sample** → **16 LEVELS**, hold **SHIFT**, then use **SHIFT + B1** to pick a mode and **SHIFT + B2/B3** for its parameters.
2. **HOLD and notes:** press **DATA** to toggle HOLD. In melodic/generative modes a tapped pad becomes a latch (green) and a new pad takes over the voice. In ARP, HOLD XORs pad bits.
3. **Freeze (MUTATOR):** hold SHIFT and tap **B3** to toggle `FRZ ON/OFF`; subsequent loops play identically.
4. **Swap samples while playing (test-confirmed):** set MELODY + HOLD with the sequence running, go back to the **Sample** tab — the music does **not** stop — tap pads to change one sample for the whole sequence, then return to 16 Levels.
5. **STOP:** a double press within about 650 ms clears engines, HOLD, the active sample and the LEDs.

## Installation

> **Compatibility:** designed for AKAI MPC Sample firmware **1.3.0.12**. Tested in the virtual emulator and confirmed on physical hardware (30–60 min test session). A downloadable ZIP is available in the **v0.0.7.4** release, but this is still **BETA software** — back up everything before installing.

1. **Back up your entire SD card** to your computer before making any changes.
2. **MOVE / COPY / BACK UP your existing `autorun/` folder.** If the SD card already has an `autorun/` folder, move it to your computer — or copy it there, verify the backup, and then **remove the original `autorun/` folder from the SD card**. The card must have **no existing `autorun/` folder** before installing ArpeggiGator. **NEVER merge the new files into an existing `autorun/` folder.**
3. **Download `arpeggigator-v0.0.7.4.zip`** from the [v0.0.7.4 release](https://github.com/OSCILLUNE/ArpeggiGator/releases/download/v0.0.7.4/arpeggigator-v0.0.7.4.zip).
4. **Extract the ZIP.** You should see a new `autorun/` folder inside.
5. **Copy the NEW `autorun/` folder to the root of the SD card**, preserving all files and subfolders. The card should contain `autorun/autorun.sh` — not a ZIP file placed on the card.
6. **Safely eject the SD card**, power off the MPC Sample, insert the card and power it on.
7. **Open Sample → 16 Levels** and use the controls above.

**Already using another mod?** ArpeggiGator's `autorun/` folder **replaces** the previous one on that card; it is **not** merged with it. Keep the old `autorun/` backup safe so you can restore it later. Running multiple autorun mods together is not supported or verified.

**Uninstall / restore:** power off the MPC, remove the ArpeggiGator card or restore your original backed-up `autorun/` folder, then restart.

## Preview

<div align="center">

**Running in the MPC Sample emulator**

<img src="docs/images/led-1-running.jpg" alt="ArpeggiGator running on the MPC Sample" width="760">

</div>

> ### ⚠️ Compatibility — read before you insert the card
>
> **TESTED:** ArpeggiGator **v0.0.7.4** was developed and tested in the virtual MPC Sample emulator (QA/DEV) and confirmed on physical AKAI MPC Sample hardware, on **firmware 1.3.0.12** (guard `44507c7a7d3029c411b2e3780ea286ca3e08b7e3ffd7b5047d52f73bf5299d1f`). The 0.0.7.x package ships a `native-audio` build targeted at firmware 1.3.0.12 with hook-signature verification. Emulator audio (`MPC_MOD_FEATURES=emulator-audio`) was used only in experimental phases; the release build uses native audio and is firmware-guarded.
>
> **NOT TESTED:** **No other firmware version**, **no other MPC model**, and **no other operating system or environment** has been tested. Compatibility outside the tested emulator + firmware 1.3.0.12 combination is **unknown and unverified**.
>
> **BEFORE YOU INSERT THE CARD:** make a complete backup of your **entire original SD card** and **all your projects** to a computer. Verify the backup, then install. If the card already has an `autorun/` folder, back it up and remove it first — do not merge.
>
> This is **BETA software**, unofficial and provided **"AS IS"**, entirely at your own risk.

## Download

Version **0.0.7.4** — one codebase, two delivery profiles. Both artifacts are published under the [v0.0.7.4 release](https://github.com/OSCILLUNE/ArpeggiGator/releases/tag/v0.0.7.4).

| Artifact | Purpose | SHA-256 |
|:--|:--|:--|
| [`arpeggigator-v0.0.7.4.zip`](https://github.com/OSCILLUNE/ArpeggiGator/releases/download/v0.0.7.4/arpeggigator-v0.0.7.4.zip) | SD card → physical MPC Sample 1.3.0.12. Unzip and copy `autorun/` to the card. | `dea08ff69b0db23a0f79e053d40f8acede78801139e5f102cd41464ab7119c97` |
| [`arpeggigator-v0.0.7.4-lab.so`](https://github.com/OSCILLUNE/ArpeggiGator/releases/download/v0.0.7.4/arpeggigator-v0.0.7.4-lab.so) | Emulator / QA-DEV via `python3 lab.py preload …`. **This is not an SD-card package.** | `60b1cbed5a74def68873fb6ef898946e66619a9ad8198aaf9d9da458375f50ad` |

Verify the package before inserting the card. The expected SHA-256 is the value in the table above:

```
shasum -a 256 arpeggigator-v0.0.7.4.zip
```

- **Verified environment:** firmware-targeted aarch64 MPC Sample emulator and physical MPC Sample hardware.
- **Independent project:** not affiliated with, endorsed by or supported by AKAI Professional.

## Development notes

Interested in the implementation? See the [changelog](CHANGELOG.md) and the [user manual](docs/index.html).

## Contact

Questions, bug reports or feedback: **ArpeggiGator@proton.me**

Built by **[OSCILLUNE](https://github.com/OSCILLUNE)** · [Changelog](CHANGELOG.md) · [License](LICENSE)

License: GNU GPL-3.0 — see [LICENSE](LICENSE).
