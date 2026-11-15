# ArpeggiGator — Handoff: Live Sample Browsing pod Grający Silnik (v0.0.4.7)

**Zalecany model:** Gemini 3.8 Medium  
**Zalecany poziom wysiłku:** Średni (wymaga koordynacji maszyny stanów w `src/lib.rs` oraz routingu nut/sampli w `src/native_audio.rs`)  
**Cel zadania:** Utrzymanie ciągłego odtwarzania sekwencji ArpeggiGatora w tle po opuszczeniu widoku `16 LEVELS` na główny ekran `SAMPLE` oraz dynamiczna zmiana podkładanego sampla padami na ekranie Sample.

---

## 1. Koncepcja i User Story

W produkcji muzycznej na samplerach Akai MPC kluczowy jest szybki odsłuch generowanego arpeggia/motywu z różnymi załadowanymi próbkami dźwiękowymi:
1. Użytkownik włącza ArpeggiGatora na ekranie `16 LEVELS` (w dowolnym z silników: `ARP`, `MELODY`, `MUTATOR`, `ACORDATOR`, `ORBITATOR`), ustala tempo/podziałkę i zatrzaskuje akord lub motyw (`HOLD: ON`).
2. Użytkownik wciska fizyczny przycisk `16 LEVELS`, co wyłącza tryb 16 Levels i powraca do widoku głównego **SAMPLE**.
3. **Oczekiwane zachowanie:**
   - **ArpeggiGator gra dalej w tle!** Nie wolno ubijać dźwięku ani resetować silnika.
   - Na ekranie `SAMPLE` użytkownik uderza kolejne fizyczne pady (0..15). W MPC Sample uderzenie pada w tym widoku wybiera aktywny sampel/program do edycji.
   - Grający w tle ArpeggiGator ma **w locie przełączyć instrument/sampel**, na którym odtwarza trwającą sekwencję, na pad właśnie uderzony przez użytkownika!
   - Użytkownik może w ten sposób „w biegu” przesłuchać ten sam pattern na stopie, basie, synthie, leadzie czy wokalu.
4. **Warunki zatrzymania grania w tle:**
   - Wciśnięcie fizycznego przycisku **STOP** (zawsze bezwzględnie wygasza głosy i zatrzymuje generowanie),
   - Zatrzymanie transportu sprzętowego sekwencera,
   - Ponowne wejście do `16 LEVELS` i manualne wyłączenie `HOLD` (kliknięcie `DATA`) lub przełączenie trybu silnika na `OFF` (`SHIFT + B1`).

---

## 2. Diagnoza Stanu Obecnego (Bariery w Kodzie)

Obecnie kod bazowy traktuje wyjście z `16 LEVELS` jako sygnał do całkowitej anihilacji stanu odtwarzania:

1. **W `src/lib.rs` (linie ~312–316):**
   ```rust
   #[cfg(feature = "emulator-audio")]
   if event.pressed && matches!(event.code, 81 | 6 | 7 | button::LEVELS) {
       // STOP, SAMPLE, SEQ and 16 LEVELS discard the current selection and kill sounding voices.
       PREVIEW_STATE.fetch_and(!(PAD_MASK | HOLD_ENABLED), Ordering::AcqRel);
       native_audio::panic();
   }
   ```
   *Błąd:* Wciśnięcie `button::LEVELS` lub `button::SAMPLE` natychmiast zeruje flagi `PAD_MASK | HOLD_ENABLED` i woła `panic()`.

2. **W `src/lib.rs` (linie ~357–359):**
   ```rust
   if !entering {
       self.reset_engine_state();
   }
   ```
   *Błąd:* Metoda `reset_engine_state()` ustawia `self.enabled = false`, `self.mode = EngineMode::Off`, czyści wszystkie 5 silników i zeruje `PREVIEW_STATE`.

