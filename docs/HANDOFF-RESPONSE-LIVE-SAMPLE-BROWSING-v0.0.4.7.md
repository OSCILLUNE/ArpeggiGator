# Raport i Odpowiedź na Handoff: Live Sample Browsing pod Grający Silnik (v0.0.4.7)

**Data:** 15.11.2026  
**Autor:** Gemini Agent  
**Dotyczy:** `docs/HANDOFF-LIVE-SAMPLE-BROWSING.md` oraz dyspozycji Rafała dotyczącej dwukrotnego STOP  
**Gałąź robocza:** `live_sample_browsing` (baza: `4985384` / `v0.0.4.6`)  
**Status testów:** 83 testy jednostkowe zaliczone (100% PASS), buildy cross-compile ARM64 dla fizycznego MPC gotowe.

---

## 1. Realizacja User Story i Wymagań

Zgodnie ze specyfikacją zmanualizowaną w `docs/HANDOFF-LIVE-SAMPLE-BROWSING.md` oraz uwagą Rafała:

1. **Ciągłość odtwarzania w tle po wyjściu z 16 Levels:**
   - Użytkownik włącza dowolny silnik (`ARP`, `MELODY`, `MUTATOR`, `ACORDATOR`, `ORBITATOR`) na ekranie `16 LEVELS`, zatrzaskuje akord (`HOLD: ON`) lub uruchamia transport.
   - Po wciśnięciu fizycznego przycisku `16 LEVELS` urządzenie przechodzi do głównego ekranu `SAMPLE`.
   - **ArpeggiGator kontynuuje odtwarzanie sekwencji w tle!** Dźwięk nie jest przerywany, a stan silnika nie ulega resetowi.

2. **Dynamiczne przełączanie sampla w locie (Live Sample Browsing):**
   - Na ekranie `SAMPLE` uderzanie fizycznych padów `0..15` przełącza aktywny slot sampla w MPC.
   - Eventy padów są przekazywane do natywnego firmware'u (dzięki czemu MPC zmienia edytowany i podglądany slot w UI), a jednocześnie ArpeggiGator aktualizuje `ACTIVE_SAMPLE_PAD` oraz `ACTIVE_SAMPLE_NOTE`.
   - W pętli generowania nut (`src/native_audio.rs::output`) generowane nuty są w locie kierowane na nowo wybrany slot sampla, zachowując dokładnie względne interwały arpeggia, dynamikę i parametry silnika.
   - Uderzenia padów na ekranie `SAMPLE` nie naruszają ani nie mutują wzorca nut zatrzaśniętego w 16 Levels.

3. **Obsługa przycisku STOP (uwaga Rafała — podwójny STOP):**
   - **Pojedynczy STOP (`81`):** zatrzymuje odtwarzanie sekwencji sprzętowej sekwencera (transport stop). ArpeggiGator w tle gra dalej, nie tracąc zatrzaśniętych nut.
   - **Podwójny STOP (dwukrotne wciśnięcie w oknie <= 650 ms):** natychmiastowo i bezwzględnie wygasza wszystkie głosy (`native_audio::panic()`), zeruje `PREVIEW_STATE` oraz czyści stan silnika (`reset_engine_state()`).

4. **100% Czysty Interfejs Graficzny (UI):**
   - Poza 16 Levels `repaint_component` natychmiast zwraca `0`, uniemożliwiając jakiekolwiek wywoływanie odświeżania ekranu przez silnik ArpeggiGatora.
   - W `on_paint` nakładka ArpeggiGatora i menu SHIFT są pomijane, a diody LED padów zostają przywrócone do natywnego stanu MPC (`restore_pad_rgbs`).
   - Ekran Sample wyświetla wyłącznie czysty, natywny widok fali i parametrów Akai.

---

## 2. Zrealizowane Zmiany Architektoniczne

### `src/native_audio.rs`
1. **Atomics routingu sampli:**
   - Dodano `ACTIVE_SAMPLE_PAD: AtomicU8` (wartość domyślna `0xff` — brak nadpisania; wartości `0..15` — indeks aktywnego pada sampla).
   - Dodano `ACTIVE_SAMPLE_NOTE: AtomicU8` (wartość bazowa `36` = Pad A01).
   - Dodano helpery `set_active_sample_pad(pad, note)`, `active_sample_pad()` oraz `active_sample_note()`.
2. **Dynamiczny routing w `output()`:**
   - Zarówno w odtwarzaniu free-running (`playback`), jak i zsynchronizowanym z transportem (`transport_playback`), metoda `output()` pobiera `active_pad`.
   - Gdy `active_pad < 16`, `key.pad` i `key.note` są nadpisywane nowym slotem sampla, a `key.pitch` zachowuje pełną transpozycję sekwencji:
     ```rust
     let active_pad = ACTIVE_SAMPLE_PAD.load(Acquire);
     if active_pad < 16 {
         key.pad = active_pad;
         key.note = ACTIVE_SAMPLE_NOTE.load(Acquire);
     }
     let total_pitch = i32::from(key.pitch) + i32::from(step.transpose);
     key.pitch = total_pitch.clamp(-24, 24) as i8;
     ```
