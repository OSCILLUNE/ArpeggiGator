# WERYFIKACJA WIZUALNA v0.0.7.2 — napisy ACCORDATOR / ACCORD / SUBTLE FRZ:OFF

**Autor:** CZESIEK-3 (zespół Jeremiasza). `date -u` pisania: **2026-11-18 22:04Z**.
**Repo:** `/Users/fafi/Downloads/ArpeggiGator`, branch `release/0.0.7.2`,
HEAD `542b33fa88f83a74fd4a79478c3f118b106657f4`.
**Zakres mojej pracy:** WYŁĄCZNIE `docs/` — ten raport + `docs/tools/v072_caption_hash.sh`.
Nie modyfikowałem `src/`, `build/`, `README`, nie uruchomiłem żadnego probe'a,
nie łączyłem się z maszynami DEV/QA ani z fizyczną MPC, nie wykonałem deployu.

---

## TRZY ZDANIA-KANON (obowiązują też tutaj)

1. **TRZEBA DOPILNOWAĆ I PRZEANALIZOWAĆ CAŁY KOD LINIA PO LINII, ŻEBY NIE ROZWALIĆ FIZYCZNEJ MPC SAMPLE.**
2. **CAŁY KOD TRZEBY PRZEPISAĆ NA NOWO — tu używaliśmy audio emulatora (`MPC_MOD_FEATURES=emulator-audio`).**
3. **MUSIMY BYĆ PEWNI, ŻE PO WYJĘCIU KARTY SD Z FIZYCZNEJ MPC KONSOLA JEST W STANIE JAK PRZED WŁOŻENIEM KARTY.**

> 7.2 to zmiany **wyłącznie kosmetyczne** — nie musimy odtwarzać buga z 7.1
> (świeży preload maskował błąd pierwszego padu). Musimy **udowodnić, że nowe
> napisy są widoczne i że nic się nie cofnęło** (audio, guard, NRestarts, ABRT, `/run`).

---

## LEGENDA STATUSU — czytaj przed użyciem

| Status | Znaczenie |
|---|---|
| **PROVEN** | Sprawdzone przeze mnie lokalnie w trakcie pisania tego raportu: odczyt kodu, odczyt istniejących zrzutów 7.1, hashe, test skryptu. |
| **PROCEDURA → PROVEN po wykonaniu** | Komenda przygotowana i skontrolowana składniowo, **nieuruchomiona** zgodnie z zakazem. Staje się PROVEN dopiero po wpisaniu pełnego wyniku do tabeli w §4. |
| **INFERRED** | Wniosek z kodu/historii, nie zweryfikowany na nowej paczce. |

**Wszystkie komendy `ag6_probe.py`, `verify-v070.py` i `nm` na nowym buildzie są
`PROCEDURA → PROVEN po wykonaniu`. Nie uruchomiłem ich.**

---

## 0. CO JEST JUŻ PROVEN (zanim ktokolwiek odpali probe)

### 0.1 Stan kodu i narzędzi

| Fakt | Dowód (pełna komenda) | Wynik |
|---|---|---|
| HEAD i branch | `git rev-parse HEAD && git branch --show-current` | `542b33fa…`, `release/0.0.7.2` |
| Docelowe stałe napisów (working tree Flint-3, odczyt 22:0xZ) | `grep -nE 'MODE_ACCORD|ACCORDATOR_TITLE|mutator_param_line|SUBTLE FRZ' src/splash.rs` | `MODE_ACCORD = "MODE: ACCORD"`, `ACCORDATOR_TITLE = "ACCORDATOR"`, `mutator_param_line("SUBTLE", false) == "SUBTLE FRZ:OFF"` |
| 3 nowe testy regresji w kodzie | `grep -n 'fn mutator_freeze\|fn accord_captions' src/splash.rs` | `mutator_freeze_off_is_not_duplicated`, `mutator_freeze_on_keeps_the_single_suffix`, `accord_captions_have_the_correct_spelling` |
| **Brak OCR w środowisku** | `for c in tesseract magick convert identify python3; do command -v $c; done; python3 -c 'import PIL'` | `tesseract`, `magick`, `convert`, `identify` → **BRAK**; `PIL` → **ImportError**. Dostępne: `sips`, `shasum`, `python3`, `cmp`. |
| Skrypt porównawczy działa | `bash docs/tools/v072_caption_hash.sh diff PREFIX PREFIX` / `diff A B` | self-diff = `0/…`; dwa różne screeny = `B1_TITLE 3654/5016`, `MODE_TITLE 1984/6480`, `PARAM_LINE 204/6480` |
| Harness jest read-only i robi screen przed release | odczyt `scripts/ag6_probe.py` l. 107–120, 198–222 | `device_shot()` → `/screen.png` 320×240; `06_screen1200_PRESSED.png` (l. 206) i `09_device_PRESSED.png` (l. 209) **przed** release (l. 216) |

