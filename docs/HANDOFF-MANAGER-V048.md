# ArpeggiGator — Handoff dla Nowego Wątku Zarządzającego (v0.0.4.8 -> v0.0.4.9)

**Data utworzenia:** 15.11.2026  
**Zalecany model dla managera:** **Gemini 3.8 Medium** (niski koszt tokenów w rolach analityczno-zarządczych, doskonała percepcja kodu i logiki MPC).  
**Zalecany poziom wysiłku:** Średni (Medium).  
**Bieżący stan gałęzi `main`:** Commit `25bebe4` (tag `v0.0.4.8`), 83/83 testy jednostkowe na zielono.

---

## 1. Aktualny Status Testów i Weryfikacji

1. **Weryfikacja v0.0.4.8:**
   - **Potwierdzona przez użytkownika na środowisku DEV:** Nowy 2-blokowy layout prawej karty oraz krokodyl z akordeonem w trybie ACORDATOR działają i wyglądają świetnie. Zostają w obecnej formie!
2. **Aktualne zadanie priorytetowe (w toku):**
   - **Wygaszanie i czyszczenie podświetlenia padów LED po wyłączeniu MPC / wyjściu.**
   - Przygotowano dedykowany manual: [`docs/HANDOFF-LED-PADS-CLEANUP.md`](HANDOFF-LED-PADS-CLEANUP.md).
   - Zabezpieczono materiał dowodowy (zdjęcia problemu):
     - [`docs/images/led-1-running.jpg`](images/led-1-running.jpg) — stan gry (pady fioletowe, pad 4 zielony).
     - [`docs/images/led-2-shutdown-dialog.jpg`](images/led-2-shutdown-dialog.jpg) — okno zamykania (pad 1 pomarańczowy, pad 4 zielony).
     - [`docs/images/led-3-mpc-off-pads-still-lit.jpg`](images/led-3-mpc-off-pads-still-lit.jpg) — MPC wyłączone, a pady 1 i 4 nadal świecą w emulatorze.

---

## 2. Podsumowanie Wdrożeń Dnia Dzisiejszego (v0.0.4.1 -> v0.0.4.8)

1. **Snapshot QA:** Zweryfikowany snapshot w `/Users/fafi/Downloads/snapshots/qa-snapshot-20261115-1045` oraz `emmc.qcow2.pristine` w katalogu QA (port `8766`, SSH `2032`).
2. **Synchronizacja Transportowa (v0.0.4.2):** `TransportClock` i `TransportPlayback` powiązane z impulsami transportu (`trans_pos`), bez opóźnień i ubytków pętli (`trans_pos % loop_len`).
3. **Persystencja HOLD (v0.0.4.3):** Zapamiętywanie flagi `HOLD` (`ON`/`OFF`) przy przełączaniu silników (`SHIFT + B1`).
4. **Stabilność Silnika i Sterty JUCE (v0.0.4.4 – v0.0.4.5):** Strażnicy współbieżności RAII (`UiEventGuard`, `PaintGuard`) eliminujący wyścigi pamięci w JUCE (`RectangleList` SIGSEGV) oraz bezpieczne indeksowanie skal Orbitatora.
5. **Eliminacja Artefaktów i Pełne Tło (v0.0.4.6):** Solidne tło osi X (`x: 14..306`, szerokość 292 px, `y: 48`, wysokość 140 px).
6. **Live Sample Browsing pod Grającym Silnikiem (v0.0.4.7):** Ciągłość grania w tle po wyjściu do `SAMPLE`, dynamiczne przełączanie sampli padami 0..15, dwuklik STOP (< 650 ms) i 3-klatkowa pompa unieważniająca rynnę (Zero-Ghosting).
7. **Nowy Layout i Akordeon (v0.0.4.8):** 2-blokowy panel sterowania oraz aligator grający na akordeonie z animowanym miechem w ACORDATORZE.

---

## 3. Stan Maszyn i Portów

- **Instancja DEV (REC):**
  - Panel HTTP: `http://127.0.0.1:8765/`, SSH: port `2022`.
  - Biblioteka `arpeggigator.so` zaktualizowana do v0.0.4.8 (hash `9149e393...`), usługa `ac50` aktywna.
- **Instancja QA:**
  - Panel HTTP: `http://127.0.0.1:8766/`, SSH: port `2032`.
  - Maszyna czysta, stan fabryczny z zachowanym snapshotem.
  - Szybki rollback po testach:
    ```bash
    cd /Users/fafi/Downloads/virtual-mpc-sample-qa
    python3 lab.py stop
    cp out/run/emmc.qcow2.pristine out/run/emmc.qcow2
    python3 lab.py run --headless
    ```

---

## 4. Zadania dla Nowego Managera

1. **Nadzorowanie wdrożenia wygaszania LED padów** na podstawie `docs/HANDOFF-LED-PADS-CLEANUP.md`.
2. **Przeprowadzenie kompleksowych testów akceptacyjnych na wolnej maszynie QA (port 8766)**:
   - Wdrożenie buildu na QA, testy nagrywania do `.xpj` z transportem (5 trybów), test zapętlenia, test dwukliku STOP, weryfikacja czystego rollbacku.
3. **Publikacja wydania:** `git push origin main --tags` po ostatecznym zatwierdzeniu.
