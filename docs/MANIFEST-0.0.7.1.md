# MANIFEST v0.0.7.1 — jeden kod, dwa artefakty

**Jeremiasz.** `date -u`: 2026-11-18 ~21:25Z.
Zasada: **jeden numer wersji, dwa profile dostarczenia**. Różnica między emulatorami a fizyczną MPC leży po stronie artefaktu, nie kodu.

## Artefakty (ten sam commit, ten sam feature `native-audio`)

| Artefakt | Przeznaczenie | SHA-256 |
|---|---|---|
| `build/arpeggigator-v0.0.7.1.zip` | karta SD → fizyczna MPC Sample 1.3.0.12 | `66846aaaf73b06b218ebc63c9ecea8606e97c14e7f9aecd677bc1aea8f4678e9` |
| `build/arpeggigator-v0.0.7.1-lab.so` | QA/DEV przez `python3 lab.py preload …` | `66e4c6cbd5125c15af83da06b29d7f6cc94e157f1cdfaa0fe847f13899ab6df8` |

`cmp build/lab/arpeggigator-lab.so build/arpeggigator.so` → exit 0 (bajtowo identyczne).
Packaged `.so` w zipie ma ten sam hash `66e4c6c…`.

## Wpis w paczce SD

| Wpis | SHA-256 |
|---|---|
| `autorun/arpeggigator.so` | `66e4c6cbd5125c15af83da06b29d7f6cc94e157f1cdfaa0fe847f13899ab6df8` |
| `autorun/autorun.sh` | `a387165d93df55fe134781162db40428865e1bc49ca9a1996cb8f62f4e64d517` |
| `autorun/autorun.json` | `a085472ee5a1f544b928f44be67579d48c3dee169ee4cb9b25c4c09efda2df6b` |
| `autorun/autorun.check` | `9decad364fd69b6fd909048631202994a7889f8c7c1783fbaa155a57a7145dad` |
| `autorun/pads-off.sh` | `a27f155b45a7c7cfeddf37f7e1c37afe0c9244f32b75d71b526e1a41bb3046f9` |
| `HANDOFF-PRZED-WLOZENIEM-0.0.7.1.md` | `33e8b408071fda04669d96a8c5fdc5a89648d460c8324ac6f8138ceb73d996ca` |

## Weryfikacja

- `cargo test`: 84/84 PASS.
- `python3 scripts/verify-v070.py`: 20/20 PASS.
- Testy karty (fizyczna karta SD 512 MB, `/Volumes/arpeggigato`): hashe plików na karcie zgodne z powyższymi.
- A/B fixu pierwszego PAD: baseline `0/20` → after `19/19` ważnych prób.

## Zasady wersjonowania (obowiązują od 7.1)

1. Jeden numer wersji w `Cargo.toml`/`Cargo.lock` dla obu profili.
2. `native-audio` to jedyny prawdziwy feature; `emulator-audio` to alias (`= ["native-audio"]`) dla starych skryptów — **nigdy osobna ścieżka kodu**.
3. `scripts/build.sh` → paczka SD (fizyczna). `scripts/build-lab.sh` → `.so` do `lab.py preload` (QA/DEV). Obie z tego samego commitu.
4. Eksperymenty (private MIDI, uprobe, phase-preserving) żyją w `experiments/` jako osobne flagi, nigdy w stabilnej linii.
5. Fix trafia najpierw do głównej linii; lab używa tego samego binarium. Zakaz utrzymywania równoległej linii 0.0.6.x.
6. Tag `v0.0.7.1` na commicie release; oba artefakty published na GitHub Releases pod tym samym tagiem.

🪖☕