> Uwaga INFERRED: powyższe stałe są w working tree w chwili pisania (Flint-3 edytuje kod
> równolegle). Po jego commicie i buildzie trzeba powtórzyć odczyt `grep` i `cargo test`.

### 0.2 Baseline „PRZED" — istniejące zrzuty QA z 7.1 (hashe PROVEN)

Źródła: `docs/logs/ag6/night-20261118/qa/<label>/09_device_PRESSED.png` (LCD 320×240,
zrzut w momencie naciśnięcia, build 7.1 ze **starymi** napisami).

| Rola baseline | Katalog 7.1 | Plan (z `00_plan.json`) | SHA-256 całego `09_device_PRESSED.png` |
|---|---|---|---|
| MUTATOR, mode=3, PAD 4 | `AB_A01` | `{"enter16":true,"mode":3,"setup_actions":[{"name":"DATA","action":"press","value":127}],"hold":[{"name":"PAD 4","value":100}],"animation":true}` | `5bb9f9a8a2ccbdba04463e221eb7068176f1c88840f97f279420e10d4c809554` |
| ACORDATOR, mode=4, PAD 4 | `AB_A02_ACO` | j.w. z `"mode":4` | `3fc5ad07ef4fcc1e4213ac302196fc10241bd4261d9593ea597b8ffe97c6950d` |
| etykieta B1 przy SHIFT (stary `ACORDATOR`) | `QA_B1_mut_shift_b1` | `{"enter16":true,"mode":3,"hold":["SHIFT","SCREEN 1"]}` | `7f2f0a91339025085924cab83d6ed7b7bd93afec1956282277e2a255d06a5930` |

Hashe **regionów** wycięte z tych plików przez `docs/tools/v072_caption_hash.sh` (PROVEN,
wygenerowane 2026-11-18 ~22:03Z):

| Region | `AB_A01` (mode 3) | `AB_A02_ACO` (mode 4) | `QA_B1_mut_shift_b1` (SHIFT) | Co widać po wycięciu (PROVEN, podgląd obrazu) |
|---|---|---|---|---|
| `B1_TITLE` (19×88 @5,13) | `1310e50bb78b1230…` | `1310e50bb78b1230…` | **`cc8319fa1ff6cb2afc95ddabac04f06bf4842153a9690d3c3da8350373992eca`** | bez SHIFT: native `Trim`; **z SHIFT: `ACORDATOR`** |
| `MODE_TITLE` (18×119 @53,184) | `61362e7efabc63d8db4b6a3c472817c41f95838d3d673002bec5e4cd4abead78` | **`ea060d6d933ded9833629b43f101c586b877ec3ffe75c7c3c24aec0616ee8cb1`** | `ea060d6d…` | `MODE : ACORD` (stary, mode 4) |
| `PARAM_LINE` (18×119 @71,184) | **`da3a940b2e7dd61d3963b8409f881a14c935ea926d554ee254e2200b3a533d1c`** | `20c4c36549f43e8dc354b1f81105fad9fb20ccf7f1fe90c5ed8bf0d967a6f90a` | `20c4c365…` | **`SUBTLE  FRZ OFF  FRZ:OFF`** — dowód podwójnego FRZ |

