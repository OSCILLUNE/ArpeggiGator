# RAPORT KRYSTYNA-3 — WERYFIKACJA WIZUALNA v0.0.7.2 NA QA

**Autor:** KRYSTYNA-3 (zespół Jeremiasza). `date -u` pisania: **2026-11-18 22:12Z**.
**Plan wykonania:** `docs/WERYFIKACJA-WIZUALNA-0.0.7.2.md` (Czesiek-3), przeczytany w całości.
**Repo:** `/Users/fafi/Downloads/ArpeggiGator`, branch `release/0.0.7.2`,
HEAD w chwili wykonania: `2d81710d6d83fedf3a9173847f1eb800a369fe1b`
(plan Czesieka zakładał `542b33fa…` — różnica to commity Flint-3/Heńka/Jurka-3; nie wpływa na QA).

## ZAKRES I TRZY ZDANIA-KANON

Wykonałem **wyłącznie QA** (`/Users/fafi/Downloads/virtual-mpc-sample-qa`, panel `127.0.0.1:8766`,
SSH `2032`, VNC `5920`). **Nie** dotykałem DEV, fizycznej MPC, karty SD ani kodu `src/`.
Zmieniłem tylko ten raport, wiersz weryfikacji w `docs/HANDOFF-7.2-LIVE.md`
oraz pliki dowodów w `docs/logs/ag6/night-20261118/qa/V072_*` (wygenerowane przez harness).
`verify-v070.py` jest jawnie read-only (nagłówek skryptu) — nie deployuje.

1. **TRZEBA DOPILNOWAĆ I PRZEANALIZOWAĆ CAŁY KOD LINIA PO LINII, ŻEBY NIE ROZWALIĆ FIZYCZNEJ MPC SAMPLE.** — niniejszy raport dotyczy tylko emulatora; przed fizycznym włożeniem karty obowiązuje osobna, pełna analiza.
2. **CAŁY KOD TRZEBY PRZEPISAĆ NA NOWO — tu używaliśmy audio emulatora (`MPC_MOD_FEATURES=emulator-audio`).** — ten raport **nie** jest dowodem dla sprzętu; QA używa lab `.so` z emulowanym audio. Paczka `native-audio` została osobno zweryfikowana przez C10 (20/20).
3. **MUSIMY BYĆ PEWNI, ŻE PO WYJĘCIU KARTY SD Z FIZYCZNEJ MPC KONSOLA JEST W STANIE JAK PRZED WŁOŻENIEM KARTY.** — po stronie QA: `preload -- --off` → `MAPS=0`, pusty `DROPIN`, `NRestarts=0` (szczegóły §7).

---

## 1. PROWENIENCJA ARTEFAKTU I STAN STARTOWY QA

| Fakt | Komenda | Wynik |
|---|---|---|
| QA działa | `cd …/virtual-mpc-sample-qa && python3 lab.py status` | `panel 8766 open`, `ssh 2032 open`, `vnc 5920 open`, `qmp open`, `console open` |
| Hash artefaktu na hoście | `shasum -a 256 build/arpeggigator-v0.0.7.2-lab.so` | `3e2fac83cc4a9c5fe3421b2e58b4ec47f1943be60d1988cec01ef013d785aca8` ✅ zgodny z zadaniem |
| Preload | `python3 lab.py preload …/arpeggigator-v0.0.7.2-lab.so` | `active`, `mapped in MPC Sample` |
| Hash w `/run` | `sha256sum /run/mpclab-preload/arpeggigator-v0.0.7.2-lab.so` | `3e2fac83cc4a9c5fe3421b2e58b4ec47f1943be60d1988cec01ef013d785aca8` ✅ identyczny |
| Dropin | `cat /run/systemd/system/ac50.service.d/mpclab-preload.conf` | `Environment=LD_PRELOAD=/run/mpclab-preload/arpeggigator-v0.0.7.2-lab.so` |
| MAPS przed probe'ami | `grep arpeggigator /proc/$(pidof 'MPC Sample')/maps` | 7 wpisów (r--p/---p/r-xp/rw-p) ✅ |
| Telemetria przed probe'ami | `systemctl show ac50 -p NRestarts -p ExecMainStatus` | `NRestarts=0`, `ExecMainStatus=0` |

---

## 2. REPRODUKCJA BASELINE'U 7.1 (§0.2 planu) — PROVEN

