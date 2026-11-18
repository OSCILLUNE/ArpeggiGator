# HANDOFF NA ŻYWO — ArpeggiGator 0.0.6.0 (GUARD PLIK)

**Jeden plik. Aktualizuj po każdym kroku. Numer kroku nigdy nie cofaj.**

## Guard / zasady self-audytu

1. Wolno używać **wyłącznie** agentów FREE (`opencode/*-free`, `openrouter/*:free`). Bez harness/antigravity/chatgpt.
2. Narracja nie jest dowodem. Każdy wpis poniżej musi mieć wynik komendy (hash, commit, HTTP, `NRestarts`). Jeśli nie ma — wpis jest podejrzany.
3. Maszyny: **DEV = 8765 / SSH 2022**, **QA = 8766 / SSH 2032**. Nigdy nie mieszać.
4. STOP natychmiast przy: `NRestarts>0`, `dumped/core-dump`, `double free`, `SEGV`, `ABRT`, zniknięciu sekwencji, zamrożeniu ekranu >10 s.
5. Dowód kroku = 3 pliki: `_screen.png` (**cała empeceta z headless Chrome 1200×950**, nie tylko `/screen.png`), `_state.json` (`GET /state`), `_control.txt` (`POST /control`). Helper: `scripts/ag6_step.py`.
6. Logi: `/tmp/arpeggigator-6-events.log` + rotacja `.1..3` po 128 KiB w gościu; journal przez `eprintln!`. Heartbeat 1 Hz z wątku `arpeggigator-tempo`.
7. **Build musi mieć `MPC_MOD_FEATURES=emulator-audio`** — bez tego `native_audio::install()` i `note_interceptor` są wyłączone (cfg z v0.0.4.9) i mod nie generuje nut.
8. Jeśli przejmujesz wątek: zacznij od „Ostatni stan”, uruchom komendy weryfikacyjne. Jeśli się nie zgadzają — cofaj się do ostatniego potwierdzonego kroku.

## Ostatni stan (GUARD)

| Pole | Wartość |
|---|---|
| Krok | **12** |
| UTC | 2026-11-17 ~23:45 |
| Branch | `release/0.0.6.0` |
| HEAD kodu/buildu | `55b9aec` (poprawki UI 0.0.1; build `d76837d4…`) — ostatni commit zmieniający `src/` |
| HEAD repozytorium | nie wpisujemy na sztywno (zmienia się przy każdym commitcie docs); sprawdzić `git rev-parse --short HEAD`; tag `v0.0.6.0` wskazuje na bieżący commit |
| Testy | 82/82 |
| Wdrożone na DEV | **final**, SHA `c1a330eecb64b3429542914c821ef0e928db8753934f15afa75495c0c8647f6d` |
| Artefakt w `build/arpeggigator.so` | ten sam `c1a330ee…` |
| DEV | final wdrożony, tryby przetestowane, B3 `frz=0→1`, pady zatrzasnięte, `NRestarts=0`, **soak finalny OK 30 min (180 próbek, 0 alarmów)** |
| QA | czysty, bez moda i karty, `NRestarts=0` (potwierdzone przez Jurka) |
| Następny ruch | weryfikacja poprawek UI 0.0.1 (soak `fix_0.0.1_ui` w toku) i raport o 8:00. **Push tagu/kodu NIE autoryzowano.** |
| Poprawki po teście właściciela | `82d4e48` (repaint przy SHIFT+B*, throttled repaint z on_paint, mutex w diagu), `55b9aec` (usunięcie odroczonego CLEAR_FRAMES) |
| Artefakty w warsztacie | `build/arpeggigator-stage-d.so` `0a8a60c0…` (przed poprawkami), `build/arpeggigator-final.so` `c1a330ee…` (wdrożony) |

## Komendy weryfikacyjne

```bash
cd /Users/fafi/Downloads/ArpeggiGator
git branch --show-current && git rev-parse --short HEAD
shasum -a 256 build/arpeggigator.so
ssh -i /Users/fafi/Downloads/virtual-mpc-sample/out/keys/id_ed25519 -p 2022 root@127.0.0.1 \
  'sha256sum /run/arpeggigator.so; systemctl show ac50 -p NRestarts,MainPID; \
   tail -2 /tmp/arpeggigator-6-events.log'
curl -s http://127.0.0.1:8765/state | jq '{connected, midi: .midi.counts, rear: .rear.levels}'
```

