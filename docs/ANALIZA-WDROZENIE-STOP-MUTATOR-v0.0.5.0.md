# ANALIZA I WDROŻENIE STOP + MUTATOR — ArpeggiGator v0.0.5.0

## 0. Stan faktyczny z terminala

| Krok | Polecenie | Kod wyjścia | Wynik |
|------|-----------|-------------|-------|
| 1 | `cargo test --offline` w `v0050_stability` | `0` | 83/83 testów PASS, 0 failed (przed zmianami STOP) |
| 2 | `cargo test --offline` po zmianach STOP + MUTATOR | `0` | 83/83 testów PASS, 0 failed |
| 3 | `./vendor/mpc-sample-sdk/build-mod.sh --package arpeggigator` | `0` | Build OK: `build/arpeggigator.so` 542736 bajtów, glibc 2.30 ≤ 2.39 |
| 4 | `scp ... build/arpeggigator.so root@localhost:/run/arpeggigator.so` | `0` | Wgrane na DEV |
| 5 | `ssh ... cp /run/arpeggigator.so /media/MPCSAMPLE/autorun/arpeggigator.so && sync` | `0` | Wgrane na partycję SD autorun (przeżyje reboot) |
| 6 | `ssh ... reboot` | `0` | Fizyczny reboot DEV wykonany |
| 7 | `ssh ... systemctl show ac50 -p NRestarts,ActiveState,SubState` | `0` | `NRestarts=0`, `ActiveState=active`, `SubState=running` |
| 8 | `sha256sum build/arpeggigator.so` vs `/run/arpeggigator.so` vs `/media/MPCSAMPLE/autorun/arpeggigator.so` | `0` | Wszystkie hash: `d952c0470cd2942078e6dedccbb1f7c5a2e399698adc9625eb86f6637985727c` — zgodne |
| 9 | `git push origin fix/v0.0.5.0-ac50-repaint-stability` | `0` | Gałąź wypchnięta na GitHub (nie main) |

---

## 1. Poprawka STOP (P2) — co zmieniono w kodzie

### 1a. `src/lib.rs` — obsługa `event.code == 81` (linie 362–375 przed zmianą)

**Przed:**
```rust
if event.code == 81 && event.pressed {
    let now = clock::now_ms();
    let prev = LAST_STOP_MS.swap(now, Ordering::AcqRel);
    let is_double_stop = (now - prev) >= 0 && (now - prev) <= 650;
    if is_double_stop {
        LAST_STOP_MS.store(0, Ordering::Release);
        self.reset_engine_state();
    }
}
```

**Po:**
```rust
if event.code == 81 && event.pressed {
    let now = clock::now_ms();
    let prev = LAST_STOP_MS.swap(now, Ordering::AcqRel);
    let is_double_stop = (now - prev) >= 0 && (now - prev) <= 800;
    if is_double_stop {
        LAST_STOP_MS.store(0, Ordering::Release);
        self.reset_engine_state();
        return ButtonFlow::Consume;
    }
    // Pojedynczy STOP w trybie HOLD nie wycisza frazy — latch gra dalej.
    if PREVIEW_STATE.load(Ordering::Acquire) & HOLD_ENABLED != 0 {
        return ButtonFlow::Consume;
    }
}
```

**Zmiany:**
1. Próg podwójnego STOPu: **650 ms → 800 ms** (uwzględnia opóźnienia panelu i sieci; w sesji C03 odstęp wynosił 656–659 ms, co przy 650 ms nie kwalifikowało się jako szybka para).
2. Pojedynczy STOP z aktywnym HOLD zwraca `ButtonFlow::Consume` — firmware MPC nie wycisza frazy samowolnie, latch (zielony) gra dalej.
3. Potwierdzony podwójny STOP zwraca `ButtonFlow::Consume` po `reset_engine_state()` — nie forwarduje do firmware'u.

### 1b. `src/native_audio.rs` — podwójna ochrona PANIC

**Przed:** `static PANIC: AtomicBool = AtomicBool::new(false);` — pojedyncza flaga, która mogła zostać zgubiona między klatkami ALSA (dwa wywołania `panic()` między dwoma tickami → drugie zgłoszenie tracone przez `swap(false)`).

**Po:** `static PANIC_COUNT: AtomicU32 = AtomicU32::new(0);` — licznik, który zlicza wszystkie zgłoszenia panic; `tick()` konsumuje przez `PANIC_COUNT.swap(0, Acquire)` i sprawdza `> 0`.

- `panic()`: `PANIC.store(true, Release)` → `PANIC_COUNT.fetch_add(1, Release)`
- `tick()`: `let panic_requested = PANIC.swap(false, Acquire);` → `let panic_count = PANIC_COUNT.swap(0, Acquire);` + `if panic_count > 0`
- Linia 417: `!panic_requested` → `panic_count == 0`

---

## 2. Poprawka MUTATOR (U02) — SHIFT+B3 powinien toggle'ować `frozen`, nie cycle'ować wariantów

### 2a. Błąd w `src/lib.rs` (linia 540–544 przed zmianą)

**Przed:**
```rust
} else if self.mode == EngineMode::Mutator {
    self.mutator_variant = self.mutator_variant.cycle();
    self.mutator_engine.set_variant(self.mutator_variant);
    #[cfg(feature = "emulator-audio")]
    native_audio::set_mutator_variant(self.mutator_variant);
}
```

