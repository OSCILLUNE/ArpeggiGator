# Raport Przekazania Prac: Synchronizacja Nagrywania ArpeggiGator (v0.0.4.2)
**Odpowiedź na:** `docs/HANDOFF-REC-PO-0.0.4.1.md`  
**Dla:** Wątek Zarządzający / QA  
**Data:** 15.11.2026  
**Gałąź robocza:** `codex/recording-sync-0041`  
**Bazowy commit:** `32b217b` (`v0.0.4.1`)  
**Wydanie końcowe:** `v0.0.4.2` (commit `16c94e7930bb9160a5253392a6dfc534e152b6ec`)  

---

## 1. Podsumowanie Realizacji Zadań (Stan Przed vs Po)

Celem zadania było zbadanie, uporządkowanie i naprawa rejestracji generowanych nut ArpeggiGatora w sekwencerze Akai MPC Sample, ze szczególnym uwzględnieniem synchronizacji z transportem, kwantyzacji pierwszego kroku oraz eliminacji anomalii przy zapętleniu.

### Porównanie Zachowania Przed i Po Wdrożeniu:

| Obszar | Stan bazowy (v0.0.4.1 / przed) | Stan wdrożony (v0.0.4.2 / po) |
| :--- | :--- | :--- |
| **Zegar i faza generowania** | Silnik opierał się na zegarze czasu rzeczywistego (`clock::now_ms()`), nieskorelowanym z pozycją transportu sekwencera. Nuty rozjeżdżały się z siatką projektów MPC. | Wprowadzono precyzyjny `TransportClock` i `TransportPlayback`. Faza generowania kroków wynika wprost z odczytu impulsów transportu (`trans_pos`). |
| **Pierwszy krok sekwencji** | Zarejestrowano sztuczne opóźnienie (często brak pierwszej nuty na starcie lub opóźnienie o 120 ticks). | Formuła startowa `((trans_pos + step_pulses - 1) / step_pulses) * step_pulses` natychmiast wyrównuje i wyzwala pierwszą nutę na siatce startowej. |
| **Przejście przez pętlę (Loop Wrap)** | Sekwencer gubił nuty na granicy taktu; flaga zawinięcia powodowała fałszywy restart fazy lub nakładanie się Note On/Off. | Prawidłowe odwijanie pętli (`trans_pos % loop_len`) z zabezpieczeniem krawędzi pętli. Ciągłe nagrywanie wielu cykli bez utraty nut. |
| **Obsługa 5 silników** | Generowanie poprawne tylko w prostym ARP; brak weryfikacji MELODY, MUTATOR, ACORDATOR, ORBITATOR pod kątem zapisu w `.xpj`. | Wszystkie 5 silników przetestowane i zsynchronizowane pod kątem nagrywania transportowego. |
| **Integralność testowa** | 76 testów jednostkowych | 76 testów jednostkowych (`cargo test`) przechodzi w 100%. |

---

## 2. KRYTYCZNE WYTYCZNE DLA WSZYSTKICH WĄTKÓW I TESTÓW QA

> [!IMPORTANT]
> **Zasada nr 1 weryfikacji wizualnej: Obowiązek zrzucania całego panelu, a nie tylko wyświetlacza LCD.**
> Sam zrzut ekranu LCD (`/screen.png` o rozdzielczości 128x64) jest niewystarczający i wprowadza w błąd! Pokazuje jedynie bufor graficzny, uniemożliwiając ocenę stanu fizycznego urządzenia.
> Każdy wątek realizujący testy ma bezwzględny obowiązek generować pełny zrzut panelu (1200x950 px) przez bezgłowy Chrome pod adresem `http://127.0.0.1:8765/`:
> ```bash
> "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" --headless --disable-gpu --window-size=1200,950 --screenshot=panel.png http://127.0.0.1:8765/
> ```
> Pozwala to weryfikować:
> 1. Kolory diod LED przycisków funkcyjnych (SAMPLE, SEQ, SHIFT, 16 LEVELS, REC).
> 2. Dokładne barwy RGB siatki 16 padów, które jednoznacznie informują o bieżącym podtrybie i stanie zatrzaśniętych nut.
> 3. Monitor komunikatów MIDI wychodzących na zewnątrz.

> [!WARNING]
> **Zasada nr 2: Pułapka trybu 16 LEVELS (Velocity vs Tune).**
> - Domyślnie wciśnięcie przycisku `16 LEVELS` aktywuje podtryb **Velocity** (na LCD widać ikonę rampy `[/] 16`, a pady świecą fioletowym gradientem). W tym trybie uderzanie padów generuje wyłącznie ten sam dźwięk (np. C2) z różną głośnością!
> - Prawidłowe testy generowania melodii/akordów wymagają podtrybu **Tune** (ikona nutki `[♪] 16`, pady odpowiadają kolejnym półtonom chromatycznym).
> - Procedura przełączenia: `SHIFT + 16 LEVELS` $\to$ pojawia się okno dialogowe $\to$ obrót pokrętłem `DATA` CW o 1 krok (lub wciśnięcie `+`) $\to$ zatwierdzenie.

