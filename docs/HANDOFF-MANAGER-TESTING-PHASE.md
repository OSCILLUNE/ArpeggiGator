# ArpeggiGator — Handoff Wątku Zarządzającego (v0.0.4.9 -> Przygotowanie Testów)

**Data utworzenia:** 15.11.2026 (godz. 20:06)  
**Poprzedni Manager:** Antigravity (Gemini 3.8)  
**Nowy Manager:** Nowy wątek przejmujący nadzór  
**Bieżący stan gałęzi `main`:** Commit `e8a7af1` (tag `v0.0.4.9`), 83/83 testy jednostkowe na zielono (`cargo test --offline`).  
**Status architekta testów:** Model **Sol** pracuje obecnie w osobnym wątku nad przygotowaniem harnessu i skryptów testowych. Użytkownik wklei wyniki pracy Sola w następnym prompcie.

---

## 1. Kontekst i Przebieg Sesji Poprzedniego Managera

1. **Wdrożenie i weryfikacja v0.0.4.8:**
   - Nowy 2-blokowy layout prawej karty oraz aligator grający na akordeonie w trybie `ACORDATOR` zostały w 100% zaakceptowane przez użytkownika.
2. **Wdrożenie i odbiór v0.0.4.9 (Czyszczenie LED padów po wyłączeniu MPC):**
   - Wątek wykonawczy dostarczył rozwiązanie oparte na funkcji `extinguish_pads()` (`src/lib.rs`) oraz systemd `ExecStopPost` ze skryptem `scripts/pads-off.sh` wysyłającym ramki SysEx bezpośrednio do urządzenia ALSA `Surface`.
   - **Kompleksowe testy akceptacyjne na maszynie QA (port 8766):**
     - Zgaszenie padów przy zatrzymaniu usługi `ac50`: potwierdzone (wszystkie 16 padów ma `[0, 0, 0]`).
     - Pełny Shutdown urządzenia (`Power: off`): potwierdzone, zero wiszących diod.
     - Podwójny STOP (< 650 ms) i wyjście z 16 Levels: natychmiastowe wygaszenie padów.
     - Procedura rollbacku QA z `emmc.qcow2.pristine`: zakończona pełnym sukcesem, QA wróciła do czystego stanu fabrycznego.
3. **Publikacja wydania:**
   - Podbito wersję w `Cargo.toml` do `0.0.4-9`.
   - Nadano tag **`v0.0.4.9`** i wykonano `git push origin main --tags`.
   - Wdrożono bibliotekę z obsługą audio (`MPC_MOD_FEATURES=emulator-audio`, hash `d8ce8c...`) na środowisko DEV (port 8765), zrestartowano `ac50`. Odsłuch audio w przeglądarce działa prawidłowo.

---

## 2. Stan Maszyn i Środowisk

- **Środowisko DEV (port 8765):**
  - Adres: `http://127.0.0.1:8765/`, SSH: port `2022`.
  - Załadowana biblioteka: `v0.0.4.9` z funkcją `emulator-audio`.
  - Usługa `ac50` jest aktywna. To jest jedyne środowisko, na którym mają odbywać się bieżące testy!
- **Środowisko QA (port 8766):**
  - **TWARDA REGUŁA:** **BEZWZGLĘDNY ZAKAZ DOTYKANIA QA BEZ WYRAŹNEJ ZGODY UŻYTKOWNIKA!**
  - Środowisko QA znajduje się w czystym stanie po fabrycznym rollbacku.

---

## 3. Aktualny Status: Prace Nad Testami

1. **Rola Sola:**
   - Sol (na poziomie myślenia Medium) tworzy obecnie skrypty i scenariusze testów na podstawie dokumentu [`docs/HANDOFF-TEST-SUITE-PREPARATION.md`](HANDOFF-TEST-SUITE-PREPARATION.md).
   - Testy mają obejmować:
     - **Część 1:** `ARP` (9 wzorców, oktawy 0..3, zmiana Rate, Hold, Stop) oraz `MELODY` (takty 1/2/4, warianty, dynamiczna zmiana prymy, ciągłość 2-minutowa).
     - **Część 2:** `MUTATOR` (intensywność, FREEZE, reset), `ACORDATOR` (akordeon, nastroje, strumming, brak stuck notes) oraz `ORBITATOR` (skale, WILD/CHAOS, stabilność sterty).
     - **Część 3:** Live Sample Browsing pod grającym silnikiem i powrót (Zero-Ghosting).
   - **Główny priorytet stabilności:** Weryfikacja, czy proces `ac50` nie restartuje się co 2 minuty (`systemctl show ac50 -p NRestarts,ActiveState`).
   - **Zrzuty całego panelu:** Użycie headless Chrome (`--window-size=1200,800`) w celu uwiecznienia całego urządzenia z kolorami padów 1..16.
   - **Zasada testów:** Wychwytywać i raportować błędy, ale ich **nie naprawiać**.

---

## 4. Instrukcja dla Nowego Managera

1. Przejmij rolę Managera ArpeggiGatora na gałęzi `main`.
2. Poczekaj na wklejenie przez użytkownika wyników/skryptu z wątku Sola.
3. Przeanalizuj przygotowane przez Sola materiały:
   - Sprawdź poprawność skryptów testowych (czy uderzają w DEV port 8765, czy robią pełne zrzuty, czy sprawdzają `NRestarts`).
   - Przygotuj się do skoordynowania ich wykonania przez lżejszy model (np. Gemini 3.8 Flash).
4. Pilnuj żelaznej zasady: **nie modyfikować QA (port 8766)**.