3. **W `src/native_audio.rs`:**
   Natywny hook `capture()` odrzuca zdarzenia lub ubija głosy, jeśli `PREVIEW_STATE & PREVIEW_ENABLED == 0` lub gdy `in_16_levels` jest nieaktywne.
   Dodatkowo baza sampla (`Key.note` / indeks slotu MPC) jest powiązana ze stanem z 16 Levels, zamiast dynamicznie przyjmować nowy aktywny pad z ekranu Sample.

---

## 3. Wymagane Zmiany Architektoniczne

### Krok 1: Rozróżnienie wyjścia do tła od zatrzymania silnika
- Gdy użytkownik wciska `button::LEVELS` i wychodzi z 16 Levels (`!entering`):
  - Jeśli silnik jest aktywny (`self.mode != EngineMode::Off`) i ma zatrzaśnięte nuty (`HOLD: ON`) lub trwa playback transportu:
    - **NIE wywołuj** `self.reset_engine_state()`!
    - **NIE zeruj** `HOLD_ENABLED` w `PREVIEW_STATE`.
    - Ustaw flagę tła (np. `BACKGROUND_PLAYBACK_ACTIVE`).
  - Jeśli silnik nie grał ani nie był w HOLD, wyjście zachowuje się normalnie.

### Krok 2: Obsługa przycisku STOP
- Gdy `event.code == 81` (STOP):
  - Bezwzględnie wygaś wszystkie głosy: `PREVIEW_STATE.fetch_and(!(PAD_MASK | HOLD_ENABLED))`, `native_audio::panic()`, `self.reset_engine_state()`.

### Krok 3: Przełączanie aktywnego sampla na ekranie SAMPLE
- Gdy użytkownik znajduje się na ekranie `SAMPLE` (poza 16 Levels):
  - Uderzenie fizycznego padu (0..15) jest przechwytywane przez firmware MPC do zmiany aktywnego sampla.
  - Zdarzenie to (w `observe_surface` lub `capture` w `src/native_audio.rs`) powinno zaktualizować docelowy indeks próbki (`ACTIVE_SAMPLE_PAD`).
  - W pętli generowania nut (`native_audio::render`): generowane nuty powinny być kierowane na bazowy numer nuty / slot odpowiadający nowo wybranemu padowi, zachowując względne interwały arpeggia.

### Krok 4: Czystość Interfejsu Graficznego (UI)
- Na ekranie `SAMPLE`:
  - Nakładka ArpeggiGatora **musi być w 100% niewidoczna**.
  - Użytkownik ma widzieć wyłącznie natywny widok fali dźwiękowej, parametry samplera i wskaźniki Akai.
  - Funkcja `repaint_component` nie może odrysowywać dashboardu ArpeggiGatora poza ekranem 16 Levels.
  - Podświetlenie fizycznych padów powinno oddawać natywne zachowanie ekranu Sample (lub delikatnie pulsować wybranym padem bez zakłócania pracy).

---

## 4. Plan Weryfikacji i Testów

1. **Testy jednostkowe (`cargo test`):**
   - Dodać test symulujący:
     1. Włączenie 16 Levels,
     2. Uruchomienie Arp / Mutator z `HOLD: ON`,
     3. Wciśnięcie `button::LEVELS` (powrót do Sample),
     4. Asercja: silnik nadal pozostaje w stanie `enabled`, `PREVIEW_STATE` utrzymuje stan generowania,
     5. Wciśnięcie `STOP` (`code 81`): asercja: silnik natychmiast cichnie i przechodzi w stan czysty.
2. **Testy w emulatorze MPC Sample:**
   - Weryfikacja odsłuchu przez gniazdo audio / logi `native_audio`.
   - Zrzut ekranu panelu HTTP po wyjściu do Sample (potwierdzenie braku artefaktów graficznych na fali).

---

## 5. Wytyczne dla Wykonawcy
- Bazuj na najnowszym zatwierdzonym commitcie `main` (`4985384` / `v0.0.4.6`).
- Utwórz nową gałąź roboczą, np. `feature/live-sample-browsing`.
- Po zakończeniu implementacji i potwierdzeniu działania testów zgłoś raport wątkowi zarządzającemu.