> **Ważna kalibracja (PROVEN z podglądu cropów):** etykieta `ACORDATOR`/`MUTATOR` w lewej
> górnej celi B1 jest rysowana przez `draw_shift_labels()` (`src/splash.rs:1343`) **tylko
> przy trzymanym SHIFT**. Bez SHIFT ten region pokazuje natywny napis `Trim`. Dlatego
> weryfikacja `ACCORDATOR` wymaga wariantu `V072_ACC_SHIFT`. Bez SHIFT nie uznawaj dowodu.

---

## 1. DZIESIĘĆ KRYTERIÓW MUST-HAVE / MUST-NOT (komenda + oczekiwany wynik)

Status każdej pozycji: **PROCEDURA → PROVEN po wykonaniu** (wpisz wynik do §4).

| ID | Typ | Kryterium | Komenda (skrót — szczegóły niżej) | Oczekiwany wynik |
|---:|---|---|---|---|
| **C01** | MUST-HAVE | `ACCORDATOR` widoczny w celi B1 przy SHIFT, mode 4 | `ag6_probe.py qa V072_ACC_SHIFT …` + `caption_hash.sh diff OLD_ACC_SHIFT NEW_ACC_SHIFT`, podgląd `B1_TITLE.png` | `B1_TITLE differing_bytes > 0` vs `cc8319fa…`; na cropie widać `ACCORDATOR` |
| **C02** | MUST-HAVE | `MODE: ACCORD` widoczny w prawym panelu, mode 4 | `ag6_probe.py qa V072_ACC …` + `diff OLD_ACC NEW_ACC`, podgląd `MODE_TITLE.png` | `MODE_TITLE differing_bytes > 0` vs `ea060d6d…`; widać `MODE : ACCORD` |
| **C03** | MUST-HAVE | `SUBTLE FRZ:OFF` widoczny jako **jedno** FRZ, mode 3 | `ag6_probe.py qa V072_MUT …` + `diff OLD_MUT NEW_MUT`, podgląd `PARAM_LINE.png` | `PARAM_LINE differing_bytes > 0` vs `da3a940b…`; widać `SUBTLE FRZ:OFF`, brak `FRZ OFF` przed `FRZ:` |
| **C04** | MUST-NOT | Brak starych `ACORDATOR` / `MODE: ACORD` | `strings -a build/arpeggigator.so \| grep -nE 'ACORDATOR\|MODE: ACORD'`; `grep -nE … src/*.rs` | **brak trafień**, `grep` zwraca `rc=1` |
| **C05** | MUST-NOT | Brak podwójnego FRZ | `strings -a build/arpeggigator.so \| grep -c 'FRZ OFF'` → `0`; `… \| grep -c 'SUBTLE FRZ:OFF'` → `1`; + crop z C03 | zero `FRZ OFF`, dokładnie jeden `FRZ:OFF` w linii |
| **C06** | MUST-HAVE | `NRestarts=0` po każdym probe | `grep -h '^NRestarts=' …/V072_*/12_service_RELEASED.txt` | 4 wiersze, każdy `NRestarts=0` |
| **C07** | MUST-HAVE | `ABRT=0` po każdym probe | `grep -h '^ABRT=' …/V072_*/12_service_RELEASED.txt` | 4 wiersze, każdy `ABRT=0` |
| **C08** | MUST-HAVE / MUST-NOT | Audio bez zmian + `g_probe` i `GATOR_NATIVE_AUDIO` | `nm -D build/arpeggigator.so \| grep -E 'g_probe\|GATOR_NATIVE_AUDIO'`; `git diff --name-only -- src/native_audio.rs src/playback.rs src/mutator.rs src/acordator.rs` | oba symbole obecne; **diff plików audio pusty** (zmiany tylko kosmetyczne/testy) |
| **C09** | MUST-HAVE / MUST-NOT | Guard firmware'u i zapisy wyłącznie do `/run` | `unzip -p build/arpeggigator.zip autorun/autorun.sh \| grep -nE '44507c7a…\|cp \|/etc\|/data\|/content\|/factory\|/usr\|/opt'` | guard `44507c7a7d3029c411b2e3780ea286ca3e08b7e3ffd7b5047d52f73bf5299d1f` przed pierwszym `cp`; **zero** trafień poza `/run` |
| **C10** | MUST-HAVE | `verify-v070.py` → 20/20 | `python3 scripts/verify-v070.py && grep 'Wynik:' docs/VP-0.0.7.0-VERIFY-OUTPUT.md` | `Wynik: 20/20 kontroli PASS`, `exit 0` |

