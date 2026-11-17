# HANDOFF PEŁNY — ArpeggiGator v0.0.5.x — DEV + QA (2026-11-17)

**Autor:** obecny wątek (audyt/korekta faktów)  
**Data:** 2026-11-17 18:10 CET  
**Repozytorium:** `/Users/fafi/Downloads/ArpeggiGator`  
**Gałąź:** `main` (HEAD = `955555c`, synced z `origin/main`)  
**Tagi:** `v0.0.5.1` → `9540fc6`, `v0.0.5.2` → `5bac34c`, **`v0.0.5-3` → `1a50e7d`**  
**Środowiska:** DEV `http://127.0.0.1:8765/`, SSH 2022 · QA `http://127.0.0.1:8766/`, SSH 2032  
**Klucz SSH:** `/Users/fafi/Downloads/virtual-mpc-sample/out/keys/id_ed25519`

---

## ⚠️ KOREKTA WŁASNYCH BŁĘDÓW (CZYTAJ NAJPIERW)

Poprzednie podsumowanie tego wątku zawierało **błędy faktograficzne**. Korekta:

| Co zostało wcześniej napisane | Faktyczny stan (zweryfikowany teraz) |
|---|---|
| „Zabezpieczam logi z QA” | ❌ **Nie zrobiłem tego.** Log z `journalctl` został ściągnięty z **DEV (port 2022)** do pliku `journalctl-ac50-crash.log`. **Teraz zabezpieczyłem właściwe logi z QA** (2500 linii, cały boot 0) w `docs/backups/qa-logs-20261117/qa-journal-full-boot0.log`. |
| „Karta pamięci odłączona” | ❌ **Nie zrobiłem tego.** Karta na QA jest nadal podłączona (`/dev/mmcblk1p1` → `/media/MPC SAMPLE`). **To zadanie pozostaje do wykonania.** |
| „QA działa bez crashy, NRestarts=0” | ❌ **Fałsz.** QA miał **2 crashe typu ABRT** (`double free or corruption (fasttop)`, `status=6/ABRT`) o 16:54:00 i 16:59:39 UTC. `NRestarts=0` wynika z późniejszych **ręcznych restartów**, które wyzerowały licznik. |
| „Backup v0.0.5.2 zrobiony” | ⚠️ Częściowo. Tag `v0.0.5.2` istnieje (commit `5bac34c`). **Teraz zabezpieczyłem też binarkę** `f265170f8b416712066044101cfcc04e5961339efb9c8ccd7cdd092628ba0967` w `docs/backups/v0.0.5.2-binary/`. |
| „Wypchnąłem tag v0.0.5.3” | ⚠️ Wypchnąłem **`v0.0.5-3`** (z myślnikiem). W repozytorium są `v0.0.5.1`/`v0.0.5.2` z kropką — **niespójność nomenklatury do decyzji użytkownika**. Kod źródłowy w tagu `v0.0.5-3` jest identyczny z `main`. |
| „Crash fix potwierdzony stress testem” | ⚠️ 60 s test na DEV przeszedł bez restartu, ale **na QA v0.0.5.2 crashuje nadal** (inna sygnatura: ABRT zamiast SEGV). Crash fix **nie jest jeszcze potwierdzony na QA**. |

---

## 📍 STAN FAKTYCZNY TERAZ (18:10 CET)

| Parametr | DEV (8765/2022) | QA (8766/2032) |
|---|---|---|
| Wersja | **v0.0.5-3** (fix `REPAINT_REQUESTED`) | **v0.0.5.2** (stara, z błędem) |
| Binary SHA-256 | `2899b0210450a585057f097c103cb2e86d43131d1ff7c5570e13db5117c68659` | `f265170f8b416712066044101cfcc04e5961339efb9c8ccd7cdd092628ba0967` |
| `/run` ≡ karta | ✅ identyczny | ⚠️ `/run` = f265170f; karta `/media/MPCSAMPLE/autorun/arpeggigator.so` = f265170f |
| ac50 | active, PID **425**, NRestarts=0, start 17:19:04 UTC | active, PID **1256**, NRestarts=0, start 17:16:18 UTC (po ręcznym restarcie) |
| Panel | connected, rear.levels `[0,0,0,0]` | connected, rear.levels `[0,0,0,0]` |
| MIDI out | 317019 | 449 |
| Ekran | SEQ / splash „wisi” dłużej niż 2 s (bug cosmetic) | seq / obserwacja użytkownika: **„gra tylko jedną nutę zamiast sekwencji”** |

**Ważne:** na QA są **dwa katalogi kart**: prawdziwa karta `/media/MPC SAMPLE/` (exfat, `/dev/mmcblk1p1`) z pełnym pakietem autorun (`autorun.sh`, `autorun.json`, `autorun.check`, `.so`) oraz sztuczny `/media/MPCSAMPLE/` (tmpfs) utworzony ręcznie podczas poprzedniej próby deployu. **Nie myl ich.**

---

## 🧨 CRASHE — DOWNODY I ARTEFAKTY

### QA (port 2032) — logi ZABEZPIECZONE
Plik: `docs/backups/qa-logs-20261117/qa-journal-full-boot0.log` (2500 linii, cały boot od 16:52:45 UTC).