3. **Bezkolizyjne przechwytywanie w `capture()`:**
   - Gdy `BACKGROUND_PLAYBACK_ACTIVE` jest aktywne, uderzenia padów są forwardowane do firmware'u (`return None`), aktualizując jedynie `ACTIVE_SAMPLE_PAD` i `ACTIVE_SAMPLE_NOTE`. `PADS[0..15]` (wzorzec nutowy z 16 Levels) pozostaje nienaruszony.

### `src/lib.rs`
1. **Flagi i maszyna stanów:**
   - Dodano flagę `BACKGROUND_PLAYBACK_ACTIVE: AtomicBool`.
   - Dodano pomiar czasu podwójnego stopu `LAST_STOP_MS: AtomicI64`.
   - Wdrożono metodę `should_continue_in_background(&self) -> bool`, weryfikującą aktywny tryb silnika, włączony `HOLD: ON` (zatrzaśnięte nuty) lub trwający playback transportu sekwencera.
2. **Usunięcie agresywnego niszczenia stanu:**
   - Usunięto natychmiastowe zerowanie `PREVIEW_STATE` i `native_audio::panic()` na kodach `button::LEVELS`, `6` (SAMPLE) i pojedynczym `81` (STOP).
   - Wciśnięcie `button::LEVELS` przy wyjściu z 16 Levels aktywuje `BACKGROUND_PLAYBACK_ACTIVE` zamiast wywoływać `reset_engine_state()`.
   - Ponowne wejście do 16 Levels wyłącza `BACKGROUND_PLAYBACK_ACTIVE` i płynnie przywraca interfejs użytkownika.
3. **Logika podwójnego STOP (kod 81):**
   - Pojedyncze wciśnięcie STOP jest forwardowane do firmware'u bez przerywania grania silnika w tle.
   - Dwukrotne wciśnięcie w czasie <= 650 ms wykonuje pełny reset silnika, zerowanie buforów nutowych oraz `native_audio::panic()`.
4. **Izolacja zdarzeń padów w `observe_surface` i `on_pad`:**
   - Gdy `is_background` jest aktywne, pad press aktualizuje `ACTIVE_SAMPLE_PAD` i `ACTIVE_SAMPLE_NOTE`, nie modyfikując zatrzaśniętych nut w `PREVIEW_STATE`.

---

## 3. Testy i Weryfikacja

### Nowe testy jednostkowe:
1. `animation_input_tests::live_sample_browsing_persists_hold_and_redirects_pads_until_double_stop`:
   - Włączenie 16 Levels, start silnika Mutator, zatrzaśnięcie `HOLD: ON` z padem 0.
   - Wyjście przyciskiem `16 LEVELS` na ekran Sample.
   - Asercja: `BACKGROUND_PLAYBACK_ACTIVE` jest `true`, silnik pozostaje `enabled`, tryb to `Mutator`.
   - Uderzenie fizycznego padu 5: asercja przełączenia `ACTIVE_SAMPLE_PAD` na 5 (nuta 41), nienaruszenie wzorca w `PREVIEW_STATE`.
   - Uderzenie fizycznego padu 9: asercja przełączenia `ACTIVE_SAMPLE_PAD` na 9 (nuta 45).
   - 1x STOP: asercja utrzymania grania w tle i stanu silnika.
   - 2x STOP (double stop): asercja natychmiastowego wygaszenia, `module.enabled == false`, tryb `Off`, wyzerowanie `PREVIEW_STATE` i `active_sample_pad() == None`.
2. `animation_input_tests::live_sample_browsing_reentering_16_levels_resumes_seamlessly`:
   - Weryfikacja powrotu do 16 Levels z grającego tła, wyłączenia flagi tła, manualnego zwolnienia `HOLD` kołem DATA i czystego wyjścia.
3. `native_audio::tests::active_sample_pad_updates_and_resets`:
   - Weryfikacja atomowej aktualizacji i czyszczenia przez `panic()`.

### Wynik wykonania testów:
```
test result: ok. 83 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

### Weryfikacja kompilacji docelowej (ARM64 cross-compile):
- `./scripts/build.sh` — pomyślnie zbudowano paczkę produkcyjną `build/arpeggigator.so` (539 280 bajtów) oraz zsynchronizowano z autorunem emulatora (`/media/MPCSAMPLE/autorun/arpeggigator.so`).
- `./scripts/build-emulator-audio.sh` — pomyślnie zbudowano moduł lab audio `build/audio-lab/arpeggigator-audio-lab.so` (573 720 bajtów).
