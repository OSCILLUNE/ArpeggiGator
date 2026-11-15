# ArpeggiGator — Raport Wydania v0.0.4.9 (Release Handoff)

**Data wydania:** 15.11.2026  
**Wersja:** `v0.0.4.9` (w `Cargo.toml`: `0.0.4-9`)  
**Commit bazowy:** `dfa8c54`  
**Autor poprawki:** Wątek Wykonawczy (Codex)  
**Nadzór i QA Acceptance:** Wątek Zarządzający (Manager)  

---

## 1. Zakres Wydania (Release Scope)

Wydanie `v0.0.4.9` rozwiązuje problem wiszących kolorów diod RGB padów (LED stuck colors) po wyłączeniu MPC oraz przy wyjściu z trybów grania.

### Zmiany techniczne:
1. **W kodzie modułu (`src/lib.rs`):**
   - Dodano funkcję `extinguish_pads()`: wysyła ona kolor czarny `0x000000` (jasność `0.0`) na 16 padów przez `lighting::send_pad_light(pad, 0, 0.0)` i dopiero potem zwalnia nadpisania `lighting::clear_pad_rgbs()`.
   - Podpięto `extinguish_pads()` pod `reset_engine_state()`, podwójny klik `STOP` oraz destruktor `Drop for ArpeggiGator`.
   - Zabezpieczono stan przed zbędnymi powtarzanymi resetami przy odmalowywaniu ekranów spoza modułu.
2. **W integracji systemowej autorun (`scripts/pads-off.sh`, `scripts/package-pad-cleanup.py`):**
   - Do pakietu autorun wstrzykiwana jest konfiguracja systemd `ac50.service.d/zz-arpeggigator-pads-off.conf` z dyrektywą:
     `ExecStopPost=-/run/arpeggigator-pads-off.sh`
   - Skrypt `pads-off.sh` po zatrzymaniu aplikacji wyszukuje kartę ALSA `Surface` i wysyła bezpośrednio do surowego portu MIDI (`/dev/snd/midiC*D0`) ramki SysEx gaszące wszystkie 16 padów.
   - Zapobiega to pozostawaniu kolorów w pamięci kontrolera MCU panelu po zamknięciu systemu.

---

## 2. Wyniki Testów Akceptacyjnych (QA Acceptance)

- **Testy jednostkowe:** 83/83 passed (`cargo test --offline`), czas wykonania 0.00s.
- **Kompilacja i pakowanie:** Pakiet `build/arpeggigator.zip` oraz biblioteka `build/arpeggigator.so` (hash `070caf...`) zbudowane bezbłędnie przez SDK.
- **Weryfikacja na wolnej maszynie QA (port 8766):**
  - Zaświecenie padów w trybie 16 Levels -> wywołanie `systemctl stop ac50`: wszystkie 16 padów natychmiast zgasiło się do `[0, 0, 0]`.
  - Zaświecenie padów -> pełny powerdown QEMU (`Power: off`): stan panelu `Pads 0..15: []` (wszystkie czarne).
  - Test podwójnego STOP (< 650 ms) i wyjścia z 16 Levels: natychmiastowe zgaszenie padów po powrocie do ekranu bazowego.
  - Test rollbacku: przywrócenie czystego stanu z `emmc.qcow2.pristine` przebiegło bez zakłóceń.
- **Wdrożenie na środowisko DEV (port 8765):**
  - Biblioteka `/run/arpeggigator.so` zaktualizowana do hash `070caf...`.
  - Usługa `ac50` aktywna, panel gotowy do pracy pod adresem: http://127.0.0.1:8765/.
