# Release Handoff & Crash Fix v0.0.4.5

## Podsumowanie wydania v0.0.4.5
Wydanie v0.0.4.5 eliminuje przyczynę awarii (`SIGSEGV` w wątku `mod-repaint`), która wystąpiła o godz. 14:23 podczas odtwarzania w trybie `ORBITATOR` (atmosfera: `Rhythm`, dynamika: `Wild`) z włączonym `HOLD: ON`.

---

### 1. Przyczyna źródłowa awarii o 14:23:08 (`mod-repaint` SIGSEGV)
* **Zdarzenie:** `audit[1218]: ANOM_ABEND comm="mod-repaint" sig=11 res=1`
* **Coredump:** `mod-repaint.2026-11-15T132308Z.core`
* **Analiza stosu (LLDB):**
  * Wątek `mod-repaint` wywołał funkcję `sites::repaint_component(screen)` z `start_repaint_pump`.
  * Wewnątrz biblioteki natywnej MPC Sample (JUCE): `Component::repaint()` wywołało wewnętrzną metodę zarządzania regionami odświeżania (`___lldb_unnamed_symbol_1147740`).
  * Na instrukcji `ldr w1, [x1, x19]` wystąpił odczyt z pustego wskaźnika (`x1 = 0x0`, `x19 = 0x0`).
  * Rejestry i pamięć wykazały, że wątek główny UI (`MPC Sample Main`) w tym samym momencie obsługiwał zdarzenie przycisku / modyfikował strukturę `juce::RectangleList` (bufor był w trakcie czyszczenia/realokacji: pole rozmiaru `w25 > 0`, ale wskaźnik danych `x1 == 0`).
  * Ponieważ przy trzymanym `HOLD: ON` flaga `PREVIEW_STATE & PAD_MASK != 0` była stale aktywna, wątek `mod-repaint` wysyłał zapytania o repaint nieprzerwanie 16,6 razy na sekundę (co 60 ms). Przy zmianie parametrów softkeyami (SHIFT + B2 / SHIFT + B3 na `RHYTHM` / `WILD`) doszło do kolizji z wątkiem głównym.

---

### 2. Zastosowane rozwiązanie architektoniczne
1. **Wyeliminowanie ciągłego spamowania repaintem w trybie HOLD:**
   * Usunięto warunek ciągłego odmalowywania `|| PREVIEW_STATE & PAD_MASK != 0` z `repaint_component()`.
   * Odświeżanie ekranu jest teraz sterowane deterministycznym oknem czasowym `DASHBOARD_REFRESH_UNTIL_MS` (400–500 ms) wyzwalanym wyłącznie wtedy, gdy użytkownik rzeczywiście wchodzi w interakcję z maszyną:
     * naciśnięcie / puszczenie padów (400 ms),
     * naciśnięcie przycisku DATA (przełączenie HOLD) (500 ms),
     * wciśnięcie SHIFT lub softkeyów B1, B2, B3 (500 ms),
     * obrót encodera (zmiana Rate) (500 ms),
     * faza boot splash (2 000 ms).
   * Podczas wielominutowego grania sekwencji w trybie HOLD wątek tła `mod-repaint` pozostaje w stanie spoczynku (`return 0`), eliminując 99,9% niepotrzebnych wywołań sterty JUCE.
2. **Strażnicy współbieżności RAII (`UiEventGuard`, `PaintGuard`):**
   * Zaimplementowano atomowe flagi `IS_PAINTING` oraz `IS_DISPATCHING_EVENT`.
   * Wątek `mod-repaint` natychmiast rezygnuje z wywołania (`return 0`), gdy wątek główny MPC Sample znajduje się wewnątrz metody `on_paint`, `on_button` lub `on_encoder`.
3. **Testy jednostkowe:**
   * Zaktualizowano test `repaint_component_only_targets_valid_screen_and_stops_outside_16_levels`, weryfikując ustępowanie odrysowywania pod flagami współbieżności.
   * Wszystkie 80 testów jednostkowych przechodzi pomyślnie.
