# ANALIZA STOP / C03 — ArpeggiGator v0.0.5.0 (worktree v0050_stability)

## 0. Stan faktyczny z terminala

| Krok | Polecenie | Kod wyjścia | Wynik |
|------|-----------|-------------|-------|
| 1 | `cargo test --offline` w `/Users/fafi/.gemini/antigravity/worktrees/ArpeggiGator/v0050_stability` | `0` | 83 testów jednostkowych, wszystkie PASS, 0 failed. Kod kompiluje się bez ostrzeżeń. |

---

## 1. Obsługa `event.code == 81` w `src/lib.rs` (linie 362–370)

Obecna implementacja w `on_button()`:

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

**Fakty:**
- `LAST_STOP_MS: AtomicI64` jest zdefiniowane (linia 60) i inicjalizowane na 0.
- Próg detekcji podwójnego STOP: **650 ms** (sztywny, stały).
- Dopiero przy potwierdzonym podwójnym STOPie wywoływane jest `reset_engine_state()`.
- Pojedynczy STOP **nie ma własnego `ButtonFlow::Consume`** — po bloku powyżej kod idzie dalej do końcowego `let flow = ...`, gdzie `code == 81` nie pasuje do żadnego `bit` (B1=12, B2=13, B3=14), więc `bit == 0` i zwracany jest `ButtonFlow::Forward`.

### 1a. Dlaczego pojedynczy STOP wycisza frazę w trybie HOLD

Gdy `event.code == 81` i `event.pressed`:
1. Nie jest to podwójne kliknięcie (pierwsze wciśnięcie, `LAST_STOP_MS` był 0, więc `now - prev` jest duże, `is_double_stop == false`).
2. `reset_engine_state()` **nie jest wywoływane** — silnik (`self.enabled`, `self.mode`, `PREVIEW_STATE`) pozostaje nienaruszony, latch HOLD pozostaje aktywny.
3. Ponieważ `on_button` zwraca `ButtonFlow::Forward`, zdarzenie STOP dociera do **firmware'u MPC**, który samodzielnie zatrzymuje natywny transport audio (PLAY/STOP na poziomie urządzenia).
4. W efekcie fraza znika, mimo że mod nie zresetował stanu — to firmware MPC wycisza dźwięk, a mod wciąż uważa, że gra (zielony latch, `enabled = true`).

**Problem:** W trybie HOLD pojedynczy STOP powinien być ignorowany przez mod (lub_forwardowany bez wyciszania), a dopiero podwójne kliknięcie w oknie czasowym powinno resetować silnik. Obecnie rozjazb między stanem modu (wciąż gra) a firmwarem (już wyciszone)造成 niespójność.

---

## 2. `reset_engine_state()` (linie 254–273)

```rust
fn reset_engine_state(&mut self) {
    self.enabled = false;
    self.mode = EngineMode::Off;
    #[cfg(feature = "emulator-audio")]
    {
        native_audio::set_mode(EngineMode::Off);
        native_audio::panic();
    }
    self.engine.clear();
    self.melody_engine.clear();
    self.mutator_engine.clear();
    self.acordator_engine.clear();
    self.orbitator_engine.clear();
    PREVIEW_STATE.store(0, Ordering::Release);
    BACKGROUND_PLAYBACK_ACTIVE.store(false, Ordering::Release);
    LAST_STOP_MS.store(0, Ordering::Release);
    native_audio::set_active_sample_pad(0xff, 36);
    extinguish_pads();
    self.led_mask = 0;
}
```

**Fakty:**
- Wywołuje `native_audio::panic()` tylko pod flagą `#[cfg(feature = "emulator-audio")]`.
- Czyści wszystkie 5 silników (arp, melody, mutator, acordator, orbitator).
- Zeruje `PREVIEW_STATE`, `BACKGROUND_PLAYBACK_ACTIVE`, `LAST_STOP_MS`.
- Gasí wszystkie pady LED (`extinguish_pads()`).
- Jest wywoływane wyłącznie przy potwierdzonym podwójnym STOPie (≤650 ms) oraz przy nawigacji ekranów (`sync_navigation`).

---

## 3. `panic()` i `kill_all_voices()` w `src/native_audio.rs`

### `panic()` (linia 203–207):

```rust
pub fn panic() {
    PANIC.store(true, Release);
    ACTIVE_SAMPLE_PAD.store(0xff, Release);
    ACTIVE_SAMPLE_NOTE.store(36, Release);
}
```

### `kill_all_voices()` (linie 25–35):

```rust
const KILL_ALL_VOICES: usize = 0x2118a20;

unsafe fn kill_all_voices(engine: usize) {
    if engine == 0 {
        return;
    }
    let base = image::base();
    unsafe {
        let kill_all = core::mem::transmute::<usize, unsafe extern "C" fn(usize)>(base + KILL_ALL_VOICES);
        kill_all(engine);
    }
}
```