Hashe pełnych `09_device_PRESSED.png` z 7.1 odtworzone 1:1:
`AB_A01` = `5bb9f9a8…c809554`, `AB_A02_ACO` = `3fc5ad07…c6950d`, `QA_B1_mut_shift_b1` = `7f2f0a91…6a5930`.

Crop'y `docs/tools/v072_caption_hash.sh crop` odtworzyły dokładnie hashe regionów z planu:

| Region | Baseline | Hash (zgodny z §0.2) |
|---|---|---|
| `B1_TITLE` | `QA_B1_mut_shift_b1` | `cc8319fa1ff6cb2afc95ddabac04f06bf4842153a9690d3c3da8350373992eca` |
| `MODE_TITLE` | `AB_A02_ACO` | `ea060d6d933ded9833629b43f101c586b877ec3ffe75c7c3c24aec0616ee8cb1` |
| `MODE_TITLE` | `AB_A01` | `61362e7efabc63d8db4b6a3c472817c41f95838d3d673002bec5e4cd4abead78` |
| `PARAM_LINE` | `AB_A01` | `da3a940b2e7dd61d3963b8409f881a14c935ea926d554ee254e2200b3a533d1c` |

---

## 3. PRZEBIEG 4 PROBE'ÓW (dokładnie komendy z §2 planu)

| Etykieta | Plan (skrót) | UTC (z `16_verdict.json`) | `pressed_1200x950` | `no_meaningful_duplicate` | `restarts` | `abrt` | `PASS` |
|---|---|---|---|---|---|---|---|
| `V072_MUT` | mode 3, DATA press, PAD 4 hold, anim | `2026-11-18T22:07:34Z` | true | true | 0 | 0 | **true** |
| `V072_ACC` | mode 4, DATA press, PAD 4 hold, anim | `2026-11-18T22:07:53Z` | true | true | 0 | 0 | **true** |
| `V072_MUT_SHIFT` | mode 3, hold `["SHIFT", PAD 4]`, anim | `2026-11-18T22:08:27Z` | true | true | 0 | 0 | **true** |
| `V072_ACC_SHIFT` | mode 4, hold `["SHIFT", PAD 4]`, anim | `2026-11-18T22:08:43Z` | true | true | 0 | 0 | **true** |

**Nie użyto `SHIFT+SCREEN 1`** (pułapka z 7.1 — przełącza mode). SHIFT w `hold` pokazał etykiety B1 bez zmiany trybu.
Wszystkie `06_screen1200_PRESSED.png` i `09_device_PRESSED.png` powstały przed `release`
(§5 planu; potwierdzone w `17_log.json`: release → dopiero `10_…_RELEASED.png`).

### 3.1 Hashe 4 screenów (twarde dowody)

| Etykieta | `06_screen1200_PRESSED.png` (1200×950) SHA-256 | `09_device_PRESSED.png` (320×240) SHA-256 |
|---|---|---|
| `V072_MUT` | `704bc9f94050c4666d68fa1cf6ccd769d1d844799c2d492fbff531394a2c273f` | `d6fc0ea705dbb1ec26a8604ff1d09e0a2739244a18f4098c12b1a54cbc996294` |
| `V072_ACC` | `33b445bae5f6f47314f4384d92aee02e47088ea6725c07117599917a912cbb0c` | `ba8e8037e4dafaea1be7b3e1b48662720757cdf4505dc19514cbdb761916c994` |
| `V072_MUT_SHIFT` | `8952442e44e1402edc904bd2632a39d11a1033123c9042d6ca170d58752b1879` | `707fe8f01088bb2cbf199be213aaedd60d82d3f55f8198dce1a5c3eafbb4af93` |
| `V072_ACC_SHIFT` | `8a3560e7eb13d44f74113d1499c6ecb0f66d7deb9dbc08eab272df81e73c217c` | `bcee16ff0ce040dfdf5de789da1c9c534f21bade5e031c37ef2aba897740a8b8` |

Wymiary sprawdzone `sips -g pixelWidth -g pixelHeight`: `06_*` = 1200×950, `09_*` = 320×240 (wszystkie 4).

### 3.2 Animacja (§2 — adnotacja OBS-1)

| Etykieta | `06` → `09b_screen1200_PRESSED_LATER.png` | `raw_duplicate` (z `15_duplicate_check.txt`) |
|---|---|---|
| `V072_MUT` | **inny hash** (`1a3406be…c976ec7c`) ✅ | `False` (4 unikalne) |
| `V072_ACC` | **inny hash** (`c2425666…96f17e4e`) ✅ | `False` (4 unikalne) |
| `V072_MUT_SHIFT` | ten sam hash ⚠️ | `True` (3 unikalne) |
| `V072_ACC_SHIFT` | ten sam hash ⚠️ | `True` (3 unikalne) |

