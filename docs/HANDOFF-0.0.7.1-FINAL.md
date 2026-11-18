# HANDOFF v0.0.7.0 — PRZED WŁOŻENIEM KARTY DO FIZYCZNEJ MPC SAMPLE

**Autor:** Jeremiasz (przejął wątek od Heńka). `date -u` pisania: 2026-11-18 ~17:55Z.
**Adresat:** kolejna AI (Gemini 3.8 Flash), która ma **zweryfikować paczkę zanim fafi włoży kartę do MPC**.
**Status:** paczka zbudowana i poddana 20 kontrolom. **NIE WŁOŻYŁEM KARTY DO ŻADNEJ FIZYCZNEJ MPC. NIE WYKONAŁEM BACKUPU (fafi zrezygnował).**

> **AKTUALIZACJA 2026-11-18 ~20:25Z — PACZKA WYDANA JAKO v0.0.7.1.**
> Ten handoff obowiązuje też dla v0.0.7.1. Poniższe 20 ostrzeżeń i procedury są bez zmian.
> Jedyna różnica kodu względem v0.0.7.0: dwie linie w `src/playback.rs` (fix opisany niżej)
> + dwa testy regresji. Raport: `docs/RAPORT-FINAL-0.0.6.5.md` w worktree
> `/Users/fafi/Downloads/ArpeggiGator-0.0.6.5`.

## AKTUALIZACJA v0.0.7.1 — FIX BŁĘDU „PIERWSZY PAD PO SHIFT+B1"

- **Przyczyna (PROVEN):** `press()` ustawiał `physical |= bit` przed testem `mode == Off`; `clear()` nie zerował `physical`. Wyjście z trybu przez Off zostawiało osierocony bit → pierwszy press Mutator/Acordator z HOLD był odrzucany (`selected=0`, zero `steps`), a animacja szła osobną ścieżką.
- **Fix:** test `mode == Off` przed zatrzaskiem + `self.physical = 0` w `clear()`. Dwa testy regresji (fail przed, pass po). 84/84 testów.
- **A/B QA (identyczna metoda, jeden preload + skoki menu):** baseline `34a9ec26…` → **0/20 prób z nutami**; fix `af821360…` → **19/19 ważnych prób z nutami** (`steps_delta` 9–29). Jeden pomiar odrzucony (`trial_rc=2` — błąd harnessu).
- **Weryfikacja paczki:** `python3 scripts/verify-v070.py` → 20/20 PASS.
- **Uwaga harnessu:** `lab.py preload -- --off` (nie `preload --off`).

---

## TRZY ZDANIA-KANON

1. **TRZEBA DOPILNOWAĆ I PRZEANALIZOWAĆ CAŁY KOD LINIA PO LINII, ŻEBY NIE ROZWALIĆ FIZYCZNEJ MPC SAMPLE.**
2. **CAŁY KOD TRZEBY PRZEPISAĆ NA NOWO — tu używaliśmy audio emulatora (`MPC_MOD_FEATURES=emulator-audio`). Nie zakładać, że build emulatora jest buildem dla fizycznego sprzętu.**
3. **MUSIMY BYĆ PEWNI, ŻE PO WYJĘCIU KARTY SD Z FIZYCZNEJ MPC KONSOLA JEST W STANIE JAK PRZED WŁOŻENIEM KARTY.**

---

## ⚠️ 20 OSTRZEŻEŃ — czytaj i odhaczaj każde z osobna