### 1.1 Komendy rozwinięte — C04…C10 (bez probe'ów, bezpieczne, ale nadal nieuruchomione przeze mnie)

```bash
cd /Users/fafi/Downloads/ArpeggiGator
date -u && git rev-parse HEAD

# C04 — stare napisy nie mogą istnieć ani w binarium, ani w źródle
strings -a build/arpeggigator.so | grep -nE 'ACORDATOR|MODE: ACORD'   # OCZEKIWANE: pusto, rc=1
grep -nE 'ACORDATOR|MODE: ACORD' src/splash.rs src/lib.rs             # OCZEKIWANE: pusto, rc=1

# C05 — pojedyncze FRZ
printf 'FRZ_OFF_count='; strings -a build/arpeggigator.so | grep -c 'FRZ OFF'
printf 'NEW_LINE_count='; strings -a build/arpeggigator.so | grep -c 'SUBTLE FRZ:OFF'
# OCZEKIWANE: FRZ_OFF_count=0 ; NEW_LINE_count=1

# C06 + C07 — telemetria usługi po probe'ach
grep -h '^NRestarts=' docs/logs/ag6/night-20261118/qa/V072_{MUT,ACC,MUT_SHIFT,ACC_SHIFT}/12_service_RELEASED.txt
grep -h '^ABRT='     docs/logs/ag6/night-20261118/qa/V072_{MUT,ACC,MUT_SHIFT,ACC_SHIFT}/12_service_RELEASED.txt
# OCZEKIWANE: po 4 wiersze, same zera

# C08 — silnik dźwięku nietknięty
nm -D build/arpeggigator.so | grep -E 'g_probe|GATOR_NATIVE_AUDIO'
git diff --name-only -- src/native_audio.rs src/playback.rs src/mutator.rs src/acordator.rs
# OCZEKIWANE: oba symbole; druga komenda pusta

# C09 — guard i higiena zapisów
unzip -p build/arpeggigator.zip autorun/autorun.sh \
  | grep -nE '44507c7a7d3029c411b2e3780ea286ca3e08b7e3ffd7b5047d52f73bf5299d1f|cp |/etc|/data|/content|/factory|/usr|/opt'
# OCZEKIWANE: FW=...||exit 0 przed cp; zero ścieżek /etc|/data|/content|/factory|/usr|/opt

# C10 — paczka
python3 scripts/verify-v070.py
grep 'Wynik:' docs/VP-0.0.7.0-VERIFY-OUTPUT.md
# OCZEKIWANE: Wynik: 20/20 kontroli PASS
```

---

## 2. DOKŁADNE KOMENDY `ag6_probe.py qa` NA SCREENY V072_MUT / V072_ACC

Wszystkie `PROCEDURA → PROVEN po wykonaniu`. Profil `qa` = panel `http://127.0.0.1:8766`,
SSH `2032`, klucz `~/Downloads/virtual-mpc-sample-qa/out/keys/id_ed25519`.
Katalog wyników: `docs/logs/ag6/night-20261118/qa/<ETYKIETA>/`.

### 2.1 MUTATOR — mode 3 (etykieta `V072_MUT`)

```bash
cd /Users/fafi/Downloads/ArpeggiGator
python3 scripts/ag6_probe.py qa V072_MUT \
  '{"enter16":true,"mode":3,"setup_actions":[{"name":"DATA","action":"press","value":127}],"hold":[{"name":"PAD 4","value":100}],"animation":true}'
```

### 2.2 ACCORDATOR — mode 4 (etykieta `V072_ACC`)

```bash
python3 scripts/ag6_probe.py qa V072_ACC \
  '{"enter16":true,"mode":4,"setup_actions":[{"name":"DATA","action":"press","value":127}],"hold":[{"name":"PAD 4","value":100}],"animation":true}'
```

### 2.3 WARIANT ZE SHIFT — etykiety panelu widoczne (kryterium C01)