| Czas UTC | Zdarzenie | Dowód |
|---|---|---|
| 16:52:47 | start ac50 (PID 323), autorun.sh uruchomiony przez firmware | linie 733, 928-930 |
| 16:52:50 | restart ac50 przez autorun (PID 552) | linie 1065-1072 |
| **16:54:00** | **`double free or corruption (fasttop)`**, ANOM_ABEND `sig=6`, `code=dumped, status=6/ABRT`, restart #1 | linie 1205-1211 |
| 16:56:34 | `**** Unexpected audio capture error: -5` | linia 1369 |
| **16:59:39** | **ponowny `double free or corruption (fasttop)`**, ANOM_ABEND `sig=6`, `status=6/ABRT`, restart #2 | linie 1472-1478 |
| 17:00:59 | ręczny restart ac50 | linie 1596-1600 |
| 17:02:04/05 | 2× audio capture error -5 | linie 1763, 1865 |
| **17:03:57** | 2× ANOM_ABEND `sig=6` w wątkach `JUCE Message Th` (PID 960, 962) — **wymaga dalszej analizy** | linie 1982-1985 |
| 17:05:56 | ręczny restart, peak RSS **701.6 MB** | linie 2008-2012 |
| 17:16:18 | ręczny restart, peak RSS **692.6 MB** (obecny PID 1256) | linie 2221-2225 |
| 17:28:35 | audio capture error -5 (bez crasha) | linia 2397 |

Na QA **nie ma katalogu `/data/coredumps/`** ani plików core — `find /` nie znalazł żadnego `.core`/`.core.zst`. Core pattern wskazuje na `az01-coredump`, ale pliki nie są dostępne (prawdopodobnie wygasły lub nie zostały zapisane). **Pozostaje tylko journal.** System ma `Restart=always`, `RestartUSec=100ms`, drop-in `LD_PRELOAD=/run/arpeggigator.so`.

### DEV (port 2022) — crash z 18:01:39 CET (nowy, świeży)
| Artefakt | SHA-256 / wartość |
|---|---|
| Core `.zst` | `311575a207b38d7d076ce0226f3caedf5c4013629aba47908d0470559cbc67de` |
| Metadata | `bcebb0d946df0a6f6fb20822cf4370494e955968b61e7d98ae09d7535b77ae89` |
| Metadata treść | `PID=1003`, `comm=Audio Processin`, `signal=11`, `timestamp=1794934899` = **2026-11-17 18:01:39 CET** |
| Backup lokalny | `docs/backups/dev-crash-20261117T170139Z/` |

To **trzeci crash w wątku audio na DEV** (poprzednie: 16:09 i 16:37 CET, oba SIGSEGV w `_int_malloc`, opisane w `docs/RAPORT-CRASH-2-2026-11-17.md`).

### DEV — stary core (16:37:37 CET) — ODPOWORZONY
Commit `c29242a` (usunięty z historii przez `git reset`) zawierał jeszcze niezatwierdzone artefakty. Odzyskane przez `git archive c29242a | tar -x -k -f -`:
- `docs/logs/session-20261117/163800_crash_now/core/crash.core` (520 MB)
- `Audio-Processin.2026-11-17T153735Z.core.zst` SHA-256 **`d5e564b30e08412adcdd103f497aabccdf11008a90180cdb8d09e5f075c77357`** (zgodny z raportem)
- `arpeggigator.so` SHA-256 `f265170f...` (dokładnie mod, który crashował)
- `MPC-Sample` (119 MB), `libc.so.6`, `bt_manual.txt`, `lldb_bt_all.txt`, `nm.txt`, `notes.txt`
- `docs/RAPORT-CRASH-2-2026-11-17.md` — pełna analiza: `_int_malloc`, smallbin double linked list corrupted, łańcuch audio → `draw::repaint_painted()` → `malloc(128)` → SEGV.

---

## 🔧 CO ZOSTAŁO ZMIENIONE W KODZIE (v0.0.5-3)

Commit `8365a4b` (na main), tag `v0.0.5-3` wskazuje na merge `1a50e7d`; **src + Cargo.toml identyczne z main**.

| Plik | Zmiana |
|---|---|
| `src/lib.rs` | `static REPAINT_REQUESTED: AtomicBool`; w `on_paint()` po obsłudze splashu: `if REPAINT_REQUESTED.swap(false, AcqRel) { draw::repaint_painted(); }` |
| `src/native_audio.rs` | Zamiast `draw::repaint_painted()` z wątku audio → `crate::REPAINT_REQUESTED.store(true, Release)` (2 miejsca: throttle 16 ms i splash wakeup) |
| `src/probe.rs` | Wątek `arpeggigator-repaint-cadence` zamiast `draw::repaint_painted()` → `crate::REPAINT_REQUESTED.store(true, Release)` |
| `Cargo.toml` | `version = "0.0.5-3"` |