1. **Nie wkładaj karty do fizycznej MPC bez pełnej, linia-po-linii weryfikacji i zgody fafi.**
2. **Nie zakładaj, że „kompiluje się i testy przechodzą" wystarcza. 82/82 testów hosta to nie test sprzętu.**
3. **Każde twierdzenie poprzyj komendą, pełnym hashem i `date -u`. PROVEN vs INFERRED — bez wyjątków.**
4. **Nie ufaj testom opartym wyłącznie na heartbeat/OCR. Wymagany jest odczyt z systemu przez SSH i screen w momencie naciśnięcia przycisku.**
5. **Nie używaj starego `build/arpeggigator.zip` sprzed rebuildu — zawierał odrzucony build `9beaf588…` (= `-063.so`).**
6. **Nie używaj `build/arpeggigator-qa.zip` — jest niekompletny (brak `pads-off.sh`, brak `ExecStopPost`, `.so` niepasujący do żadnego buildu).**
7. **Nie przywracaj `0.0.6.1`, `0.0.6.2`, `0.0.6.3` jako bazy. Baza to czysty `v0.0.6.0` = `b912eed…`.**
8. **Nie włączaj `stage-arp-only` ani `stage-melody-only` — to są gate'y stagingowe. W buildzie jest tylko `native-audio`.**
9. **Nie wyłączaj feature'u `native-audio` „dla bezpieczeństwa" — dostaniesz tylko nakładkę UI bez nut. To zmiana vs zwalidowane 0.0.6.0.**
10. **Nie zakładaj, że stara nazwa `emulator-audio` oznaczała detekcję VM. To był główny przełącznik silnika dźwięku. Zmiana nazwy to część rewrite'u.**
11. **Sprawdź guard firmware'u `44507c7a…` w `autorun.sh` — musi być przed pierwszym `cp`. Jeśli nie pasuje, autorun musi wyjść bez zmian.**
12. **Sprawdź `g_probe` ORAZ marker `GATOR_NATIVE_AUDIO` w `.so`. Sam `g_probe` nie dowodzi, że arpegiator generuje nuty.**
13. **Sprawdź `autorun.check`: `sha256(autorun.json + autorun.sh + f7bed1c9-7e90-4731-9194-c996666b4331)`. Nie myl hashy pliku z treścią checku.**
14. **Wszystkie zapisy autorunu muszą iść do `/run`. Jeśli w `autorun.sh` pojawi się `/etc`, `/data`, `/content`, `/factory`, `/usr` — STOP.**
15. **Sprawdź, że log diagnostyczny jest w `/run/arpeggigator-6-events.log*`, a nie w `/tmp`. Zapis do `/tmp` był realnym naruszeniem kanonu 3.**
16. **Pamiętaj: SDK `HookEngine::abort()` nie cofa commitowanych hooków. Nie ma API do unhooku w runtime. Powrót do stanu fabrycznego następuje po RESTARCIE (RAM + `/run` są ulotne), nie w chwili wyjęcia karty.**
17. **Scenariusz A (wyjęcie karty bez restartu): mod, drop-iny i zapalone pady ZOSTAJĄ. To nie jest stan fabryczny — nie ukrywaj tego.**
18. **`pads-off.sh` wysyła surowe SysEx na `/dev/snd/midiC<n>D0` (karta `Surface`). Format i nazwa pochodzą z emulatora — na fizycznym kontrolerze LED to NIEZWERYFIKOWANE.**
19. **Część SyEx/ALSA/threadingu opiera się na obserwacjach z emulatora. Nie przedstawiaj ich jako dowodu z fizycznej MPC.**
20. **QA jest czysta pod względem systemu, firmware'u i usług, ale NIE MA fabrycznej biblioteki `/content` (0 plików). Nie nazywaj jej „jak ze sklepu" bez zastrzeżenia. Backup fizycznej MPC nie został wykonany (decyzja fafi).**

---

## CO ZOSTAŁO ZROBIONE (PROVEN)

### Repo i wersja
- HEAD bazowy: `b912eed1028c9b9c54a166b36433cd349e782be0` (czysty `v0.0.6.0`), branch `release/0.0.6.4`.
- Wersja: `0.0.7-0` w `Cargo.toml` i `Cargo.lock`; wpis `## 0.0.7.0` w `CHANGELOG.md`.
- `cargo test`: **82 passed; 0 failed**.
- Build: `sh scripts/build.sh` → `build/arpeggigator.zip`, `build/arpeggigator.so`.

### Rewrite (kanon 2)
- `emulator-audio` → `native-audio` w `Cargo.toml` i we **wszystkich 16** `#[cfg]` w `src/lib.rs`.
- `scripts/build.sh` eksportuje `MPC_MOD_FEATURES=native-audio` (z możliwością `A/B override`).
- Nowy marker `GATOR_NATIVE_AUDIO` w `.so` — dowód, że silnik nut jest w binarium.
- Log diagnostyczny: `/tmp/...` → `/run/arpeggigator-6-events.log` (`src/diag.rs`).
- `src/tempo.rs`: `MONITOR_STOP` + `stop_monitor()`; `Drop` wygasza wątek, ustawia `panic()` i gasi pady.

### Paczka
- 5 plików runtime w `autorun/`: `arpeggigator.so`, `autorun.sh`, `autorun.json`, `autorun.check`, `pads-off.sh`.
- `g_probe` wyeksportowany (wymóg `build.sh:69`).
- Guard `44507c7a7d3029c411b2e3780ea286ca3e08b7e3ffd7b5047d52f73bf5299d1f` w `autorun.sh` przed `cp`.
- Drop-iny wyłącznie w `/run/systemd/system/ac50.service.d/` (`arpeggigator.conf` z `LD_PRELOAD`, `zz-arpeggigator-pads-off.conf` z `ExecStopPost`).
- Handoff dla kolejnej AI wstrzyknięty do zipu jako `HANDOFF-PRZED-WLOZENIEM-0.0.7.0.md` (poza `autorun/`, nie wpływa na `autorun.check`).