```bash
python3 scripts/ag6_probe.py qa V072_MUT_SHIFT \
  '{"enter16":true,"mode":3,"setup_actions":[{"name":"DATA","action":"press","value":127}],"hold":["SHIFT",{"name":"PAD 4","value":100}],"animation":true}'

python3 scripts/ag6_probe.py qa V072_ACC_SHIFT \
  '{"enter16":true,"mode":4,"setup_actions":[{"name":"DATA","action":"press","value":127}],"hold":["SHIFT",{"name":"PAD 4","value":100}],"animation":true}'
```

**⚠️ Pułapka z 7.1 (PROVEN z `QA_B1_mut_shift_b1/00_plan.json`):** nie używaj
`"hold":["SHIFT","SCREEN 1"]` do pokazania etykiet — `SHIFT+SCREEN 1` **przełącza mode**
(w tamtym zrzucie z mode=3 zrobił się mode=4 i widać `MODE : ACORD`). Sam `SHIFT` w `hold`
pokazuje etykiety B1/B2/B3 bez zmiany trybu.

**Oczekiwany wynik każdego probe (z `16_verdict.json`):**
```
pressed_1200x950 = true
no_meaningful_duplicate = true
restarts = "0"
abrt = "0"
PASS = true
```
Dodatkowo przy `animation:true`: `09b_screen1200_PRESSED_LATER.png` musi mieć **inny hash**
niż `06_screen1200_PRESSED.png` (ekran nie zamarzł).

---

## 3. OBIEKTYWNA KONTROLA „STAREGO NAPISU" (bez OCR)

### 3.1 Dlaczego nie OCR

`PROVEN`: w środowisku nie ma `tesseract`, ImageMagick (`convert`/`magick`) ani `PIL`.
Zamiast zgadywać „wygląda na ACCORD", używamy **twardych danych**: wycinamy ten sam
prostokąt z LCD 320×240 baseline'u 7.1 i z nowego zrzutu, liczymy SHA-256 oraz liczbę
różniących się bajtów pikseli. Różnica ≠ 0 dowodzi, że region się zmienił; kierunek
zmiany dowodzi `strings` z binarium (C04/C05) + podgląd cropu (człowiek czyta 3 słowa).

Dostarczony skrypt: **`docs/tools/v072_caption_hash.sh`** (PROVEN przetestowany).

### 3.2 Geometria regionów (współrzędne z `src/splash.rs`, odczyt PROVEN)

| Region | Źródło w kodzie | Crop `sips -c H W --cropOffset Y X` | Rozmiar |
|---|---|---|---|
| `B1_TITLE` | `draw_shift_labels`, `graphics.text(..., 13, 8, 88, 19, …)` (`splash.rs:1381`) | `-c 19 88 --cropOffset 5 13` | 19×88 (z 1px marginesem) |
| `MODE_TITLE` | `draw_engine_right_panel`, `text(mode_title, 185, 54, 118, 16, 13.0)` (`splash.rs:88`) | `-c 18 119 --cropOffset 53 184` | 18×119 |
| `PARAM_LINE` | `draw_engine_right_panel`, `text(param_line, 185, 72, 118, 16, 13.0)` (`splash.rs:91`) | `-c 18 119 --cropOffset 71 184` | 18×119 |

`cropOffset` liczy się od **lewego górnego rogu** `screen.png` 320×240.

### 3.3 Komendy — przygotowanie baseline'u (jednorazowo, PROVEN dane wejściowe)

```bash
cd /Users/fafi/Downloads/ArpeggiGator
BASE=docs/logs/ag6/night-20261118/qa

bash docs/tools/v072_caption_hash.sh crop $BASE/QA_B1_mut_shift_b1/09_device_PRESSED.png /tmp/v072_OLD_ACC_SHIFT
bash docs/tools/v072_caption_hash.sh crop $BASE/AB_A02_ACO/09_device_PRESSED.png          /tmp/v072_OLD_ACC
bash docs/tools/v072_caption_hash.sh crop $BASE/AB_A01/09_device_PRESSED.png             /tmp/v072_OLD_MUT
```
Oczekiwane hashe (PROVEN): `B1_TITLE cc8319fa…`, `MODE_TITLE ea060d6d…`, `PARAM_LINE da3a940b…`.