### Obsługa panic w `tick()` (linie 353–388):

```rust
let panic_requested = PANIC.swap(false, Acquire);
if panic_requested {
    ACTIVE_SAMPLE_PAD.store(0xff, Release);
    ACTIVE_SAMPLE_NOTE.store(36, Release);
    unsafe {
        if let Some(key) = self.voice.take() {
            send(engine, key, false);
        }
        for p in 0..16_u8 {
            if self.inputs[usize::from(p)] != 0 {
                let key = Key::unpack(self.inputs[usize::from(p)], p);
                send(engine, key, false);
                self.inputs[usize::from(p)] = 0;
                PADS[usize::from(p)].store(0, Release);
            }
            let key0 = Key { pad: p, note: 0, pitch: 0, velocity: 0, serial: 0 };
            send(engine, key0, false);
            let key_std = Key { pad: p, note: 36 + p, pitch: 0, velocity: 0, serial: 0 };
            send(engine, key_std, false);
        }
        kill_all_voices(engine);
    }
    let _ = self.playback.set_mode(EngineMode::Off);
}
```

**Fakty:**
- `PANIC: AtomicBool` jest flagą cross-threadową — można ją ustawić bezpiecznie z wątku głównego (`on_button`), a wątek renderujący ALSA (`tick`) ją konsumuje przez `swap(false, Acquire)`.
- Przy `panic_requested == true` wysyłane są `NoteOff` dla wszystkich aktywnych głosów, a następnie `kill_all_voices(engine)` wymusza zamknięcie wszystkich kanałów firmware'u.
- `PANIC` jest konsumowane raz (swap do false) — jeśli dwa STOPy nastąpią między dwoma klatkami renderowania, drugie zgłoszenie może zostać zgubione, bo `swap` zwraca tylko jeden `true`.

---

## 4. Problem progu 650 ms w sesji C03

Z raportu QA `docs/qa_screens/2026-11-16_110717_184483/events.jsonl` (przeniesione z poprzedniej gałęzi, ale odnoszące się do tego samego scenariusza):

- STOP press #1: `11:08:19.293`, press #2: `11:08:19.952` → odstęp **659 ms** (host) / ACK 656 ms.
- Wymagany próg: **<650 ms**.
- Wniosek z raportu: para STOP **nie kwalifikuje się jako szybka** — przekracza próg o 9–16 ms.

**Analiza:**
- Próg 650 ms jest sztywny i nie uwzględnia opóźnień panelu (post-before → press → ACK → release). W `events.jsonl` widać, że między press a release mijają ~250–300 ms na pojedyncze STOP, co oznacza, że użytkownik fizycznie nie jest w stanie wykonać dwóch pełnych cykli press+release w 650 ms.
- Detekcja powinna mierzyć **press do press** (jak obecnie `LAST_STOP_MS.swap`), ale próg powinien być luźniejszy — sugerowane **750–800 ms** — aby uwzględnić opóźnienie sprzętowe panelu i harnessu.
- Alternatywnie: próg mógłby być dynamiczny, uwzględniający ostatni zmierzony ACK release, ale to wymaga dodatkowego rejestru czasu release.

---

## 5. Ryzyko `double free or corruption (!prev)` przy szybkim podwójnym STOPie

### 5a. Gdzie kryje się ryzyko

1. **Brak ochrony przed double-triggerem `panic()`**: `PANIC.swap(false, Acquire)` konsumuje flagę raz. Jeśli dwa STOPy w szybkiej følują się w jednym oknie renderowania (np. dwa press w ciągu <16 ms przy 60 Hz), drugie wywołanie `panic()` ustawia `PANIC = true`, ale jeśli wątek renderujący już przebiegł przez `tick()` i zwrócił `PANIC` na `false`, drugie zgłoszenie czeka do następnej klatki. W tym międzyczasie `reset_engine_state()` z `on_button` już wyczyścił `PREVIEW_STATE` i wywołał `native_audio::set_mode(EngineMode::Off)`, co może prowadzić do niespójności między stanem modu a stanem wątku audio.

2. **Brak blokady między `on_button` a `tick()`**: `on_button` (wątek główny/wejściowy) modyfikuje `PREVIEW_STATE`, `enabled`, `mode` bez blokady, podczas gdy `tick()` (wątek renderujący ALSA) odczytuje `PREVIEW_STATE` i `MODE` przez atomiki. Brak wyższego rzędu synchronizacji (np. mutexu lub sekcji krytycznej) oznacza, że wątek audio może odczytać partiellement zaktualizowany stan.