### 20 kontroli
- `python3 scripts/verify-v070.py` → raport `docs/VP-0.0.7.0-VERIFY-OUTPUT.md`. Wynik: **20/20 PASS** (jeśli raport mówi inaczej — wierz raportowi, nie temu zdaniu).

---

## CO NIE JEST UDOWODNIONE (INFERRED / BLOKADA)

1. **Fizyczny test guarda i hooków.** Sygnatury firmware 1.3.0.12 sprawdzane w kodzie, ale nie uruchomione na fizycznej MPC.
2. **Kanon 3 w sensie „w chwili wyjęcia karty".** Po restarcie — tak (RAM + `/run` ulotne). Bez restartu — NIE.
3. **`pads-off.sh` na fizycznym kontrolerze LED** — format SysEx i nazwa karty z emulatora.
4. **QA z fabryczną biblioteką** — `/content` puste; nie ma backupu ani zrzutu (fafi zrezygnował).
5. **Nie wykonano backupu fizycznej MPC.** Nie ma rollbacku `dd` ani obrazu `/dev/mmcblk0`.

**Werdykt:** `NIE NADAJE SIĘ DO DEPLOYU NA FIZYCZNĄ MPC` bez fizycznego testu A/B i bez sprawdzenia przez kolejną AI. Paczka jest kompletna i czysta na poziomie kodu, ale **nie jest certyfikowana sprzętowo**.

---

## JAK ZWERYFIKOWAĆ (komendy dla Gemini 3.8 Flash)

```
cd /Users/fafi/Downloads/ArpeggiGator
date -u
git rev-parse HEAD
shasum -a 256 build/arpeggigator.zip build/arpeggigator.so
unzip -l build/arpeggigator.zip
python3 scripts/verify-v070.py
cat docs/VP-0.0.7.0-VERIFY-OUTPUT.md
unzip -p build/arpeggigator.zip autorun/autorun.sh
nm -D build/arpeggigator.so | grep -E 'g_probe|GATOR_NATIVE_AUDIO'
git status --short
```

Oczekiwane: 20/20 PASS, guard przed `cp`, drop-iny tylko w `/run`, oba symbole obecne.

### Screen w momencie naciśnięcia przycisku (QA)
```
python3 scripts/ag6_probe.py qa WERYFIKACJA-070 \
  '{"enter16":true,"mode":3,"hold":["SHIFT","SCREEN 1"]}'
```
Efekt: `06_screen1200_PRESSED.png` i `09_device_PRESSED.png` — ekran zrobiony **zanim** `release`. Bez tego nie uznawaj weryfikacji za kompletną.

### Porty i klucze
- DEV: panel `8765`, SSH `2022`, VNC `5910`, klucz `~/Downloads/virtual-mpc-sample/out/keys/id_ed25519`.
- QA: panel `8766`, SSH `2032`, VNC `5920`, klucz `~/Downloads/virtual-mpc-sample-qa/out/keys/id_ed25519`.
- `lab.py status` w katalogu laboratorium pokazuje stan portów.

---

## RAPORTY AGENTÓW (wszystkie w `docs/`)

- `VP-0.0.7.0-JUREK-INTEGRALNOSC.md` — hashe, tabela źródło→paczka, wykrycie zanieczyszczonej starej paczki.
- `VP-0.0.7.0-FLINT-KOD-LINIA-PO-LINII.md` — mapa 16 plików, cfg gates, brak unhooku.
- `VP-0.0.7.0-KRYSTYNA-BEZPIECZENSTWO.md` — autorun, `ac50`, SysEx/ALSA, `/tmp`, scenariusze A/B.
- `VP-0.0.7.0-CZESIEK-PLAN.md` — 40 kryteriów akceptacji i guidelines screenów.
- `VP-0.0.7.0-ERNIE-CROSSCHECK.md` — niezależna weryfikacja Jurka.
- `VP-0.0.7.0-VERIFY-OUTPUT.md` — wynik 20 kontroli.

---

## ZAKAZY DLA KOLEJNEJ AI

- Nie wdrażaj na fizyczną MPC.
- Nie pushuj, nie resetuj OpenCode.
- Nie dotykaj fizycznej MPC (nawet „tylko odczyt" bez zgody).
- Nie używaj backupu/snapshotu jako dowodu stanu fabrycznego.
- Nie powtarzaj 0.0.6.1/0.0.6.2/0.0.6.3.
- Jeśli czegokolwiek nie umiesz udowodnić — napisz wprost: **„NIE NADAJE SIĘ DO DEPLOYU"**.

Przejmujesz wątek. Nie pocieszaj — diagnozuj. 🪖☕