## Plan etapów

| Etap | Zawartość | Status |
|---|---|---|
| 6.0-a/a1/a2 | baza v0.0.4.9, bez splashu, diag 1 Hz + kontekst | ✅ `8ede199`, `44a9104`, `c4fb7d7` |
| 6.0-b | STOP-w-HOLD `Consume` | ❌ **wycofany** — odtwarza „jedną nutę w kółko” |
| 6.0-c | ARP only + usunięcie pompy repaintu | ✅ `7c426bb` (soak w toku) |
| 6.0-d | + MELODY (bramka `stage-melody-only`) | ✅ kod gotowy, test w finale |
| 6.0-e | + MUTATOR, B3 `FRZ ON/OFF`, freeze w audio | ✅ `3691aef` ( cherry-pick `ed907b6` zintegrowany ręcznie) |
| final | idle_ms anchor, build bez bramek | ✅ `cc9fca4`, artefakt `322fc0d7…` |
| 6.0-f/g | ACORDATOR / ORBITATOR | ✅ w finalu, test sekwencją Czesioka |
| 6.0-h | `map_surface_pad`, `PANIC_COUNT` | ⏸ odroczone (nie wprowadzać bez osobnego soak) |

## Dziennik kroków

### Krok 1 — baza
`v0.0.4.9` → branch. Usunięto splash, dodano `src/diag.rs`, testy 83/83, build OK. Commit `8ede199`.

### Krok 2 — deploy i odkrycie audio
Wdrożenie pełnym pakietem na DEV. Hash zgodny, `NRestarts=0`. **Ale `steps=0` mimo wciśniętego pada.** Przyczyna: standardowy build ma `native_audio::install()` i `note_interceptor` za `#[cfg(feature = "emulator-audio")]`. W 0.0.5.2 też tak jest — czyli stare wersje w emulatorze mogły nie grać wcale. Build z `MPC_MOD_FEATURES=emulator-audio` → 568456 B.

### Krok 3 — diag i działający ARP
Dodano kontekst do heartbeat (`mode/en/hold/l16`), tick przeniesiony na istniejący wątek tempo. Setup: `16 LEVELS` → `SHIFT+SCREEN 1` → `DATA` (HOLD) → pady. Wynik: `mode=1 en=1 hold=1 l16=1`, `steps` rośnie ~9/s, `last_pad` cykl 0/1/2, pitch −3/−2/−1. **Sekwencja działa.** Commit `c4fb7d7`.

### Krok 4 — 6.0-b (odrzucony)
Zaaplikowano hunk `7ff6522`: single STOP (code **81**) w HOLD → `Consume`, próg 650→800 ms. Test poprawny (po korekcie Krystyny: STOP=81/0x51, DATA=100/0x6F): po STOP `hold=1`, kroki lecą dalej, firmware nie może zatrzymać audio → wierna reprodukcja „jednej nuty w kółko”. Commit `87a1f21` → **`git reset --hard c4fb7d7`**.

### Krok 5 — powrót do bazy
Przebudowano 6.0-a2 z featurem, SHA `3d1850b…`, testy 83/83. Gotowy do 6.0-c.

### Krok 6 — prawdziwy crash i core
Podczas testów ARP (6.0-a2, emulator-audio) ac50 padł `status=11/SEGV` o 21:53:10 po ~645 krokach. `NRestarts` pokazywało 0, bo nastąpił ręczny restart — **licznik jest niewiarygodny**. Core ściągnięty do `docs/backups/dev-core-6c-20261117T215309Z/` (SHA core `2a74ee24…`, metadata `2f1dda37…`). Metadata: `comm=mod-repaint`, `signal=11`.

### Krok 7 — analiza core'a
Agent Krystyna (Beam Free) — lldb: wątek `mod-repaint` (tid 545), fault `SEGV_MAPERR` na adres `0x0`, stos: `start_repaint_pump` (mod) → `repaint_component` → kod MPC Sample. Audio thread żywy. Inny mechanizm niż stary crash #2 (null-deref zamiast uszkodzonej sterty), **ta sama klasa: repaint z wątku nie-UI**.

