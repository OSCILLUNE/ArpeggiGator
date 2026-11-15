# ArpeggiGator — Aktualny Rejestr Zadań (Stan po v0.0.4.9)

Data aktualizacji: 15.11.2026  
Bieżący stan `main`: Commit `dfa8c54` (tag `v0.0.4.9`).  
Wszystkie testy jednostkowe: 83/83 na zielono (`cargo test --offline`).

---

## 1. Zadania Funkcjonalne i UX (Status po v0.0.4.9)

### [ZREALIZOWANE w v0.0.4.7] Ciągłe odtwarzanie sekwencji w tle po wyjściu do ekranu SAMPLE (Live Sample Browsing)
- Utrzymanie grania w tle po wyjściu z `16 LEVELS` na stronę `SAMPLE` oraz dynamiczna zmiana podkładanego sampla uderzaniem w pady (odsłuch w locie).
- Zatrzymanie przyciskiem sprzętowym `STOP` (dwuklik < 650 ms) oraz 3-klatkowa pompa czyszczenia rynny ekranu (Zero-Ghosting).

### [ZREALIZOWANE w v0.0.4.8] Redesign prawego panelu kontrolnego i akordeon w Acordatorze
- Uporządkowanie i poprawa czytelności prawej karty: górny blok (MODE + parametry B2/B3) oraz dolny blok (RATE + instrukcja pokrętła + status HOLD).
- Krokodyl grający na akordeonie z dynamicznym miechem reagującym na tempo i impulsy w trybie `ACORDATOR`.
- Zweryfikowane i zaakceptowane w 100% przez użytkownika na środowisku DEV.

### [ZREALIZOWANE w v0.0.4.9] Czyszczenie i gaszenie podświetlenia padów LED po wyłączeniu MPC / wyjściu
- **Rozwiązanie w kodzie (`src/lib.rs`):** Wprowadzenie `extinguish_pads()` wysyłającego jawne ramki SysEx czerni (`0x000000`) na 16 padów i zwalniającego nadpisania; wywoływane w `reset_engine_state()`, przy podwójnym STOP oraz w destruktorze `Drop`.
- **Rozwiązanie systemowe (Shutdown / `systemctl stop ac50`):** Do pakietu autorun dodano skrypt `pads-off.sh` podpięty jako `ExecStopPost` w usłudze systemd `ac50`. Po zatrzymaniu usługi skrypt wysyła bezpośrednio do kontrolera ALSA `Surface` (`/dev/snd/midiC*D0`) ramki SysEx gaszące wszystkie 16 padów.
- **Weryfikacja QA (port 8766) i DEV (port 8765):** Potwierdzono wygaszenie padów 0..15 do `[0, 0, 0]` przy zamykaniu usługi, wyjściu z 16 Levels, podwójnym STOP oraz pełnym wyłączeniu zasilania (Power button -> Shutdown dialog -> poweroff).

---

## 2. Testy i Walidacja QA (Odizolowana Maszyna QA na porcie 8766)

*Środowisko QA posiada zweryfikowany snapshot `/Users/fafi/Downloads/snapshots/qa-snapshot-20261115-1045` oraz kopię szybkiego rollbacku `emmc.qcow2.pristine`.*

- [x] **Wdrożenie i testy akceptacyjne v0.0.4.9 na instancji QA:**
  1. **Wygaszanie LED przy Shutdown i zatrzymaniu usługi:** Potwierdzone pełne wygaszenie padów 0..15 do stanu `[0, 0, 0]`.
  2. **Podwójny STOP (< 650 ms) i wyjście z 16 Levels:** Pady natychmiastowo gasną po resecie.
  3. **Weryfikacja procedury rollbacku:** Przywrócenie czystego obrazu QA z `emmc.qcow2.pristine` zakończone sukcesem – maszyna powróciła do stanu fabrycznego bez wstrzykniętych modów.

---

## 3. Architektura, Stabilność i Fizyczny Sprzęt

- [ ] **Docelowa synchronizacja odrysowywania z pętlą zdarzeń JUCE (SDK)**
  - Kolejkowanie odświeżania na wątku UI (`MessageManager::callFunctionOnMessageThread` lub natywny timer JUCE).
- [ ] **Weryfikacja buildu na fizycznym standalone MPC Sample**
  - Potwierdzenie działania na fizycznym procesorze ARM urządzenia bez flag emulatorowych (`emulator-audio`).