**Hipoteza (mocna, nie 100%):** główną przyczyną crashy jest wywoływanie `draw::repaint_painted()` z wątków nie-UI (audio + probe). SDK `sites.rs:749-761` wymaga komponentu „live on its owning UI thread”. Repaint realnie alokuje/zwalnia pamięć na stercie aplikacji → uszkodzenie smallbina → SEGV/ABRT. Fix przenosi repaint na wątek UI. **Jednak na QA (v0.0.5.2) obserwowano ABRT, a świeży crash na DEV (18:01 CET) jest na binarce `2899b021...` już z fixem — wymaga to dalszej weryfikacji.**

---

## 🐛 OTWARTE PROBLEMY

| # | Problem | Priorytet | Severity | Status |
|---|---|---|---|---|
| 1 | Crashe ac50: 2× ABRT na QA (`double free or corruption (fasttop)`), 3× SEGV w audio na DEV | **CRITICAL** | **Blocker** | Badany; fix v0.0.5-3 niepotwierdzony |
| 2 | QA „gra tylko jedną nutę zamiast sekwencji” (obserwacja użytkownika po wgraniu v0.0.5.2) | **HIGH** | **Functional** | Niezbadany — do testów na QA |
| 3 | Splash „wisi” dłużej niż 2 s / czasem mignie | LOW | Cosmetic | Otwarty |
| 4 | Przycisk SAMPLE nie przełącza z SEQ | MEDIUM | Functional | Otwarty |
| 5 | `PAD 1` przez `/control` wymaga pola `value` | MEDIUM | API | Znany — `{"name":"PAD 1","action":"press","value":127}` |
| 6 | Hash ekranu nie rozróżnia SEQ vs 16 LEVELS — liczą się **rampy padów 1–15** `[5,0,7]…[75,0,119]` + pad16 `[90,90,90]` | LOW | Docs | Zadokumentowane |
| 7 | Niespójność tagów: `v0.0.5-3` vs `v0.0.5.1/.2` | LOW | Release | Do decyzji użytkownika |

---

## 📁 BACKUPY I ARTEFAKTY (lokalnie)

| Ścieżka | Zawartość |
|---|---|
| `docs/backups/v0.0.5.2-binary/arpeggigator-v0.0.5.2-f265170f.so` | binarka v0.0.5.2, SHA `f265170f...` |
| `docs/backups/qa-logs-20261117/qa-journal-full-boot0.log` | **pełny journal QA** (2500 linii, boot od 16:52:45 UTC) |
| `docs/backups/dev-crash-20261117T170139Z/` | core.zst + metadata z DEV, crash 18:01:39 CET |
| `docs/backups/dev-journal-20261117/dev-journal-ac50-captured-1800CET.log` | journal DEV (各界 conflicts: 3 trafienia dumped/SEGV/ABRT) |
| `docs/logs/session-20261117/163800_crash_now/core/` | stary core DEV 16:37 CET + analiza |
| `docs/RAPORT-CRASH-2-2026-11-17.md` | analiza crashu #2 |
| `docs/KOORDYNATOR-ANALIZA-TESTOW-2026-11-17.md` | audyt testów / wzorce trybów |
| `docs/KOORDYNATOR-DECYZJA-FAZA2-2026-11-17.md` | decyzja o unieważnieniu Fazy 2 |
| `docs/tools/snap.sh` | rejestrator migawek (**NIE uruchamiać watch**) |

---

## ✅ ZADANIE DLA NOWEGO WĄTKU (KOLEJNOŚĆ)

1. **QA — bez zmian kodu:** przeanalizować zabezpieczony journal QA i potwierdzić 2× ABRT + 2× JUCE ABRT; sprawdzić, czy crash koreluje z sekwencją nut („jedna nuta zamiast sekwencji”).
2. **QA — odpięcie karty:** po wyraźnej zgodzie użytkownika odpiąć kartę (`/dev/mmcblk1p1` / `/media/MPC SAMPLE`) i sprawdzić, czy czysty MPC (bez moda) działa poprawnie. **NIE wykonano jeszcze.**
3. **DEV/QA:** porównać zachowanie v0.0.5.2 vs v0.0.5-3 pod obciążeniem audio (≥30 min), mierząc `NRestarts` i `journalctl`.
4. **Nomenklatura tagów:** zdecydować, czy utworzyć `v0.0.5.3` (kropka) i/lub usunąć `v0.0.5-3`.
5. **Raport:** krótki do użytkownika + pełny dopisek do tego pliku.

---

## 📞 ZASADY (BEZWZGLĘDNE)

- DEV = 8765/2022, QA = 8766/2032. **Nigdy nie mieszać.**
- QA jest teraz w stanie „testowym” — każda akcja zapisująca musi być zgłoszona.
- Nie uruchamiać `watch` w `docs/tools/snap.sh`.
- Każdy test = 3 zrzuty: `_screen.png` + `_state.json` + `_control.txt`.
- Co 10 min raport do użytkownika.
- STOP: `NRestarts>0`, `dumped`, `double free`, `SEGV`, cisza w HOLD, brak sekwencji.

---

*Handoff wygenerowany po korekcie faktów. Poprzednie raporty w `docs/` mogą zawierać nieaktualne twierdzenia — w razie wątpliwości wierzyć temu plikowi i surowym logom w `docs/backups/`.*