**OBS-1 (nie jest jednym z C01–C10):** przy trzymanym SHIFT panel 1200×950 nie zmienił się w ciągu 1,5 s.
Nie jest to zamrożenie aplikacji: `before_equals_pressed=False`, `no_meaningful_duplicate=true`,
wszystkie verdicty `PASS=true`, a crop LCD (zrobiony w momencie naciśnięcia) zawiera poprawny napis.
Najpewniej SHIFT wyłącza pulsujący element panelu. Zgłaszam transparentnie, bez ukrywania.

---

## 4. TABELA §4 PLANU — PRZED vs PO (regiony LCD 320×240)

| Probe / kryterium | PRZED — plik 7.1 | PRZED — hash regionu | PO — plik V072 | PO — hash regionu | `differing_bytes` PRZED→PO | NRestarts | ABRT | Verdict | `date -u` |
|---|---|---|---|---|---:|---:|---|---|
| `V072_MUT` / C03 (`PARAM_LINE`) | `AB_A01/09_device_PRESSED.png` | `da3a940b2e7dd61d3963b8409f881a14c935ea926d554ee254e2200b3a533d1c` | `V072_MUT/09_device_PRESSED.png` | `7b2db79e53c0e39cfad1dd6081677f5ae31565e593d3c9b49783cadcfa9b3000` | **1819 / 6480 (28.07%)** | 0 | 0 | **PASS** | 2026-11-18T22:07:34Z |
| `V072_MUT` / C08 (`MODE_TITLE`) | `AB_A01/09_device_PRESSED.png` | `61362e7efabc63d8db4b6a3c472817c41f95838d3d673002bec5e4cd4abead78` | `V072_MUT/09_device_PRESSED.png` | `61362e7efabc63d8db4b6a3c472817c41f95838d3d673002bec5e4cd4abead78` | 0 / 6480 (0.00%) — napis MUTATOR bez zmian, zgodnie z planem | 0 | 0 | **PASS** | 2026-11-18T22:07:34Z |
| `V072_ACC` / C02 (`MODE_TITLE`) | `AB_A02_ACO/09_device_PRESSED.png` | `ea060d6d933ded9833629b43f101c586b877ec3ffe75c7c3c24aec0616ee8cb1` | `V072_ACC/09_device_PRESSED.png` | `beb718f8ff153ee06a0d808ca1dcf34a085485ae7a0395ec43031e350155177f` | **1234 / 6480 (19.04%)** | 0 | 0 | **PASS** | 2026-11-18T22:07:53Z |
| `V072_ACC_SHIFT` / C01 (`B1_TITLE`) | `QA_B1_mut_shift_b1/09_device_PRESSED.png` | `cc8319fa1ff6cb2afc95ddabac04f06bf4842153a9690d3c3da8350373992eca` | `V072_ACC_SHIFT/09_device_PRESSED.png` | `066ef2283dc63075f0e8700276835fbfcf3f735b1ff436c15fce88a639b52171` | **758 / 5016 (15.11%)** | 0 | 0 | **PASS** | 2026-11-18T22:08:43Z |
| `V072_MUT_SHIFT` / C01B (`B1_TITLE`, kontrolny `MUTATOR`) | brak zrzutu 7.1 z samym SHIFT w mode 3 | n/a | `V072_MUT_SHIFT/09_device_PRESSED.png` | `2e38f7314ba191c3bc808fa07d69a68e7067717c89013b1558a3bcf67db4a396` | n/a | 0 | 0 | **PASS** (nowy wzorzec) | 2026-11-18T22:08:27Z |

**Kontrola krzyżowa** `diff /tmp/v072_NEW_ACC /tmp/v072_NEW_MUT`:
`MODE_TITLE differing_bytes=4728/6480 (72.96%)`, `PARAM_LINE differing_bytes=4744/6480 (73.21%)` —
mode 3 i mode 4 są wyraźnie różne, screeny nie zostały podmienione.

### 4.1 Podgląd cropów (bez OCR — człowiek czyta 3 słowa)

`z_SHIFT` i `OLD` powiększone przez `sips -z` wyłącznie do wglądu (hashe liczone z oryginałów):