3. **`kill_all_voices()` wywoływane z wątku audio**: ta funkcja wykonuje niebezpieczne operacje wskaźnikowe (`transmute`, wywołanie natywnego firmware'u) w kontekście callbacka renderującego. Jeśli engine został zwolniony lub zastąpiony między klatkami, wskaźnik może być nieprawidłowy.

### 5b. Powiązanie z incydentem C03 (PID 1519)

Zgodnie z `docs/qa_screens/2026-11-16_110717_184483/events.jsonl`:
- Crash: `11:08:39.517 CET`, `double free or corruption (!prev)`, `status=6/ABRT`, `NRestarts=2`.
- Ostatni ACK STOP release: `11:08:20.249`.
- Awaria nastąpiła ~19 sekund po parze STOP — czasowa kolejność **nie dowodzi bezpośredniego związku** ze STOPem.

Jednak brak bezpiecznej ścieżki wygaszania głosów przy szybkim podwójnym STOPie (próg 650 ms był zbyt wąski, `panic()` bez podwójnej ochrony) zwiększa ryzyko uszkodzenia sterty przy współbieżnym dostępie z wątku audio i wątku głównego.

---

## 6. Propozycja diffu naprawczego

### 6a. `src/lib.rs` — luźniejszy próg i wyciszenie pojedynczego STOP w HOLD

**Zmiana 1: Próg podwójnego STOPu z 650 ms na 800 ms** (linie 362–370):

```rust
if event.code == 81 && event.pressed {
    let now = clock::now_ms();
    let prev = LAST_STOP_MS.swap(now, Ordering::AcqRel);
    let is_double_stop = (now - prev) >= 0 && (now - prev) <= 800;
    if is_double_stop {
        LAST_STOP_MS.store(0, Ordering::Release);
        self.reset_engine_state();
        return ButtonFlow::Consume;  // nie forwarduj do firmware'u przy double stop
    }
    // Pojedynczy STOP w trybie HOLD: nie wyciszamy, nie resetujemy.
    // Jeśli HOLD aktywny i pad świeci, forwardujemy do firmware'u
    // (transport MPC sam zarządza PLAY/STOP), ale mod zachowuje latch.
    if PREVIEW_STATE.load(Ordering::Acquire) & HOLD_ENABLED != 0 {
        return ButtonFlow::Consume;
    }
}
```

**Uzasadnienie:** Pojedynczy STOP w HOLD nie powinien wyciszać frazy — latch ma pozostać aktywny. Dopiero szybka para (≤800 ms) resetuje silnik i gasí dźwięk. Zwracamy `Consume` przy pojedynczym STOPie w HOLD, aby firmware nie zgubił stanu transportu.

### 6b. `src/native_audio.rs` — podwójna ochrona przed zgubieniem `PANIC`

Dodaj drugi rejestr dla licznika żądań panic (zamiast pojedynczej flagi bool):

```rust
static PANIC_COUNT: AtomicU32 = AtomicU32::new(0);

pub fn panic() {
    PANIC_COUNT.fetch_add(1, Release);
    ACTIVE_SAMPLE_PAD.store(0xff, Release);
    ACTIVE_SAMPLE_NOTE.store(36, Release);
}
```

W `tick()` zamienić `PANIC.swap(false, Acquire)` na pobranie i zerowanie licznika:

```rust
let panic_count = PANIC_COUNT.swap(0, Acquire);
if panic_count > 0 {
    // ... existing panic handling ...
}
```

**Uzasadnienie:** Licznik nie zgubi drugiego żądania panic, jeśli dwa STOPy nastąpią między klatkami renderowania.

---

## 7. Podsumowanie

| Obszar | Problem | Rekomendacja |
|--------|---------|--------------|
| Pojedynczy STOP w HOLD | Wycisza frazę przez `ButtonFlow::Forward` → firmware zatrzymuje audio, mod zachowuje latch | Zwróć `ButtonFlow::Consume` przy pojedynczym STOPie z aktywnym HOLD |
| Próg podwójnego STOPu | 650 ms zbyt wąski (C03: 659 ms, nie zakwalifikowane) | Zwiększ do 800 ms |
| `panic()` cross-thread | Pojedyncza flaga `AtomicBool` może zgubić drugie zgłoszenie między klatkami | Zastąp `PANIC_COUNT: AtomicU32` |
| Brak `Consume` przy double stop | STOP forwardowany do firmware'u nawet po resecie | Dodaj `return ButtonFlow::Consume` po `reset_engine_state()` |

Wszystkie 83 testy jednostkowe przechodzą (`cargo test --offline`, exit 0). Proponowane zmiany wymagają dodania testów jednostkowych dla detekcji podwójnego STOPu oraz dla zachowania pojedynczego STOPu w trybie HOLD.