### 3.4 Komendy — po buildzie i po probe'ach (PROCEDURA → PROVEN po wykonaniu)

```bash
NEW=docs/logs/ag6/night-20261118/qa

# nowe crop'y z LCD 320x240 (09_device_PRESSED.png)
bash docs/tools/v072_caption_hash.sh crop $NEW/V072_ACC_SHIFT/09_device_PRESSED.png /tmp/v072_NEW_ACC_SHIFT
bash docs/tools/v072_caption_hash.sh crop $NEW/V072_ACC/09_device_PRESSED.png       /tmp/v072_NEW_ACC
bash docs/tools/v072_caption_hash.sh crop $NEW/V072_MUT/09_device_PRESSED.png       /tmp/v072_NEW_MUT

# porównanie regionów: PRZED vs PO
bash docs/tools/v072_caption_hash.sh diff /tmp/v072_OLD_ACC_SHIFT /tmp/v072_NEW_ACC_SHIFT   # C01: B1_TITLE > 0
bash docs/tools/v072_caption_hash.sh diff /tmp/v072_OLD_ACC       /tmp/v072_NEW_ACC          # C02: MODE_TITLE > 0
bash docs/tools/v072_caption_hash.sh diff /tmp/v072_OLD_MUT       /tmp/v072_NEW_MUT          # C03: PARAM_LINE > 0

# kontrola krzyżowa: nowy ACC vs nowy MUT — mode title MUSI się różnić
bash docs/tools/v072_caption_hash.sh diff /tmp/v072_NEW_ACC /tmp/v072_NEW_MUT
```

**Interpretacja wyniku `diff`:**

| Wynik | Znaczenie | Werdykt |
|---|---|---|
| `differing_bytes=0` w regionie, w którym spodziewamy zmiany | region identyczny jak w 7.1 → stary napis **prawdopodobnie nadal wisi** | **FAIL** — sprawdź `strings` i podgląd PNG |
| `differing_bytes>0` + `strings` bez starych stringów + podgląd z nowym napisem | zmiana dotarła na ekran | **PASS** |
| `differing_bytes>0`, ale `strings` nadal ma `ACORDATOR` | zmieniła się animacja/tło, nie napis | **FAIL** — nie ufaj samemu opisowi |

### 3.5 Opcjonalnie, jeśli ktoś doinstaluje OCR (teraz NIEDOSTĘPNE — INFERRED)

```bash
# WYMAGA: brew install tesseract  (NIE instalowałem, NIE uruchamiałem)
tesseract docs/logs/ag6/night-20261118/qa/V072_ACC/09_device_PRESSED.png stdout --psm 7 2>/dev/null \
  | grep -E 'ACCORD|ACORD'
tesseract docs/logs/ag6/night-20261118/qa/V072_ACC_SHIFT/09_device_PRESSED.png stdout --psm 7 2>/dev/null \
  | grep -E 'ACCORDATOR|ACORDATOR'
tesseract docs/logs/ag6/night-20261118/qa/V072_MUT/09_device_PRESSED.png stdout --psm 7 2>/dev/null \
  | grep -E 'FRZ'
```
Oczekiwane: `MODE: ACCORD`, `ACCORDATOR`, `SUBTLE FRZ:OFF`; zero `ACORD` (bez `D`),
zero `FRZ OFF`. **Nie traktuj OCR jako jedynego dowodu** — kanon 7.1: nie ufamy
testom opartym wyłącznie na heartbeat/OCR.

---

## 4. TABELA WYNIKÓW DO WYPEŁNIENIA

Wszystkie pola „PO" i „Verdict" uzupełnia wykonujący po buildzie. Hashe „PRZED" są
PROVEN (obliczone przeze mnie z zrzutów 7.1).

