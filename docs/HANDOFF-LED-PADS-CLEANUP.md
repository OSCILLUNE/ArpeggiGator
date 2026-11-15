# ArpeggiGator — Handoff: Czyszczenie i Wygaszanie Diod LED Padów (v0.0.4.9-prep)

**Zalecany model:** **GPT 6.1-Terra** (lub **Gemini 3.8 Flash**)  
**Zalecany poziom wysiłku:** Średni (wymaga zrozumienia mechanizmu sygnałów w Linux/glibc oraz API `lighting` w SDK).  
**Główne pliki:** [`src/lib.rs`](file:///Users/fafi/.gemini/antigravity/worktrees/ArpeggiGator/gemini_shift_handoff/src/lib.rs), [`vendor/mpc-sample-sdk/src/platform/lighting.rs`](file:///Users/fafi/.gemini/antigravity/worktrees/ArpeggiGator/gemini_shift_handoff/vendor/mpc-sample-sdk/src/platform/lighting.rs)

---

## 1. Zgłoszony Błąd i Materiał Dowodowy (Zdjęcia)

Podczas wyłączania MPC Sample diody LED padów potrafią pozostać zapalone w panelu emulatora, nawet gdy urządzenie przejdzie w stan całkowitego wyłączenia (*"MPC is off"*).

Zabezpieczono 3 zrzuty ekranu w `docs/images/`:
1. [`docs/images/led-1-running.jpg`](docs/images/led-1-running.jpg) — stan normalnej pracy w 16 Levels (pady fioletowe, pad 4 zielony).
2. [`docs/images/led-2-shutdown-dialog.jpg`](docs/images/led-2-shutdown-dialog.jpg) — okno dialogowe zamykania systemu (*"Shutdown: Do you want to shut down?"*). Pad 1 świeci na pomarańczowo, Pad 4 na zielono.
3. [`docs/images/led-3-mpc-off-pads-still-lit.jpg`](docs/images/led-3-mpc-off-pads-still-lit.jpg) — ekran zgaszony (*"MPC is off - POWER on the rear panel turns it on"*), a **Pad 1 (pomarańczowy) i Pad 4 (zielony) nadal bezwzględnie świecą!**

---

## 2. Diagnoza Przyczyny Źródłowej

1. **Pamięć kontrolera panelu (MCU):**
   - Panel emulatora (`panel/server.py`) emuluje sprzętowy mikrokontroler połączony przez UART. Odbiera ramki SysEx formatu:
     `F0 47 7F 5C 65 00 04 <pad_idx> <r> <g> <b> F7`
   - MCU nie posiada własnego timeoutu – pamięta ostatni ustawiony kolor RGB dla każdego z 16 padów, dopóki nie otrzyma jawnej ramki z zerowym kolorem (`r=0, g=0, b=0`).
2. **Zachowanie procesu przy zamykaniu systemu:**
   - Gdy użytkownik potwierdza wyłączenie ("Yes"), systemd wysyła sygnał `SIGTERM` / `SIGINT` do procesów lub usługa `ac50` jest ubijana.
   - W bibliotekach dynamicznych (`.so` ładowanych przez `dlopen`/`LD_PRELOAD`) w Linuksie destruktor `Drop for ArpeggiGator` **nie jest wywoływany** przy nadejściu sygnału `SIGTERM`/`SIGKILL` ani nagłym wyjściu procesu!
   - W rezultacie kontroler panelu nigdy nie otrzymuje komendy wygaszenia padów.

---

## 3. Wymagane Rozwiązanie

### Krok 1: Instalacja procedury wygaszania przy wyjściu (Signal Handler & atexit)
- W module zainstaluj procedurę czyszczącą uruchamianą w momentach krytycznych:
  1. `libc::atexit`: zarejestruj funkcję czyszczącą wywoływaną przy normalnym wyjściu procesu z `libc::exit`.
  2. Obsługa sygnałów `SIGTERM`, `SIGINT`, `SIGHUP`: zarejestruj bezpieczny handler sygnałów (lub flagę atomową wymuszającą natychmiastowe wysłanie czerni do wszystkich 16 padów).
- Funkcja czyszcząca musi wysłać kolor czarny (`0x000000`, jasność 0.0) na wszystkie 16 padów bezpośrednio przez funkcję `send()` / `lighting::send_pad_light(pad, 0, 0.0)`.

### Krok 2: Wymuszenie zerowania przy wyjściu z 16 Levels i przy STOP
- W `reset_engine_state()` oraz przy podwójnym STOP:
  - Upewnij się, że wywoływane jest jawne wygaszenie:
    ```rust
    for pad in 0..16 {
        let _ = lighting::send_pad_light(pad, 0x000000, 0.0);
    }
    lighting::clear_pad_rgbs();
    ```
- Weryfikacja: upewnij się, że po wyjściu z modułu żaden pad nie pozostaje w stanie zawieszenia (wiszący kolor).

---

## 4. Weryfikacja

1. `cargo test --offline`: wszystkie testy muszą przechodzić.
2. W emulatorze (port 8765):
   - Włącz 16 Levels, zaświeć pady.
   - Zamknij usługę `systemctl stop ac50` lub zainicjuj procedurę shutdown.
   - Sprawdź stan panelu HTTP: wszystkie pady (1..16) muszą być zgaszone (ciemnoszare, brak kolorowej poświaty).