---

## 3. Rejestr Incydentów, Napotkanych Problemów i Analiza Coredumpa

### Incydent 1: Crash `SIGABRT` w `malloc_consolidate(): invalid chunk size`
- **Objaw:** Awaria procesu `/usr/bin/MPC Sample` na maszynie wirtualnej.
- **Zabezpieczenie materiału dowodowego:** Coredump został zabezpieczony na hoście w katalogu scratch:
  - `MPC-Sample-Main.2026-11-15T111419Z.core.zst`
  - `MPC-Sample-Main.core` (477 MB)
  - `mpc_sample_bin` (wyekstrahowana binarka 114 MB)
- **Diagnoza w `lldb`:**
  Wątek główny `MPC Sample Main` alokował pamięć w trakcie odrysowywania komponentu JUCE (`malloc(0x1900)`). Równolegle wątek tła `mod-repaint` wywoływał `sites::repaint_component(screen)` co 60 ms z funkcji `draw::start_repaint_pump` w `src/lib.rs`.
  W JUCE metoda `repaint()` nie jest bezpieczna wątkowo (brak `MessageManagerLock`), co powodowało wyścig danych i uszkodzenie metadanych sterty glibc.
- **Lekcja z próby usunięcia pompy:**
  Próba prostego wykomentowania `start_repaint_pump` doprowadziła do natychmiastowej awarii interfejsu użytkownika (GUI przestało reagować na softkeye B1–B3 pod ekranem przy trzymanym `SHIFT`, a zmiany pojawiały się dopiero po puszczeniu `SHIFT`).
  **Rekomendacja architektoniczna:** Pompa repainty musi pozostać aktywna do czasu opracowania w SDK bezpiecznego mechanizmu kolejkowania odświeżania na wątku UI (`MessageManager::callFunctionOnMessageThread`).

### Incydent 2: Blokady restartów `systemd` (Rate Limiting)
- Częste wywoływanie restartu usługi `ac50` w pętli testowej powoduje przekroczenie limitu `StartLimitBurst=5` w ciągu 60 sekund. Usługa przechodzi w stan `failed` (`StartLimitHit`).
- **Rozwiązanie:** Przed kolejnymi restartami w skryptach testowych należy wywoływać `systemctl reset-failed ac50`.

### Incydent 3: Ścieżki do narzędzi na hoście macOS
- Binarki `cargo` (`/opt/homebrew/Cellar/rustup/1.29.3/bin/cargo`) oraz `zig` (`/opt/homebrew/bin/zig`) nie są domyślnie obecne w nieinteraktywnych sesjach powłoki. W skryptach automatyzacji należy zawsze eksportować:
  `export PATH="/opt/homebrew/bin:/opt/homebrew/Cellar/rustup/1.29.3/bin:$PATH"`.

---

## 4. Nowe Zadanie do Backlogu (Dla Wątku Zarządzającego)

> [!NOTE]
> **Zadanie do dodania do listy zadań:**
> **Persystencja stanu `HOLD` przy zmianie trybów (`SHIFT + B1`).**
> - **Problem:** Obecnie każde kliknięcie `SHIFT + B1` (przełączenie trybu silnika np. ARP $\to$ MELODY $\to$ MUTATOR $\to$ ACORDATOR $\to$ ORBITATOR) bezwarunkowo resetuje flagę `HOLD` do stanu `OFF`.
> - **Oczekiwanie:** Użytkownik, który ustawił `HOLD: ON` i zatrzasnął nuty/pady, oczekuje zachowania ciągłości odtwarzania przy testowaniu różnych wariantów brzmieniowych bez konieczności ponownego wciskania pokrętła `DATA` po każdej zmianie trybu.

---

## 5. Podsumowanie Wersji i Artefaktów

- **Zatwierdzony Commit:** [`16c94e7`](file:///Users/fafi/.gemini/antigravity/worktrees/ArpeggiGator/arpeggigator_rec_sync_fix)
- **Tag:** `v0.0.4.2`
- **Zmiany w kodzie:**
  - Dodano moduły [`src/transport_clock.rs`](file:///Users/fafi/.gemini/antigravity/worktrees/ArpeggiGator/arpeggigator_rec_sync_fix/src/transport_clock.rs) oraz [`src/transport_playback.rs`](file:///Users/fafi/.gemini/antigravity/worktrees/ArpeggiGator/arpeggigator_rec_sync_fix/src/transport_playback.rs).
  - Wpięto obsługę transportową do [`src/native_audio.rs`](file:///Users/fafi/.gemini/antigravity/worktrees/ArpeggiGator/arpeggigator_rec_sync_fix/src/native_audio.rs) z zachowaniem fallbacku na podgląd free-run.
  - Zabezpieczono testy jednostkowe (76 passed).
  - Skompilowano i zsynchronizowano bibliotekę `arpeggigator.so` (hash `40d8b92dbee2de22efe96a5d67434092b824fe6428fa4cf0ef909784faae937d`) na maszynie wirtualnej.