| Probe / kryterium | PRZED — plik 7.1 | PRZED — hash regionu | PO — plik V072 | PO — hash regionu | `differing_bytes` PRZED→PO | NRestarts | ABRT | Verdict | `date -u` |
|---|---|---|---|---|---|---:|---:|---|---|
| `V072_MUT` / C03 (`PARAM_LINE`) | `AB_A01/09_device_PRESSED.png` | `da3a940b2e7dd61d3963b8409f881a14c935ea926d554ee254e2200b3a533d1c` | `…` | `<<DO WYPEŁNIENIA>>` | `<<…>>` | `<<…>>` | `<<…>>` | `<<…>>` | `<<…>>` |
| `V072_MUT` / C08 (`MODE_TITLE`) | `AB_A01/09_device_PRESSED.png` | `61362e7efabc63d8db4b6a3c472817c41f95838d3d673002bec5e4cd4abead78` | `…` | `<<DO WYPEŁNIENIA>>` | `<<…>>` | `<<…>>` | `<<…>>` | `<<…>>` | `<<…>>` |
| `V072_ACC` / C02 (`MODE_TITLE`) | `AB_A02_ACO/09_device_PRESSED.png` | `ea060d6d933ded9833629b43f101c586b877ec3ffe75c7c3c24aec0616ee8cb1` | `…` | `<<DO WYPEŁNIENIA>>` | `<<…>>` | `<<…>>` | `<<…>>` | `<<…>>` | `<<…>>` |
| `V072_ACC_SHIFT` / C01 (`B1_TITLE`) | `QA_B1_mut_shift_b1/09_device_PRESSED.png` | `cc8319fa1ff6cb2afc95ddabac04f06bf4842153a9690d3c3da8350373992eca` | `…` | `<<DO WYPEŁNIENIA>>` | `<<…>>` | `<<…>>` | `<<…>>` | `<<…>>` | `<<…>>` |
| `V072_MUT_SHIFT` / C01B (`B1_TITLE`, kontrolny `MUTATOR`) | `<<brak zrzutu 7.1 z samym SHIFT w mode 3 — użyć `V072_MUT_SHIFT` jako nowego wzorca>>` | `n/a` | `…` | `<<DO WYPEŁNIENIA>>` | `n/a` | `<<…>>` | `<<…>>` | `<<…>>` | `<<…>>` |

**Tabela zbiorcza 10 kryteriów:**

| ID | Komenda skrócona | Wynik (PROVEN / FAIL) | Pełny dowód (ścieżka + hash) |
|---:|---|---|---|
| C01 | `diff OLD_ACC_SHIFT NEW_ACC_SHIFT` | `<<…>>` | `<<…>>` |
| C02 | `diff OLD_ACC NEW_ACC` | `<<…>>` | `<<…>>` |
| C03 | `diff OLD_MUT NEW_MUT` | `<<…>>` | `<<…>>` |
| C04 | `strings … \| grep -E 'ACORDATOR\|MODE: ACORD'` | `<<…>>` | `<<…>>` |
| C05 | `grep -c 'FRZ OFF'` / `grep -c 'SUBTLE FRZ:OFF'` | `<<…>>` | `<<…>>` |
| C06 | `grep -h '^NRestarts='` (4 pliki) | `<<…>>` | `<<…>>` |
| C07 | `grep -h '^ABRT='` (4 pliki) | `<<…>>` | `<<…>>` |
| C08 | `nm -D … \| grep -E 'g_probe\|GATOR_NATIVE_AUDIO'` + `git diff` | `<<…>>` | `<<…>>` |
| C09 | `unzip -p … autorun.sh \| grep -nE …` | `<<…>>` | `<<…>>` |
| C10 | `python3 scripts/verify-v070.py` | `<<…>>` | `<<…>>` |

**Werdykt końcowy (do wpisania):** `<<WSZYSTKIE 10 PASS → gotowe do A/B Jeremiasza >>`
albo `<<NIE NADAJE SIĘ DO DEPLOYU — patrz FAIL powyżej>>`.

---

## 5. JAK ZROBIĆ SCREEN W MOMENCIE NACIŚNIĘCIA PRZYCISKU

Wzorzec z 7.1, bez zmian. Harness `scripts/ag6_probe.py` robi to w kolejności
(PROVEN z odczytu kodu, linie 198–222):