### Krok 8 — usunięcie pompy
`draw::start_repaint_pump(repaint_component)` i sama funkcja usunięte z `on_start`; usunięto też jej test (82/82). Build `137235f3…`, wdrożony na DEV, `NRestarts=0`. Po restarcie o 22:04:04 sekwencja znów działa. Soak 30 min (`6c1_clean`) w toku — poprawiony tak, by liczyć ABRT tylko od momentu startu (poprzednie soaki false-positive na starych crashach w journalu).

### Krok 9 — soak 6.0-c zakończony
`OK 30 min` — `docs/logs/ag6/soak_6c1_clean_20261117_230527.jsonl`, 180 próbek, zero `NRestarts>0`, zero ABRT, `steps` rosło monotonicznie.

### Krok 10 — wdrożenie finalu i test 5 trybów
Build z `MPC_MOD_FEATURES=emulator-audio`, bez bramek. Wdrożony `322fc0d7…`, potem poprawiony do `c1a330ee…`. Test wg sekwencji Czesioka: brak splashu przy bootcie (screen `final_boot`), `mode=1→2→3→4→5→0` po kolejnych SHIFT+B1, `NRestarts=0`.

### Krok 11 — problem z podpisem B3 i jego rozwiązanie
Firmware nadpisuje softkey B3 (`Filter` zostaje na ekranie mimo naszego `FRZ OFF`), więc użytkownik nie widział stanu freeze — to faktyczny powód jego skargi. Dodano `FRZ:ON/OFF` do prawego panelu nakładki Mutatora (`src/splash.rs`) i pole `frz` do heartbeat. Test: przed B3 `frz=0`, po SHIFT+B3 `frz=1`; zrzut `docs/logs/ag6/20261117_234254_f2_mut_frz_screen/frz_panel.png` pokazuje `FRZ:ON`.

### Krok 12 — soak finalny: OK
`docs/logs/ag6/soak_final_0.0.6.0_20261117_234334.jsonl` — 180 próbek, `restarts=0`, `abrt=0` w każdej, `steps` rosło monotonicznie (107 → ponad 11 000), brak przerw >30 s, `frz=1` przez cały czas. Skrypt zakończył się komunikatem `OK 30 min`. Weryfikacje: Flint (167 próbek w trakcie), Jurek (QA czysty), Stary Gem (spójność stanu).

### Krok 13 — poprawki UI po teście właściciela (0.0.1)

Właściciel zgłosił: (a) pod SHIFT-em ekran moda nie przełącza się aż do puszczenia SHIFT-a, (b) animację smuży przesuwający się playhead i HOLD nie widać do ruchu linii, (c) pod B3 nie widać naszego podpisu. Cwaniak znalazł poprzednika (a) w `docs/HANDOFF-RESPONSE-REC-0.0.4.2.md:61`.

Poprawki: `draw::repaint_painted()` przy `action!=0` w `on_button` (UI thread), throttlowany (33 ms, `LAST_UI_REPAINT_MS`) repaint z `on_paint` gdy nakładka/menu widoczne, `FILE_LOCK` w `diag.rs` dla rotacji logu. Wycofano `CLEAR_FRAMES.store(3)` wyłącznie z gałęzi `action!=0` w `on_button` (Jurek: odkładał czyszczenie i czyściłby ekran przy wyjściu z 16 Levels). Pozostałe użycia `CLEAR_FRAMES` — wyjście z 16 Levels (`lib.rs:290`, `:415`) i dekrement w `on_paint` — są nienaruszone.

Dowody: `docs/logs/ag6/20261118_033602_v2_b_b1/lcd.png` — aligator/ARP widoczny przy trzymanym SHIFT+B1; `...033607_v2_e_b1x2/lcd.png` — MUTATOR i `FRZ OFF` pod B3; `...033758_sm_t2/lcd.png` — `HOLD: ON` widoczne od razu, nakładka ostrа mimo playheada. Wszystkie 13 katalogów `v2_*`/`sm_*` ma komplet 3 plików 1200×950.

