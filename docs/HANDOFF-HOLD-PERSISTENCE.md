# ArpeggiGator — Handoff: Persystencja Ustawienia HOLD (v0.0.4.3-prep)

**Zalecany model:** Gemini 3.8 Flash (lub Gemini 3.8 Medium)  
**Zalecany poziom wysiłku:** Niski / Średni (zadanie jest ściśle zdefiniowane w logice UI i maszynie stanów)  
**Cel zadania:** Zapamiętywanie stanu `HOLD` (`ON` / `OFF`) przy przełączaniu trybów silnika (`SHIFT + B1`).

---

## 1. Kontekst i Wymagania Biznesowe

Obecnie w ArpeggiGatorze każde naciśnięcie kombinacji `SHIFT + B1` (przełączającej tryby silnika: `ARP` $\to$ `MELODY` $\to$ `MUTATOR` $\to$ `ACORDATOR` $\to$ `ORBITATOR` $\to$ `OFF` $\to$ `ARP`):
- Bezwarunkowo resetuje flagę `HOLD` do stanu `OFF`.

### Wymagane zachowanie:
1. **Domyślny stan początkowy:** Na starcie nowego projektu lub po zresetowaniu modułu `HOLD` ma być wyłączony (`OFF`).
2. **Zapamiętywanie stanu:** Jeśli użytkownik włączył `HOLD: ON` (kliknięciem pokrętła `DATA`), a następnie zmienia tryb silnika przyciskiem `SHIFT + B1`:
   - Stan `HOLD` ma pozostać `ON` w kolejnym trybie.
   - Na wyświetlaczu nowego trybu ma pojawić się zielony napis `HOLD: ON`.
   - Nowy silnik ma natychmiast działać w trybie zatrzaskiwania nut/padów.
3. **Jeśli HOLD był OFF:** Przy zmianie trybu ma pozostać `OFF`.
4. **Wyłączenie silnika (`EngineMode::Off`):** Przejście w tryb `Off` (ostatni krok w cyklu `SHIFT + B1`) powinno wyczyścić zatrzaśnięte nuty i zresetować `HOLD` do `OFF`. Przy kolejnym włączeniu (`Off` $\to$ `Arp`) silnik startuje ze stanem domyślnym (`HOLD: OFF`).

---

## 2. Analiza Kodu i Miejsca do Zmiany

### A. Główna pętla zdarzeń: `src/lib.rs`
W obsłudze przycisku `B1` przy aktywnym `SHIFT`:
```rust
// W okolicach linii 375-386:
if self.mode == EngineMode::Off {
    self.engine.clear();
    self.melody_engine.clear();
    self.mutator_engine.clear();
    self.acordator_engine.clear();
    self.orbitator_engine.clear();
    PREVIEW_STATE.store(0, Ordering::Release);
} else if self.mode == EngineMode::Arp {
    PREVIEW_STATE.store(PREVIEW_ENABLED, Ordering::Release);
} else {
    PREVIEW_STATE.store(PREVIEW_ENABLED | MODE_MELODY, Ordering::Release);
}
```
**Błąd:** Wywołania te nadpisują cały `PREVIEW_STATE`, bezpowrotnie niszcząc bit `HOLD_ENABLED`.

**Rozwiązanie:**
- Zachowaj bit `HOLD_ENABLED` z poprzedniego stanu:
  ```rust
  let hold_bit = PREVIEW_STATE.load(Ordering::Acquire) & HOLD_ENABLED;
  if self.mode == EngineMode::Off {
      // ... wyczyść silniki ...
      PREVIEW_STATE.store(0, Ordering::Release);
  } else if self.mode == EngineMode::Arp {
      PREVIEW_STATE.store(PREVIEW_ENABLED | hold_bit, Ordering::Release);
  } else {
      PREVIEW_STATE.store(PREVIEW_ENABLED | MODE_MELODY | hold_bit, Ordering::Release);
  }
  ```
- Zwróć uwagę na głosy w locie: przy zmianie trybu należy zwolnić aktualnie grające głosy (`native_audio::panic()` lub zwolnienie aktywnego głosu), aby brzmienie poprzedniego silnika nie wisiało w nieskończoność po zmianie algorytmu.

### B. Maszyna stanu odtwarzania: `src/playback.rs`
W metodzie `Playback::set_mode`:
```rust
if mode == EngineMode::Off {
    self.hold = false;
    self.clear()
} else if old_mode != EngineMode::Off {
    // Uwaga: self.clear() czyści selected pads i release'uje voice,
    // ale self.hold pozostaje nienaruszone (co jest pożądane).
    self.clear()
} else {
    None
}
```
Upewnij się, że zachowanie w `playback.rs` jest w pełni spójne z flagą w `src/lib.rs`.

### C. Warstwa wizualna: `src/splash.rs`
Wszystkie widoki (`draw_arp_view`, `draw_melody_view`, `draw_mutator_view`, `draw_acordator_view`, `draw_orbitator_view`) pobierają stan `hold: bool` z krotki `preview` i wyświetlają:
`if hold { "HOLD: ON" } else { "HOLD: OFF" }` z odpowiednim kolorem (zielony/czerwony). Nie wymagają zmian graficznych.

---

## 3. Weryfikacja i Testy

1. **Testy jednostkowe w Rust (`src/lib.rs` / `src/playback.rs`):**
   - Dodać test symulujący:
     1. Włączenie 16 Levels.
     2. Załączenie HOLD (kliknięcie `DATA`).
     3. Potwierdzenie `PREVIEW_STATE & HOLD_ENABLED != 0`.
     4. Wciśnięcie `SHIFT + B1` (przejście np. do `Melody`).
     5. Potwierdzenie, że `PREVIEW_STATE & HOLD_ENABLED != 0` nadal jest prawdziwe.
     6. Przejście przez kolejne tryby aż do `EngineMode::Off`.
     7. Potwierdzenie, że w trybie `Off` oraz po ponownym wejściu w `Arp` flaga `HOLD` została zresetowana do `OFF`.
2. **Kompilacja i testy:**
   ```bash
   export PATH="/opt/homebrew/bin:/opt/homebrew/Cellar/rustup/1.29.3/bin:$PATH"
   cargo test --offline
   ```
   Wszystkie testy (dotychczasowe 76 + nowe) muszą przechodzić na zielono.

---

## 4. Wytyczne Realizacyjne
- Pracuj na nowej gałęzi (np. `feature/hold-persistence` lub `codex/hold-persistence`).
- Po zaimplementowaniu i przejściu testów przygotuj zwięzłe podsumowanie zmian dla wątku zarządzającego.