```
192  setup_actions  → np. DATA press (HOLD w 16 Levels)
195  04_state_setup.json / 196  05_service_setup.txt
199  for item in hold:  control(item, "press", value)     ← wciskamy
204  time.sleep(0.3)
206  shot("06_screen1200_PRESSED.png")                    ← panel 1200x950, PRZED release
207  state("07_state_PRESSED.json")
208  service("08_service_PRESSED.txt")
209  device_shot("09_device_PRESSED.png")                 ← LCD 320x240, PRZED release
211  if animation: sleep(1.5) → 09b_…_PRESSED_LATER.png
216  for item in reversed(hold): control(item, "release") ← dopiero teraz puszczamy
220  shot("10_screen1200_RELEASED.png") / 221 state / 222 service
224+ 13_hashes.json, 14_dimensions.txt, 15_duplicate_check.txt,
      16_verdict.json, 17_log.json
```

**Kluczowe:** `06_screen1200_PRESSED.png` i `09_device_PRESSED.png` powstają
**zanim** harness wyśle `release`. Bez nich nie uznawaj weryfikacji za kompletną.
Jeżeli `before_h == pressed_h` przy niepustym `hold`, harness zapisuje
`no_meaningful_duplicate=false` i `PASS=false` — to sygnał, że screen „nie złapał"
naciśnięcia.

### 5.1 Wariant ręczny (curl + Chrome) — jeśli trzeba doobiecrać details

```bash
PANEL=http://127.0.0.1:8766
# wejdź w mode 3/4 i ustaw HOLD wcześniej (patrz 00_plan.json), potem:
curl -s -X POST $PANEL/control -H 'Content-Type: application/json' -d '{"name":"SHIFT","action":"press"}'
curl -s -X POST $PANEL/control -H 'Content-Type: application/json' -d '{"name":"PAD 4","action":"press","value":100}'
sleep 0.3
"/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" --headless --disable-gpu \
  --hide-scrollbars --window-size=1200,950 \
  --screenshot=docs/qa_screens/2026-11-18_V072_ACC_SHIFT_PRESSED.png $PANEL
curl -s $PANEL/screen.png -o docs/qa_screens/2026-11-18_V072_ACC_SHIFT_PRESSED_LCD.png
curl -s $PANEL/state > docs/qa_screens/2026-11-18_V072_ACC_SHIFT_PRESSED_state.json
# dopiero teraz release:
curl -s -X POST $PANEL/control -H 'Content-Type: application/json' -d '{"name":"PAD 4","action":"release"}'
curl -s -X POST $PANEL/control -H 'Content-Type: application/json' -d '{"name":"SHIFT","action":"release"}'
```
Status: **PROCEDURA → PROVEN po wykonaniu** (na zasadzie 7.0 §5.3).
Używaj nowej etykiety za każdym razem — nie nadpisuj starych dowodów `AB_*`.

---

## ZAKAZY (obowiązują mnie i wykonującego)

- Nie uruchamiać probe'ów ani deployu bez wyraźnej zgody fafi/Jeremiasza — A/B robi Jeremiasz po buildzie.
- Nie dotykać fizycznej MPC ani karty SD bez zgody i pełnej weryfikacji linia-po-linii (kanon 1 i 3).
- Nie traktować buildu `emulator-audio` jako dowodu dla fizycznego sprzętu (kanon 2):
  w paczce musi być `native-audio` + marker `GATOR_NATIVE_AUDIO`.
- Nie polegać wyłącznie na heartbeat/OCR — zawsze krzyżowo: crop + hash + `state.json` + `service.txt`.
- Nie zmyślać mtime, hashy ani „wygląda dobrze". Brak dowodu = wpisz **FAIL**.

---

## PODSUMOWANIE — 5 najważniejszych sekcji

1. **§0.2** — baseline 7.1 z PROVEN hashami (stare `ACORDATOR`, `MODE : ACORD`, `SUBTLE FRZ OFF FRZ:OFF`).
2. **§1** — 10 kryteriów MUST-HAVE/MUST-NOT, każde z komendą i oczekiwanym wynikiem.
3. **§2** — gotowe komendy `ag6_probe.py qa` dla `V072_MUT`, `V072_ACC` i wariantów `_SHIFT`.
4. **§3** — obiektywna kontrola starego napisu bez OCR: `sips` crop → SHA-256 → BMP byte-diff + `strings`.
5. **§4 + §5** — tabela wyników do wypełnienia i procedura screenu w momencie naciśnięcia (przed `release`).