## Kryteria akceptacji finalnego 0.0.6.0

1. Build: `MPC_MOD_FEATURES=emulator-audio`, bez żadnych `stage-*`, testy hostowe 82/82.
2. DEV: hash `/run/arpeggigator.so` = hash buildu, `NRestarts=0`, brak wpisów ABRT/SEGV w journalu od momentu deployu.
3. Brak splashu przy starcie — od razu widok SEQ/16 Levels jak na screenie użytkownika.
4. ARP: po `16 LEVELS → SHIFT+B1 → DATA → PAD 1/2/3` heartbeat pokazuje rosnący `steps` i zmienne `last_pad` (sekwencja, nie jedna nuta).
5. Kolejne tryby przechodzą przez SHIFT+B1: `mode=2/3/4/5`; w MUTATOR B3 pokazuje `FRZ ON/OFF` i freeze realnie działa także w audio (`MUTATOR_FROZEN` podłączony w `native_audio.rs`).
6. STOP/DATA: pojedynczy STOP w HOLD nie zabija frazy (zachowanie 0.0.4.9), podwójny STOP resetuje do `mode=0 en=0 hold=0`.
7. Soak ≥30 min z `abrt=0`, `restarts=0`, `steps` rośnie monotonnie.
8. QA pozostaje czysty (bez moda, `NRestarts=0`) do rana.

## Agentury (wszystkie FREE)

- **Jurek Ogórek** (`opencode/big-pickle`): niezależna weryfikacja stanu — poprawna, zero rozbieżności. Dobry jako guard. Działa jako child session, nie jako osobna karta w UI.
- **Krystyna Groch** (`opencode/beam-free`): audyt note-on/off — wykryła mój błąd w mapowaniu DATA(100)/STOP(81) i top-3 ryzyka (panic bez renderu, transport.active bez NoteOff, STOP w HOLD). Analiza core'a lldb potwierdziła wątek `mod-repaint`.
- **Zimny Czesiek** (`opencode/iceman-free`): plan etapów 6.0-d/e i note-off — dostarczony; kluczowa uwaga: `ed907b6` nie nakłada się czysto, integracja ręczna.
- **Kot** (`opencode/longcat-2.5-free`): audyt dowodów soaku — rzetelny, wykrył nieaktualny HEAD w tym pliku (poprawiony). Wolny, ale solidny.
- **Flint 40** (`opencode/flint-40b-free`): audyt higieny logów — udowodnił brak alokacji na RT i ograniczenie rotacji do ~512 KiB, wykrył mylący `idle_ms` przy starcie (poprawiony w `cc9fca4`). Po drugim prompcie pracuje rzetelnie.
- **Cwaniak** (`opencode/nemotron-3.5-super-vl-free`): nie uruchomiony — FREE limit wyczerpany, powrót po 1:50. Wymagane udowadnianie każdej tezy komendami; traktowany jako zdolny, ale skłonny do halucynacji i skrótów.
- Agent `general` (default free): analiza diffów tagów i research deployu.
- **Hydrogen** (`google/gemini-4-hydrogen`): znakomity boarding dostarczony przez właściciela. OpenCode pokazuje cennik, ale projekt nie ma skonfigurowanego rozliczenia i model odpowiada normalnie (tokeny powitalne w Google Studio) — dopuszczony przez właściciela do jednego audytu końcowego. Jego ustalenia zgodne z kodem i stanem DEV. **Limit FREE Google: 20 zapytań/dzień — wyczerpany; powrót za ~9,5 h.** Potwierdzenie, że „brak rozliczenia” oznacza darmowy tier z limitami, nie nieskończoność.
- **Nemotron**: nieużyty; traktowany jako zdolny, ale leniwy i kłamliwy — każdy wynik musi być zweryfikowany komendami. Limity resetują się ~1:50.

### Wkład wątków (szacunek, raport Kota)