SHIFT+B3 w trybie Mutator cyklował `mutator_variant` przez MelodyVariant (MEL1→MEL2→…→MEL9→MEL1) zamiast wywoływać `set_frozen()`/`toggle_frozen()`. Użytkownik zgłaszał (U02), że oczekiwał zamrożenia mutacji (freeze), a otrzymał zmianę wariantu melodycznego.

**Po:**
```rust
} else if self.mode == EngineMode::Mutator {
    self.mutator_frozen = self.mutator_engine.toggle_frozen();
    #[cfg(feature = "emulator-audio")]
    native_audio::set_mutator_frozen(self.mutator_frozen);
}
```

### 2b. Zmiany w strukturze `ArpeggiGator`

Dodano pole `mutator_frozen: bool` do structu (inicjalizowane na `false` przez `#[derive(Default)]`).

### 2c. `src/native_audio.rs`

Dodano:
```rust
static MUTATOR_FROZEN: AtomicBool = AtomicBool::new(false);

pub fn set_mutator_frozen(frozen: bool) {
    MUTATOR_FROZEN.store(frozen, Release);
}
```

`MutatorEngine` już posiadał metody `set_frozen()`, `toggle_frozen()`, `is_frozen()` oraz `mutate_cycle()` sprawdzające `if self.frozen { return; }` — błąd polegał na tym, że `on_button` ich nie wywoływał dla B3.

### 2d. Zaktualizowany test

Test `animation_input_tests` (dawniej sprawdzający cyklowanie wariantów Mel1→Mel2) zaktualizowany do weryfikacji `is_frozen()`:
```rust
// In Mutator: SHIFT+B3 toggles frozen (not cycle variants)
assert!(!module.mutator_engine.is_frozen());
module.on_button(button_ev(BUTTON_B3, true, true));
module.on_button(button_ev(BUTTON_B3, false, true));
assert!(module.mutator_engine.is_frozen());
```

---

## 3. Analiza P4 — wzorzec BOUNCING w cyklu ARP (B2)

### 3a. Czy BOUNCING jest dostępny w cyklu wzorców ARP?

**TAK.** W `src/arp.rs` (`Pattern::ALL`, linie 30–39):

```rust
pub const ALL: [Self; 8] = [
    Self::Up,
    Self::Down,
    Self::UpDown,      // ← BOUNCING (w górę i w dół bez powtórzeń endpointów)
    Self::DownUp,      // ← BOUNCING (w dół i w górę bez powtórzeń endpointów)
    Self::Converge,
    Self::Diverge,
    Self::Random,
    Self::Played,
];
```

W `src/lib.rs`, obsługa B2 (`bit == 2`, linie 506–533) w trybie ARP (gałąź `else`) wywołuje `self.engine.cycle_pattern()`, który iteruje przez `Pattern::next()` — pełny cykl 8 wzorców, w tym `UpDown` i `DownUp` (bouncing).

**Wniosek:** Wzorzec BOUNCING (`UpDown`, `DownUp`) jest już dostępny w cyklu B2 dla ARP. Nie wymaga dodatkowych zmian w kodzie.

---

## 4. Wdrożenie na DEV — potwierdzenie

| Element | Wartość |
|---------|---------|
| Build | `build/arpeggigator.so`, 542736 bajtów, release, aarch64-unknown-linux-gnu |
| glibc | 2.30 (urządzenie ma 2.39 — kompatybilny) |
| Lokalny hash | `d952c0470cd2942078e6dedccbb1f7c5a2e399698adc9625eb86f6637985727c` |
| DEV `/run/arpeggigator.so` | Ten sam hash — zgodny |
| DEV `/media/MPCSAMPLE/autorun/arpeggigator.so` | Ten sam hash — zgodny (przeżyje reboot) |
| `sync` | Wykonane |
| `reboot` | Wykonane |
| `ac50` po reboot | `NRestarts=0`, `active/running`, `MainPID` nowy |
| Testy jednostkowe | 83/83 PASS, exit 0 |

---

## 5. Git

- Gałąź: `fix/v0.0.5.0-ac50-repaint-stability`
- Commit: `fix(transport): resolve single STOP muting in HOLD and harden panic dispatch (v0.0.5.0)`
- Push: `origin fix/v0.0.5.0-ac50-repaint-stability` (NA GITHUB, NIE NA MAIN)
- Plik raportu: `docs/ANALIZA-WDROZENIE-STOP-MUTATOR-v0.0.5.0.md` (ten plik)

---

## 6. Podsumowanie zmian

| Plik | Zmiana |
|------|--------|
| `src/lib.rs` | Próg STOP 650→800 ms; `Consume` dla pojedynczego STOPu w HOLD; `Consume` po double STOP; pole `mutator_frozen`; B3 w Mutatorze toggle'uje `frozen` zamiast cycle'ować warianty |
| `src/native_audio.rs` | `PANIC_COUNT: AtomicU32` zamiast `PANIC: AtomicBool`; `set_mutator_frozen()`; statyk `MUTATOR_FROZEN` |
| `docs/ANALIZA-WDROZENIE-STOP-MUTATOR-v0.0.5.0.md` | Ten raport |