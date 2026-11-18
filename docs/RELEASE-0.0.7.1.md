# RELEASE v0.0.7.1 — paczka na kartę SD (z fixem błędu pierwszego PAD)

**Jeremiasz.** `date -u`: 2026-11-18 ~20:30Z.
**Plik do wrzucenia:** `build/arpeggigator-v0.0.7.1.zip`
**SHA-256:** `66846aaaf73b06b218ebc63c9ecea8606e97c14e7f9aecd677bc1aea8f4678e9`
**Sidecar:** `build/arpeggigator-v0.0.7.1.zip.sha256` → `shasum -a 256 -c` = OK.

Różnica względem v0.0.7.0: port fixu z `0.0.6.5` (2 linie w `Playback`) + 2 testy regresji. Wersja `0.0.7-1`. Wszystkie pozostałe mechanizmy (native-audio, `/run`, guard, pads-off, 20 kontroli) bez zmian.

## Hashe wpisów

| Wpis | SHA-256 |
|---|---|
| `autorun/arpeggigator.so` | `66e4c6cbd5125c15af83da06b29d7f6cc94e157f1cdfaa0fe847f13899ab6df8` |
| `autorun/autorun.sh` | `a387165d93df55fe134781162db40428865e1bc49ca9a1996cb8f62f4e64d517` |
| `autorun/autorun.json` | `a085472ee5a1f544b928f44be67579d48c3dee169ee4cb9b25c4c09efda2df6b` |
| `autorun/autorun.check` | `9decad364fd69b6fd909048631202994a7889f8c7c1783fbaa155a57a7145dad` (treść `26a97bfc…`, spójna) |
| `autorun/pads-off.sh` | `a27f155b45a7c7cfeddf37f7e1c37afe0c9244f32b75d71b526e1a41bb3046f9` |
| `HANDOFF-PRZED-WLOZENIEM-0.0.7.1.md` | `33e8b408071fda04669d96a8c5fdc5a89648d460c8324ac6f8138ceb73d996ca` |

## Weryfikacja

- `cargo test`: **84/84 PASS** (82 + 2 regresji; oba nowe testy FAIL na kodzie sprzed fixu).
- `python3 scripts/verify-v070.py`: **20/20 PASS** → `docs/VP-0.0.7.0-VERIFY-OUTPUT.md`.
- `g_probe` + `GATOR_NATIVE_AUDIO` obecne w `.so`; guard `44507c7a…` przed `cp`; zapisy wyłącznie do `/run`.
- QA po testach czysta: `MAPS=0`, `DROPIN=` puste, `NRestarts=0`, `preload -- --off`.

## Dowód fixu (A/B na QA, identyczna metoda)

| Seria | Moduł | Nuty (`steps_delta>0`) |
|---|---|---|
| BASELINE | `34a9ec26…` | **0/20** |
| AFTER | `66e4c6c…` | **19/19 ważnych** (1 próba z `trial_rc=2` odrzucona jako błąd harnessu) |

Szczegóły: `/Users/fafi/Downloads/ArpeggiGator-0.0.6.5/docs/RAPORT-FINAL-0.0.6.5.md`.

## Warunki (bez zmian względem v0.0.7.0)

- **NIE NADAJE SIĘ DO DEPLOYU bez weryfikacji przez Gemini 3.8 Flash** — handoff jest w paczce.
- Brak backupu fizycznej MPC; QA bez fabrycznego `/content` (0 plików).
- Po wyjęciu karty bez restartu mod zostaje w RAM — powrót do stanu fabrycznego następuje po restarcie.

🪖☕
