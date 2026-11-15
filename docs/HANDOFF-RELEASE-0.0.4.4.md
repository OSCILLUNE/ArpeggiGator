# ArpeggiGator — Raport Wydania v0.0.4.4 i Analiza Incydentu 14:08–14:09

**Wersja:** `v0.0.4.4`  
**Data:** 15.11.2026  
**Status:** Przetestowane, zatwierdzone przez użytkownika  

---

## 1. Zmiany Wdrożone w Wersji v0.0.4.4

### A. Persystencja stanu `HOLD` przy przełączaniu silników (`SHIFT + B1`)
- Wcześniejsze zachowanie: przełączenie trybu silnika kombinacją `SHIFT + B1` bezwarunkowo kasowało flagę `HOLD` do `OFF` i resetowało stan zatrzaśnięcia.
- Nowe zachowanie:
  1. Stan `HOLD` (`ON` / `OFF`) jest zapamiętywany w `PREVIEW_STATE` przy przechodzeniu pomiędzy wszystkimi aktywnymi trybami (`ARP` $\to$ `MELODY` $\to$ `MUTATOR` $\to$ `ACORDATOR` $\to$ `ORBITATOR`).
  2. Na ekranie zachowany jest zielony indykator `HOLD: ON`.
  3. Nowy silnik od razu działa w trybie zatrzaskiwania nut z poprawną semantyką (polifoniczny w ARP, monophoniczny z voice-stealingiem w Melody/Mutator/Acordator/Orbitator).
  4. Przejście w tryb `OFF` bezwarunkowo resetuje flagę `HOLD` do `OFF` i czyści nuty. Kolejne włączenie (`OFF` $\to$ `ARP`) startuje ze stanem domyślnym (`HOLD: OFF`).
  5. Przy przełączaniu wywoływane jest uciszenie głosów w locie (`native_audio::panic()`), zapobiegając wiszącym nutom poprzedniego silnika.

### B. Wzmocnienie stabilności silnika ORBITATOR
- Wyeliminowano ryzyko błędu indeksacji skali (`index out of bounds`) przy dynamicznej zmianie atmosfery w trakcie odtwarzania (`OrbitAtmosphere::cycle()` / `set_atmosphere()`).
- Różne skale mają różną liczbę stopni (`Ambient`: 7, `Space`: 9, `Dark`: 10, `Rhythm`: 8). Zabezpieczono indeks bieżący `self.current_index % scale.len()`, co całkowicie chroni silnik przed przekroczeniem zakresu tablicy.

---

## 2. Diagnoza Incydentu Crashu z Godziny 14:08–14:09

### Zgłoszenie:
Awaria procesu `MPC Sample` na maszynie wirtualnej w trakcie korzystania z trybu `ORBITATOR` (`Ambient` / `Flow`).

### Pobrane dowody:
- Dziennik systemowy `journalctl -u ac50`:
  `Nov 15 13:09:09 mpc-sample launch-ac50[830]: double free or corruption (!prev)`
  `Nov 15 13:09:10 mpc-sample systemd[1]: ac50.service: Main process exited, code=dumped, status=6/ABRT`
- Zrzut pamięci:
  `/data/coredumps/MPC-Sample-Main.2026-11-15T130909Z.core.zst` (522 MB nieskompresowany)

### Wyniki analizy w `lldb`:
1. **Wątek 1 (`MPC Sample Main`):**
   - Wątek główny przetwarzał kolejkę komunikatów JUCE (`MessageManager::runDispatchLoop` $\to$ `dispatchNextMessageOnSystemQueue`).
   - W trakcie zwalniania obsłużonego zdarzenia wywołał `free()`, w którym biblioteka `glibc` wykryła uszkodzenie nagłówków sterty (`double free or corruption (!prev)`) i wywołała `abort()`.
2. **Przyczyna źródłowa:**
   - Identyczna z incydentem opisanym w v0.0.4.2: tło `mod-repaint` wywołuje `sites::repaint_component(screen)` asynchronicznie poza wątkiem UI JUCE (`MPC Sample Main`). W JUCE `Component::repaint()` alokuje i modyfikuje regiony odświeżania na stercie bez blokady `MessageManagerLock`.
   - Długotrwałe odtwarzanie w trybie ciągłym (`HOLD: ON`) generuje nieustanne wywołania odświeżania co 60 ms, co przy zbiegu z alokacjami wątku głównego prowadzi po kilku minutach do wyścigu pamięciowego na stercie glibc.
3. **Orbitator:**
   - Sam moduł `OrbitatorEngine` jest czysto obliczeniowy, nie wykonuje żadnych alokacji stertowych (`alloc-free`) ani operacji `unsafe`. Zabezpieczono w nim dodatkowo indeksowanie skal przy przełączaniu atmosfer.

---

## 3. Integralność i Weryfikacja
- Komplet 80 testów jednostkowych przechodzi pomyślnie (`cargo test` oraz `cargo test --features emulator-audio`).
- Zbudowano wersję z opt-in audio dla emulatora (`scripts/build-emulator-audio.sh`).
- Zsynchronizowano binarkę na maszynie wirtualnej (`scripts/sync-emulator-autorun.py --audio-lab`).