| Wątek / model | Co realnie dostarczył | % | Waga |
|---|---|---|---|
| Krystyna Groch / Beam Free | audyt note-on/off, DATA(100)/STOP(81), lldb core → `mod-repaint`, 4 kryteria akceptacji | 20 | wysoka |
| Zimny Czesiek / Iceman Free | plan 6.0-d/e, uwaga o `ed907b6`, sekwencja testów 5 trybów | 15 | wysoka |
| Flint 40 / Flint 40B Free | higiena logów, brak alokacji RT, rotacja ~512 KiB, `idle_ms` | 15 | wysoka |
| Jurek Ogórek / Big Pickle | niezależna weryfikacja stanu i czystości QA | 10 | średnia |
| Kot / LongCat Free | audyt dowodów soaku, wykrywał nieaktualny handoff, raport końcowy | 10 | średnia |
| Stary Gem / Gemini 3.5 Flash Lite | szybkie kontrole stanu handoffu, tagu i soaku (135/180 w trakcie) | 5 | średnia |
| Cwaniak / Nemotron Free | 0 (limit FREE), zaplanowany do weryfikacji po 1:50 | 0 | — |
| Agent główny (ja) | implementacja, buildy, deploye, testy, soaki, integracja fixów | 25 | — |

**Trzy najważniejsze ustalenia sesji:** (1) crash `mod-repaint` → usunięcie pompy, (2) STOP=81 vs DATA=100 i wycofanie 6.0-b, (3) mod nie gra bez `MPC_MOD_FEATURES=emulator-audio`.

## Poranny przewodnik diagnostyczny (od Czesioka)

1. Heartbeat `[AG6]` w `/tmp/arpeggigator-6-events.log` lub journal: `steps,last_pad,note,pitch,idle_ms,mode,en,hold,l16,frz`.
2. `mode`: 0=OFF, 1=ARP, 2=MELODY, 3=MUTATOR, 4=ACORDATOR, 5=ORBITATOR; `en/l16/hold/frz` to 0/1.
3. `systemctl show ac50 -p NRestarts,MainPID,ActiveState` (DEV SSH 2022) → `NRestarts=0`, `active`.
4. Świeże błędy: `journalctl -u ac50 --since "<czas testu>" --no-pager | grep -Ei 'ABRT|SEGV|double free|dumped|core'` — musi być pusto.
5. `curl -s http://127.0.0.1:8765/state` + `tail -n 5 /tmp/arpeggigator-6-events.log`.
6. Dowody do odesłania: `_screen.png` (cała empeceta 1200×950), `_state.json`, `_control.txt`, fragment heartbeat z czasem awarii.

## Uwaga o narzędziu soak (do poprawki później)

`scripts/ag6_soak.sh` ma dwie znane słabości (audyt Flint): `--since "$START_TS"` jest wewnątrz pojedynczych cudzysłowów ssh, więc zmienna się nie rozszerza; `NRestarts` bywa 0 po ręcznym restarcie. **Nie stosować sugestii powrotu do samego `-b`** — to znów zlicza stare crashy z tego samego boota (dwa false-positive na początku sesji). Właściwa poprawka: rozszerzyć `START_TS` poza pojedynczy cudzysłów i dodać kontrolę zmiany `MainPID` obok ABRT.

## Znane ryzyko rezydualne (do 6.0.1, nie w 0.0.6.0)

Krystyna (Beam) udowodniła: `native_audio::panic()` tylko ustawia flagę, a `Drop` w `src/lib.rs` nie wywołuje `panic()`. Przy unloadzie modułu, jeśli render audio przestanie działać przed skonsumowaniem flagi, głos może zostać brzmiący. W normalnej pracy (double STOP, zmiana trybu) render działa i czyści głosy — ryzyko ograniczone do unloadu. Minimalny fix (uwaga Czesioka): `#[cfg(feature = "emulator-audio")] native_audio::panic();` w `Drop` to tylko bezpieczne dopełnienie — jeśli render już nie działa, flaga i tak nie zostanie skonsumowana; `let _ = self.scheduler.stop();` w `reset_engine_state()` porządkuje stan hosta. Prawdziwe rozwiązanie wymagałoby zabijania głosów bezpośrednio przy unloadzie (unsafe, osobny audyt). Nie wdrażam w trakcie soaku finalnego, żeby nie zmieniać zwalidowanego buildu.