| Region | PRZED (7.1) | PO (0.0.7.2) |
|---|---|---|
| `B1_TITLE` (SHIFT) | `ACORDATOR` | **`ACCORDATOR`** |
| `MODE_TITLE` (mode 4) | `MODE : ACORD` | **`MODE : ACCORD`** |
| `PARAM_LINE` (mode 3) | `SUBTLE  FRZ OFF  FRZ:OFF` | **`SUBTLE FRZ:OFF`** (jedno FRZ) |

Kierunek zmiany zgodny z planem; w szczególności zniknął podwójny `FRZ`.

---

## 5. TABELA 10 KRYTERIÓW C01–C10

| ID | Komenda skrócona | Wynik | Pełny dowód (ścieżka + hash / wynik) |
|---:|---|---|---|
| **C01** | `diff /tmp/v072_OLD_ACC_SHIFT /tmp/v072_NEW_ACC_SHIFT` | **PASS** | `B1_TITLE differing_bytes=758/5016 (15.11%)`; `cc8319fa…` → `066ef228…`; crop: `ACCORDATOR`; `docs/logs/ag6/night-20261118/qa/V072_ACC_SHIFT/09_device_PRESSED.png` = `bcee16ff0ce040dfdf5de789da1c9c534f21bade5e031c37ef2aba897740a8b8` |
| **C02** | `diff /tmp/v072_OLD_ACC /tmp/v072_NEW_ACC` | **PASS** | `MODE_TITLE differing_bytes=1234/6480 (19.04%)`; `ea060d6d…` → `beb718f8…`; crop: `MODE : ACCORD`; `V072_ACC/09_device_PRESSED.png` = `ba8e8037e4dafaea1be7b3e1b48662720757cdf4505dc19514cbdb761916c994` |
| **C03** | `diff /tmp/v072_OLD_MUT /tmp/v072_NEW_MUT` | **PASS** | `PARAM_LINE differing_bytes=1819/6480 (28.07%)`; `da3a940b…` → `7b2db79e…`; crop: `SUBTLE FRZ:OFF` (bez `FRZ OFF`); `V072_MUT/09_device_PRESSED.png` = `d6fc0ea705dbb1ec26a8604ff1d09e0a2739244a18f4098c12b1a54cbc996294` |
| **C04** | `strings -a <lab.so> \| grep -E 'ACORDATOR\|MODE: ACORD'` | **PASS** (binarium) | `grep_rc=1`, zero trafień w `.so`. UWAGA adnotacja A-1: w `src/splash.rs:1382` jest jeden komentarz `// One character wider than the old "ACORDATOR"…` — to nie jest aktywny napis; `grep_rc=0` dotyczy tylko komentarza. |
| **C05** | `grep -c 'FRZ OFF'` / `grep -c 'SUBTLE FRZ:OFF'` + crop | **PASS** (semantycznie) | crop C03: dokładnie jedno `FRZ:OFF`, zero `FRZ OFF` w linii. UWAGA adnotacja A-2: literal `SUBTLE FRZ:OFF` nie występuje w `.so` (`count=0`), bo `splash.rs:185-190` składa go w runtime z `format!("{intensity_label} FRZ:{}",…)`; `FRZ OFF` (`count=1`) to osobny, poprawny napis B3 z `src/lib.rs:242` (`mutator_b3_caption()`), nie stary podwójny FRZ. |
| **C06** | `grep -h '^NRestarts='` (4 pliki `12_service_RELEASED.txt`) | **PASS** | 4 × `NRestarts=0` |
| **C07** | `grep -h '^ABRT='` (4 pliki) | **PASS** | 4 × `ABRT=0` |
| **C08** | `nm -D <lab.so> \| grep -E 'g_probe\|GATOR_NATIVE_AUDIO'` + `git diff` | **PASS** | `GATOR_NATIVE_AUDIO` @ `0x5aee R`, `g_probe` @ `0xb8070 D`; `git diff --name-only -- src/native_audio.rs src/playback.rs src/mutator.rs src/acordator.rs` → **pusto** |
| **C09** | `unzip -p build/arpeggigator.zip autorun/autorun.sh \| grep -nE …` | **PASS** | `FW=44507c7a7d3029c411b2e3780ea286ca3e08b7e3ffd7b5047d52f73bf5299d1f` w linii 5, `\|\| exit 0` przed pierwszym `cp` (linia 7); wszystkie zapisy → `/run/`; zero `/etc\|/data\|/content\|/factory\|/opt`. `/usr/bin/MPC Sample` to tylko odczyt guarda (`STOCK=`). |
| **C10** | `python3 scripts/verify-v070.py` | **PASS** | `Wynik: 20/20 kontroli PASS`, `exit 0`; ZIP `a882223bfea2d9e96e4e1d8d2067974c35952ad688526fe24122354fe1936fba`; zapis `docs/VP-0.0.7.0-VERIFY-OUTPUT.md` |

**Werdykt końcowy:** **10/10 kryteriów C01–C10 PASS** → gotowe do A/B Jeremiasza.
Dwie jawne adnotacje (A-1, A-2) dotyczą **proxy z planu**, nie zachowania na ekranie; obie wyjaśnione powyżej.
Dodatkowa obserwacja OBS-1 (statyczny panel przy SHIFT) nie jest kryterium i nie zmienia werdyktu.

---

## 6. ZDROWIE QA POD PROBAMI

| Kontrola | Wynik |
|---|---|
| `NRestarts` / `ABRT` (4 × `12_service_RELEASED.txt`) | same zera |
| `journalctl -u ac50 --since 22:05 --until 22:10 \| grep -iE 'segfault\|core\|crash\|abrt\|killed\|failed'` | **brak markerów crashy** |
| Stuck notes — panel `/state` 2× w odstępie 4 s po probe'ach | `midi.counts.out = 14651` w obu odczytach (brak nowych note-on); ostatnie zdarzenie MIDI to `83 24 00` = **Note off**. Brak stuck notes. |
| `MAPS` / `DROPIN` w trakcie probe'ów | `MAPS=7`, `DROPIN=mpclab-preload.conf` (zgodnie z preloadem) |
| `ActiveState` / `ExecMainStatus` | `active` / `0` |

> Uwaga do surowego logu MIDI: okno `midi.log` ma 100 wpisów i obejmuje też zdarzenia sprzed ~1h43m;
> w `V072_MUT` ostatnim wpisem jest note-on w momencie zrzutu stanu (snapshot „w połowie nuty"),
> ale natychmiast potem licznik `midi.out` przestaje rosnąć — to nie jest stuck note.

---

## 7. CLEANUP QA (ostatnia operacja)

Wykonane dokładnie `python3 lab.py preload -- --off` (**z `--`**, nie `preload --off`):

| Kontrola po cleanupie | Wynik |
|---|---|
| `lab.py preload -- --off` | `active` |
| `systemctl show ac50 -p NRestarts -p ActiveState -p ExecMainStatus` | `NRestarts=0`, `ActiveState=active`, `ExecMainStatus=0` |
| `MAPS` (świeży `SERVICE_CMD`: `grep -c arpeggigator /proc/$(MainPID)/maps`) | **`MAPS=0`** ✅ |
| `DROPIN` (`ls /run/systemd/system/ac50.service.d`) | **`DROPIN=` (pusto)** ✅ |
| `ABRT` (`journalctl -u ac50 -b \| grep -ciE 'ABRT\|SEGV\|double free\|dumped\|core'`) | **`ABRT=0`** ✅ |
| `/run/mpclab-preload/` | pozostały stare, **nieaktywne** pliki `.so` (`arpeggigator-060.so`, `arpeggigator.so`, `arpeggigator-v0.0.7.2-lab.so`) — brak `LD_PRELOAD`, `MAPS=0`, więc nic nie jest wczytane. Zgodne ze stanem „czyste po preload -- --off". |

**Stan końcowy QA: czysty, usługa `ac50` aktywna bez preloadu, `MAPS=0`, brak dropinów, `NRestarts=0`, `ABRT=0`.**

---

## 8. PODSUMOWANIE DLA JEREMIASZA

- Napisy `ACCORDATOR`, `MODE: ACCORD`, `SUBTLE FRZ:OFF` **dotarły na ekran LCD w QA** — udowodnione crop+hash+podglądem, bez OCR.
- Stare `ACORDATOR` / `MODE: ACORD` **nie istnieją w binarium**; stary podwójny `FRZ OFF FRZ:OFF` **nie jest już renderowany**.
- Audio, guard firmware'u i higiena zapisów do `/run` **bez zmian** (C08/C09/C10 = PASS, 20/20).
- 4 komplety screenów (`06_screen1200_PRESSED.png` + `09_device_PRESSED.png` + `09b_…_LATER.png`) w
  `docs/logs/ag6/night-20261118/qa/V072_{MUT,ACC,MUT_SHIFT,ACC_SHIFT}/`.
- QA po cleanupie w stanie wyjściowym. **Nie dotykałem fizycznej MPC ani karty SD** (kanon 1 i 3 nadal obowiązują przed A/B na żywym sprzęcie).

🪖☕
