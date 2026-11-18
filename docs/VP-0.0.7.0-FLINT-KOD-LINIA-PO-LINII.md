# VP-0.0.7.0-FLINT-KOD-LINIA-PO-LINII — audyt kodu linia po linii

**Agent:** FLINT (rewersja i przepisanie kodu linia po linii), zespół Jeremiasza
**Data audytu (date -u):** `Wed Nov 18 17:34:19 UTC 2026` (odczyt weryfikacyjny), pisanie raportu `2026-11-18 ~17:4xZ`
**Repo:** `/Users/fafi/Downloads/ArpeggiGator`
**HEAD (PROVEN):** `b912eed1028c9b9c54a166b36433cd349e782be0` = czysty `v0.0.6.0`
**Stan tracked (PROVEN):** czysty — wszystkie wpisy `git status --short` to `??` (artefakty nieśledzone). Nie zmieniono ŻADNEGO pliku `src/`.
**Tagi (PROVEN):** istnieją `v0.0.6.0`, `v0.0.6.1`, `v0.0.6.2`; brak `v0.0.6.3`, brak `v0.0.7.0`.
**Przeczytano:** wszystkie 16 plików `src/*.rs` (10 105 linii wg `wc -l`) + krytyczne pliki SDK (`loader.rs`, `hook.rs`, `sites.rs`, `audio.rs`, `state.rs`, `event.rs`), `Cargo.toml`, `scripts/build.sh`, `vendor/mpc-sample-sdk/build-mod.sh`, `scripts/build-emulator-audio.sh`, `scripts/package-pad-cleanup.py`, `scripts/pads-off.sh` oraz dokumentacja źródłowa decyzji o featurze.

**Kanon (powtarzam przy każdym wniosku):**
1. TRZEBA DOPILNOWAĆ I PRZEANALIZOWAĆ CAŁY KOD LINIA PO LINII, ŻEBY NIE ROZWALIĆ FIZYCZNEJ MPC SAMPLE.
2. CAŁY KOD TRZEBY PRZEPISAĆ NA NOWO — tu używaliśmy audio emulatora (`MPC_MOD_FEATURES=emulator-audio`). Nie zakładać, że build emulatora jest buildem dla fizycznego sprzętu.
3. MUSIMY BYĆ PEWNI, ŻE PO WYJĘCIU KARTY SD Z FIZYCZNEJ MPC KONSOLA JEST W STANIE JAK PRZED WŁOŻENIEM KARTY.

---

## 0. PIĘĆ NAJWAŻNIEJSZYCH WNIOSKÓW

1. **`emulator-audio` to nazwa błędna — funkcjonalnie jest to „włącz generowanie dźwięku". PROVEN.** Wszystkie 16 bramek `#[cfg(feature = "emulator-audio")]` w `src/lib.rs` otaczają wyłącznie wywołania `native_audio::*`. Bez tego feature'u `note_interceptor()` zwraca `None` (`lib.rs:583-586`), `native_audio::install()` nie jest wołany (`lib.rs:335-336`) → mod **nie generuje ani jednej nuty**, pozostaje nakładką UI. Nie ma w kodzie ŻADNEGO wykrywania emulatora (ani `target_os`-owego, ani runtime'owego) — ta sama maszyna wykonuje się na fizycznym ARM przy zgodnym firmware.

2. **Fizyczny build MUSI mieć włączoną ścieżkę audio (feature albo jej usunięcie przez rewrite), ale NIE WOLNO go wdrożyć w obecnej formie. PROVEN.** Ta sama ścieżka to jedyny kod w repo, który robi *runtime patching* firmware'u: `native_audio::install()` (`native_audio.rs:300-313`) wstawia branch pod adres `base+0x18dbcc0` i wywołuje funkcje `kill_all_voices` / `pitch` / `dispatch` po twardych offsetach z firmware 1.3.0.12. Guard sygnatur (`native_audio.rs:268-293`) ogranicza to do jednego wersji firmware, ale nie zapewnia rollbacku.

3. **„Po wyjęciu karty jak przed" NIE DA SIĘ udowodnić z obecnego kodu. PROVEN (brak mechanizmu) / INFERRED (skutek na fizycznej MPC).** `HookEngine::abort()` celowo nie robi nic po opublikowaniu brancha (`hook.rs:364-373`), `Drop for ArpeggiGator` gasi tylko diody (`lib.rs:750-754`) — nie woła `native_audio::panic()` ani `kill_all_voices()`. Wątek `arpeggigator-tempo` (`tempo.rs:33-67`) jest odłączony, nieskończony i nie ma stopu. Nie istnieje żadna ścieżka przywrócenia patchy / głosów / wątków przy unloadzie.

4. **`probe` (g_probe) jest bezpieczny i wymagany przez build, ale `probe::capture_and_schedule()` to nieuzbrojony, niebezpieczny kod. PROVEN.** `g_probe` eksportowany przez `export_static!` (`probe.rs:104-107`), `build.sh:69` wymaga symbolu. `capture_and_schedule()` (`probe.rs:155-237`) spawnuje wątek wołający `draw::repaint_painted()` **z wątku nie-UI** — dokładnie klasa kodu, która spowodowała udokumentowany SEGV (`mod-repaint`, core `2a74ee24…`, `HANDOFF-0.0.6.0-LIVE.md:76-83`). Nie jest wywołany z `src/` (tylko `button_event` i `on_paint` są podpięte).

5. **Istnieje rozjazd między „zwalidowanym buildem" a skryptem pakującym. PROVEN.** `scripts/build.sh:63-71` NIE ustawia `MPC_MOD_FEATURES`. Build z `sh scripts/build.sh` bez env = build **bez audio**. Historyczny, zwalidowany build 0.0.6.0 wymagał `MPC_MOD_FEATURES=emulator-audio` ustawionego z zewnątrz (`HANDOFF-0.0.6.0-LIVE.md:13,65,110`; `BOARDING…:34,87`). To realna pułapka: ten sam skrypt produkuje dwie funkcjonalnie różne paczki. `--require-symbol g_arp_audio_on/off` nie wykrywa tego, bo te symbole są w binarium niezależnie od feature'u.

---

## 1. MAPA PLIK → FUNKCJE → ZALEŻNOŚCI → RYZYKO

Legenda ryzyka: **WYSOKIE** = dotyka kodu firmware'u / ALSA / wątków / pamięci procesu; **ŚREDNIE** =.UI/CD / pliki / atomiki współdzielone; **NISKIE** = czysta logika, brak efektów ubocznych.

| # | Plik (linie) | Funkcje / typy | Zależności | Ryzyko sprzętowe | `emulator-audio` |
|---|---|---|---|---|---|
| 1 | `acordator.rs` (260) | `ChordMood`, `VoicingStyle`, `AcordatorEngine` (`press/set_root/release/clear/next_step`), `StepEngine` | `crate::{Step, StepEngine}` | **NISKIE** — czysta logika, stałe tabele interwałów | NIE |
| 2 | `arp.rs` (577) | `Pattern`, `Step`, `StepEngine`, `ArpEngine` (`press/release/update_at_boundary/clear/held_count/next_step`, `bounce_*`, `converge`, `diverge`, `random_index`) | `core` tylko | **NISKIE** — brak wywołań firmware'u, wątków, I/O | NIE |
| 3 | `diag.rs` (108) | `FILE_LOCK`, 11 atomików, `set_context`, `note_on`, `tick`, `append_rotated` | `std::fs`, `std::sync::Mutex`, `SystemTime` | **ŚREDNIE** — zapis do `/tmp/arpeggigator-6-events.log` + rotacja 4×128 KiB; mutex; `eprintln!`. Brak I/O na ścieżce RT (tylko atomiki) | NIE |
| 4 | `lib.rs` (1520) | moduły 1–21, `observe_surface`, `ArpeggiGator` (`Mod`: `on_start/on_button/on_encoder/note_interceptor/surface_observer/on_pad/on_pitch/on_paint`), `reset_engine_state`, `extinguish_pads`, `Drop`, `export_mod!`, 12 testów | SDK (`Mod`, `draw`, `lighting`, `page`, `clock`, `sites`), `native_audio`, `tempo`, `probe`, `diag`, `splash`, wszystkie silniki | **WYSOKIE** — punkt wejścia .so; wywołuje `native_audio::install()`, `panic()`, wszystkie settery; modyfikuje światła padów; `repaint` tylko z wątku UI | **TAK — 16 bramek** + 2 bramki `stage-*` |
| 5 | `melody.rs` (966) | `BarLength`, `MelodyVariant`, `MelodyStep`, 36 statycznych tablic (`MEL_*`), `get_step`, `MelodyEngine`, `StepEngine` | `crate::{Step, StepEngine}` | **NISKIE** — dane + czysta logika | NIE |
| 6 | `mutator.rs` (495) | `MutationIntensity`, `Rng` (xorshift), `MutatorEngine` (`mutate_cycle`, `toggle_frozen`, …), `StepEngine` | `crate::melody`, `crate::{Step, StepEngine}` | **NISKIE** — czysta logika, brak alokacji | NIE |
| 7 | `native_audio.rs` (739) | stałe offsety, `kill_all_voices`, 25 statics/atomików, `g_arp_*`, `set_*`, `panic`, `capture` (interceptor), `install` (HookEngine), `render`, `State::{tick,output,send}`, `PadPacket` | SDK `image`, `hook`, `sites`, `midi`, `platform::clock`; `playback`, `transport_playback`, `transport_clock`, `tempo`, `diag` | **WYSOKIE — NAJWYŻSZE** — patchowanie kodu firmware'u, fałszowanie pakietów padów, `kill_all_voices`, transmute wskaźników funkcji, `static mut STATE` | Plik nie ma `cfg(feature)`; **wszystkie wywołania są bramkowane w `lib.rs`** |
| 8 | `orbitator.rs` (311) | `OrbitAtmosphere`, `OrbitDynamics`, `OrbitatorEngine`, `StepEngine` | `crate::mutator::Rng`, `crate::{Step, StepEngine}` | **NISKIE** | NIE |
| 9 | `playback.rs` (607) | `EngineMode` (`#[repr(usize)]`), `Playback` (`set_mode/press/release/tick/clear/stop…`) | silniki + `ArpScheduler` | **NISKIE** — maszyna stanów, zero I/O | NIE |
| 10 | `probe.rs` (324) | `ProbeDiagnostics` (`#[repr(C)]`), `button_event`, `encode_button_snapshot`, **`g_probe`**, `capture_and_schedule` (**nieużywane**), `source_pad_released`, `on_paint`, `current_thread_id` (syscall `svc 0`, x8=178) | SDK `draw`, `PadEvent`, atomiki, `thread`, `OnceLock` | **ŚREDNIE/WYSOKIE** — eksport symbolu OK; niepodpięty wątek `arpeggigator-repaint-cadence` woła `repaint_painted()` spoza UI (klasa crasha) | NIE (ma tylko `cfg(target_os/arch)`) |
| 11 | `scheduler.rs` (545) | `StepRate`, `ScheduledEvent`, `ActiveNote`, `ArpScheduler` (`start/stop/set_rate/tick/deadline_us`), `bpm_to_milli` | `crate::{Step, StepEngine}` | **NISKIE** | NIE |
| 12 | `splash.rs` (1629) | `draw_arp_overlay` (jedyna używana), ~24 funkcje rysowania, **`draw_boot_splash` (nieużywana)** | SDK `draw::Graphics`, `crate::EngineMode`, `splash_art::GATOR_GROUPS` | **ŚREDNIE** — rysowanie wyłącznie z `on_paint`; boot splash musi pozostać wyłączony | NIE |
| 13 | `splash_art.rs` (1321) | `ColorGroup`, `Rect16`, 24 tablice `GATOR_RECTS_*`, `GATOR_GROUPS` | tylko `splash.rs::draw_boot_splash` | **NISKIE** — dane; osiągalne tylko przez nieużywaną funkcję | NIE |
| 14 | `tempo.rs` (192) | `CURRENT_PROJECT`, `MONITOR_STARTED`, `BPM_BITS/VALID`, `start_monitor` (wątek), `current_bpm`, `read_current_project_bpm`, parser GZip/JSON | `flate2`, `serde_json`, `std::fs`, `thread`, `diag::tick` | **WYSOKIE (cykl życia)** — odłączony, nieskończony wątek; tylko odczyt `/data/TempProjects/Temp-Project.xpj` | NIE — włączone zawsze w `on_start` |
| 15 | `transport_clock.rs` (135) | `Snapshot`, `snapshot(engine, base)`, `UnwrapClock::observe` | SDK `image` (checked reads) | **ŚREDNIE** — odczyt pamięci firmware'u po twardych offsetach, ale przez bezpieczny reader i z vtable-guardem; wołane tylko ze ścieżki audio | NIE (ale osiągalne tylko z `native_audio`) |
| 16 | `transport_playback.rs` (376) | `TransportPlayback` (`tick(pulse)`, `set_*`, `stop`) | silniki, `ScheduledEvent` | **NISKIE** — czysta logika taktowana pulse | NIE |

Podsumowanie mapy: **13 z 16 plików to czysta logika lub dane** i może zostać przepisana bez ryzyka dla hardware'u. **3 pliki są newralgiczne:** `lib.rs` (punkt wejścia + bramki), `native_audio.rs` (patching firmware'u), `tempo.rs` (wątek). `diag.rs`, `probe.rs`, `transport_clock.rs` i `splash.rs` mają ryzyko pośrednie.

---

## 2. ANALIZA PLIK PO PLIKU (PROVEN/INFERRED)

### 2.1 `lib.rs` — punkt wejścia modułu

**Deklaracja modułów i bramki (1–31):**
```rust
 7: mod acordator;
 8: mod arp;
 9: mod diag;
10: mod melody;
11: mod mutator;
12: mod native_audio;
...
17: mod splash;
18: pub(crate) mod splash_art;
...
31: use core::sync::atomic::{AtomicBool, AtomicI64, AtomicU32, Ordering};
```
`native_audio` jest zadeklarowany **bez warunku** — kompiluje się w obu wariantach buildu. Różnica polega wyłącznie na tym, czy jego funkcje są wołane.

**Globale (50–64) — wszystkie atomiki, bez `Mutex`, bez `static mut`:**
```rust
50: static DASHBOARD_REFRESH_UNTIL_MS: AtomicI64 = AtomicI64::new(0);
55: static PHYSICAL_PADS: AtomicU32 = AtomicU32::new(0);
56: static SHORTCUT_PADS: AtomicU32 = AtomicU32::new(0);
57: static PREVIEW_STATE: AtomicU32 = AtomicU32::new(0);
58: static SHIFT_ACTIVE: AtomicBool = AtomicBool::new(false);
59: pub(crate) static IN_16_LEVELS_ACTIVE: AtomicBool = AtomicBool::new(false);
60: pub(crate) static BACKGROUND_PLAYBACK_ACTIVE: AtomicBool = AtomicBool::new(false);
61: static LAST_STOP_MS: AtomicI64 = AtomicI64::new(0);
62: static LAST_UI_REPAINT_MS: AtomicI64 = AtomicI64::new(0);
63: static IS_PAINTING: AtomicBool = AtomicBool::new(false);
64: static IS_DISPATCHING_EVENT: AtomicBool = AtomicBool::new(false);
```
PROVEN: guardy `PaintGuard`/`UiEventGuard` (66–90) ustawiają atomiki i zdejmują je w `Drop`. To mechanizm przeciwko re-entrant repaint (crash 08:45 wg `BOARDING…:58`, 68). INFERRED: skuteczność na fizycznym MPC nie jest udowodniona (brak twardego dowodu wg boardingowego zapisu).

**`observe_surface` (94–171) — jedyne miejsce, gdzie dane wejściowe dotykają audio (bramka 128):**
```rust
 94: // Runs on the input thread before the SDK's non-blocking module lock.
 95: // Only scalar event fields and atomics cross the callback boundary.
...
125:                     native_audio::set_active_sample_pad(pad as u8, note);
126:                     return;
127:                 }
128:                 #[cfg(feature = "emulator-audio")]
129:                 native_audio::begin_press(pad as usize);
```
`begin_press` jest **no-op** (`native_audio.rs:139-142`) — bramka pod linią 128 nie ma skutku funkcjonalnego. `set_active_sample_pad` (linia 125) jest wołana **bez warunku**, ale tylko zapisuje dwa atomiki (`native_audio.rs:51-54`). PROVEN.

**`on_start` (321–340) — instalacja hooka audio tylko z featurem:**
```rust
334:         tempo::start_monitor();
335:         #[cfg(feature = "emulator-audio")]
336:         native_audio::install();
337:         // Stage 6.0-c1: no background repaint pump. The SDK requires repaint
338:         // on the owning UI thread; a core from 21:53 shows mod-repaint
339:         // faulting after ~645 notes (docs/backups/dev-core-6c-...).
```
`tempo::start_monitor()` (wątek!) jest zawsze. `native_audio::install()` — tylko z featurem. PROVEN.

**`should_continue_in_background` / `reset_engine_state` (245–271):**
```rust
248:         let is_transport_playing = native_audio::is_transport_playing();
...
252:     fn reset_engine_state(&mut self) {
253:         self.enabled = false;
254:         self.mode = EngineMode::Off;
255:         #[cfg(feature = "emulator-audio")]
256:         {
257:             native_audio::set_mode(EngineMode::Off);
258:             native_audio::panic();
259:         }
```
`is_transport_playing()` czyta `g_arp_transport_mode` (`native_audio.rs:71-73`) — atomik zapisywany tylko z `State::tick`, więc bez feature'u zawsze `false`. Nieszkodliwe. PROVEN.

**`note_interceptor` i `surface_observer` (583–590) — decydujący fragment dla 2.3:**
```rust
583:     #[cfg(feature = "emulator-audio")]
584:     fn note_interceptor(&self) -> Option<mpc_sample_sdk::NoteInterceptor> {
585:         Some(native_audio::capture)
586:     }
587: 
588:     fn surface_observer(&self) -> Option<fn(SurfaceEvent)> {
589:         Some(observe_surface)
590:     }
```
Bez feature'u `Mod::note_interceptor()` domyślnie zwraca `None` (`event.rs:300-302`). SDK w `sites::choose_note_flow` (`sites.rs:682-691`) wtedy woła fallback `on_note` → domyślnie `NoteFlow::Forward` (`event.rs:305-307`) → nuty firmware'u idą niezmienione, a mod nie generuje żadnych. **To jest dowód, że feature = dźwięk.** PROVEN.

**`on_paint` (653–747)** — rysowanie + throttled repaint z wątku UI (740-746). Zgodne z kontraktem SDK; brak wątka tła. PROVEN.

**`Drop` i punkt wejścia (750–756) — newralgiczne:**
```rust
750: impl Drop for ArpeggiGator {
751:     fn drop(&mut self) {
752:         extinguish_pads();
753:     }
754: }
755: 
756: mpc_sample_sdk::export_mod!(ArpeggiGator);
```
`extinguish_pads()` (314–319) wysyła `send_pad_light(pad, 0, 0.0)` + `clear_pad_rgbs()`. **Nie ma tu `native_audio::panic()`, `kill_all_voices()` ani przywrócenia hooków.** PROVEN. To wprost potwierdza „znane ryzyko rezydualne" z `HANDOFF-0.0.6.0-LIVE.md:165`.

**`export_mod!` rozwija się do `.init_array` (SDK `loader.rs:228-247`):**
```rust
231:         #[used]
232:         #[cfg_attr(target_os = "linux", unsafe(link_section = ".init_array"))]
233:         static MPC_SDK_ENTRY: extern "C" fn() = {
234:             extern "C" fn start() {
...
242:                 let _ = unsafe { $crate::install(module) };
```
Konstruktor działa raz, przed wątkami aplikacji. `install()` (loader.rs:196-223) weryfikuje firmware (`firmware_ok`, 38 słów), woła `sites::install`, zapisuje interceptor/observer, `module.on_start()`, zapisuje `MODULE` do `OnceLock`. **Brak uninstall.** PROVEN.

### 2.2 `native_audio.rs` — NAJBARDZIEJ NIEBEZPIECZNY PLIK

**Twarde offsety i bezpośrednie wywołanie firmware'u (16–35):**
```rust
16: const RENDER_OFFSET: usize = 0x18dbcc0;
17: const RENDER_VTABLE: usize = 0x6e3b220;
18: const ENGINE_OFFSET: usize = 0x5c0;
19: const ENGINE_VTABLE: usize = 0x6e7f2d0;
20: const DISPATCH_OFFSET: usize = 0x211b090;
21: const PITCH_OFFSET: usize = 0x27e4400;
22: const KILL_ALL_VOICES: usize = 0x2118a20;
23: const PAD_SOURCE: u32 = 0x7fff_fffa;
...
25: unsafe fn kill_all_voices(engine: usize) {
26:     if engine == 0 { return; }
29:     let base = image::base();
31:         let kill_all =
32:             core::mem::transmute::<usize, unsafe extern "C" fn(usize)>(base + KILL_ALL_VOICES);
33:         kill_all(engine);
```
PROVEN: to jest kod specyficzny dla firmware'u 1.3.0.12 (`44507c7a…`). INFERRED: na fizycznej MPC z innym firmware'em guard z `install()` powinien odmówić (sygnatury), ale **brak testu fizycznego** w repo.

**`install()` (267–316) — runtime patching:**
```rust
267: pub fn install() {
268:     let base = image::base();
269:     let signatures = [
270:         (
271:             RENDER_OFFSET,
272:             [0xd503233f, 0xd10383ff, 0xa9077bfd, 0x9101c3fd],
...
284:     if base == 0
285:         || !signatures.into_iter().all(|(offset, words)| {
286:             words.into_iter().enumerate().all(|(i, word)|
287:                 image::read_u32(base + offset + i * 4) == Some(word))
288:         })
289:     {
290:         return;                      // <-- guard: brak patcha przy złym firmware
291:     }
...
300:     let mut hooks = match hook::HookEngine::new(
...
308:     if let Ok(original) = unsafe { hooks.prepare(RENDER_OFFSET, render as *const () as usize) } {
309:         ORIGINAL.store(original, Relaxed);
310:         if hooks.seal().is_ok() && hooks.commit().is_ok() {
311:             // The hooks and their trampolines live for the duration of the MPC process.
312:             return;
313:         }
314:     }
315:     hooks.abort();
316: }
```
PROVEN: (a) guard sygnatur jest; (b) `HookEngine` to **lokalna zmienna** — po `return` jest dropowana, ale `HookEngine` nie implementuje `Drop`, więc branch i strony `mmap` zostają w procesie; (c) komentarz 311 to **INFERRED** założenie, nie mechanizm.

**`render()` (320–343) — callback w kodzie firmware'u:**
```rust
320: unsafe extern "C" fn render(renderer: usize, block: usize, midi: usize) {
321:     let original = ORIGINAL.load(Relaxed);
...
328:         if (renderer as *const usize).read() == base + RENDER_VTABLE {
329:             let engine = ((renderer + ENGINE_OFFSET) as *const usize).read();
330:             if engine != 0
331:                 && (engine as *const usize).read() == base + ENGINE_VTABLE
332:                 && !RENDER_ACTIVE.swap(true, Acquire)
333:             {
334:                 let state =
335:                     (&mut *core::ptr::addr_of_mut!(STATE)).get_or_insert_with(State::default);
336:                 state.tick(engine);
337:                 READY.store(true, Release);
338:                 RENDER_ACTIVE.store(false, Release);
339:             }
340:         }
341:         forward(renderer, block, midi);   // <-- oryginalny audio path zawsze leci dalej
```
PROVEN: `static mut STATE` (137) jest chroniony atomikiem `RENDER_ACTIVE` (332). `forward` zawsze wołany → natywny dźwięk nie jest wyciszany. `vtable`-checki ograniczają dostęp do właściwego obiektu.

**PANIC (358–393) — flaga, nie natychmiastowe wyciszenie:**
```rust
358:         let panic_requested = PANIC.swap(false, Acquire);
359:         if panic_requested {
...
363:                 if let Some(key) = self.voice.take() {
364:                     send(engine, key, false);
...
390:                 kill_all_voices(engine);
391:             }
392:             let _ = self.playback.set_mode(EngineMode::Off);
393:         }
```
PROVEN: `panic()` (`native_audio.rs:208-212`) tylko ustawia `PANIC = true` i resetuje aktywny pad. Jeśli `render` przestanie być wołany przed konsumpcją flagi, głos zostaje. To jest dokładnie ryzyko opisane w `HANDOFF-0.0.6.0-LIVE.md:165`.

**`send()` (653–694) — fałszowanie pakietów padów:**
```rust
653: unsafe fn send(engine: usize, key: Key, on: bool) {
654:     let mut packet = PadPacket([0; 128]);
655:     let tag: u32 = 3 | ((u32::from(key.pad) & 0xf) << 12);
...
670:         let pitch = core::mem::transmute::<usize, unsafe extern "C" fn(usize, u64, u64, i32)>(
671:             PITCH.load(Relaxed),
672:         );
673:         let dispatch = core::mem::transmute::<
674:             usize,
675:             unsafe extern "C" fn(usize, usize, *mut PadPacket, u32),
676:         >(DISPATCH.load(Relaxed));
677:         pitch(
678:             packet.0.as_mut_ptr().add(0x14) as usize,
679:             0, 0, i32::from(key.pitch),
682:         );
683:         dispatch(engine, 0, &mut packet, PAD_SOURCE);
```
PROVEN: wywołania funkcji firmware'u z rzekomym źródłem `0x7fff_fffa` (izolacja od fizycznych padów). Pitch clamp do [-24,+24] w `output()` (622-624). INFERRED: brak audytu skutków ubocznych w firmware (voice matching, recorder) — boarding `BOARDING…:71` jawnie to oznacza jako otwarte.

**`g_arp_*` (75–88)** — 8 symboli `#[unsafe(no_mangle)]`; obserwowalne z zewnątrz, ale to tylko atomiki (liczniki). PROVEN.

### 2.3 `tempo.rs` — odłączony wątek

```rust
19: const CURRENT_PROJECT: &str = "/data/TempProjects/Temp-Project.xpj";
20: static MONITOR_STARTED: AtomicBool = AtomicBool::new(false);
...
28: pub fn start_monitor() {
29:     if MONITOR_STARTED.swap(true, Ordering::AcqRel) { return; }
33:     let result = thread::Builder::new()
34:         .name("arpeggigator-tempo".to_string())
35:         .spawn(|| {
...
38:             loop {
39:                 let modified = fs::metadata(CURRENT_PROJECT)
...
63:                 crate::diag::tick();
65:                 thread::sleep(Duration::from_millis(500));
66:             }
67:         });
69:     if result.is_err() { MONITOR_STARTED.store(false, Ordering::Release); }
```
PROVEN: (a) wątek jest uruchamiany zawsze (`lib.rs:334`), niezależnie od feature'u; (b) pętla nie ma warunku wyjścia; (c) nie ma `JoinHandle` ani `stop()`; (d) `MONITOR_STARTED` nie jest resetowany przy unloadzie; (e) jedyny zapis to `/tmp` przez `diag::tick()`. INFERRED: przy wydjęciu karty i wyładowaniu .so wątek może wskazywać na zwolniony kod → SIGSEGV; zależy od tego, czy ac50 faktycznie robi `dlclose` (czego kod repo nie pokazuje).

### 2.4 `probe.rs` — `g_probe` i nieuzbrojony wątek

```rust
104: mpc_sample_sdk::export_static!(
105:     /// Emulator observer diagnostics. The repr(C) layout is fixed for the script.
106:     g_probe: ProbeDiagnostics = ProbeDiagnostics::new()
107: );
```
`export_static!` (`state.rs:83-91`) daje `#[unsafe(export_name = "g_probe")]`. `build.sh:69` wymaga tego symbolu w ELF. PROVEN.

Nieuzbrojony, niebezpieczny fragment:
```rust
192:     let spawned = thread::Builder::new()
193:         .name("arpeggigator-repaint-cadence".to_string())
194:         .spawn(|| {
...
216:                     // Repaint requests only. This probe never invokes note hooks.
217:                     draw::repaint_painted();
218:                     thread::sleep(REPAINT_INTERVAL);
...
231:         });
```
Grep po całym `src/` pokazuje tylko dwa wywołania z `lib.rs`: `probe::button_event` (559) i `probe::on_paint` (659). `capture_and_schedule` / `source_pad_released` **nie są wywołane nigdzie**. PROVEN: martwy kod. INFERRED: podpięcie go w v0.0.7.0 odtworzy klasę crasha `mod-repaint`.

`current_thread_id()` (121-134) używa bezpośredniego `svc 0` (gettid, x8=178) na AArch64 Linux — tylko do logowania; wywoływany z `on_paint` i z wątka probe (gdyby był uzbrojony). PROVEN.

### 2.5 `diag.rs` — logowanie

```rust
14: const LOG_PATH: &str = "/tmp/arpeggigator-6-events.log";
15: const MAX_LOG_BYTES: u64 = 128 * 1024;
...
45: /// Called from the audio render path. Cheap atomics only.
46: pub fn note_on(pad: u8, note: u8, pitch: i8) { ... }
...
89: fn append_rotated(line: &str) {
90:     let _guard = FILE_LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
...
94:                 let _ = std::fs::remove_file(format!("{LOG_PATH}.3"));
95:                 let _ = std::fs::rename(format!("{LOG_PATH}.2"), format!("{LOG_PATH}.3"));
```
PROVEN: plik jest w `/tmp` (zniknie przy restarcie), 4 pliki × 128 KiB (~512 KiB); `Mutex` chroni rotację (dodane w 0.0.6.0 UI-fix); `note_on` na ścieżce RT tylko inkrementuje atomiki. Brak zapisu na trwałe nośniki. INFERRED: sam fakt istnienia plików w `/tmp` po wyjęciu karty (bez rebootu) nie przywraca stanu „sprzed włożenia" w sensie ścisłym — ale to nie jest zmiana systemu.

### 2.6 `transport_clock.rs` — odczyt pamięci firmware'u

```rust
21: pub(crate) fn snapshot(engine: usize, base: usize) -> Option<Snapshot> {
22:     if image::read_pointer(engine)? != base.checked_add(0x6e7f2d0)? {
23:         return None;
24:     }
...
27:     if image::read_pointer(vtable.checked_add(0x18)?)? != base.checked_add(0x1445640)?
```
PROVEN: wyłącznie odczyt przez `image::read_*` (checked), z guardami vtable; brak zapisu. Wołane tylko z `native_audio.rs:406`, czyli ze ścieżki audio. INFERRED: zależne od nieudokumentowanego layoutu firmware'u — boarding `BOARDING…:70` mówi wprost: „Brak oficjalnego API pozycji transportu — tylko read-only research offsetów; nie przenosić do paczki sprzętowej."

### 2.7 `splash.rs` / `splash_art.rs` — rysowanie, w tym martwy splash

```rust
1275: pub unsafe fn draw_boot_splash(graphics: &mut Graphics) {
1278:         graphics.colour(0xff16082c);
1279:         graphics.rect(0, 0, 320, 240);     // <-- pełny clear ekranu
...
1301:         for group in &crate::splash_art::GATOR_GROUPS {
```
PROVEN: `draw_boot_splash` **nie ma ani jednego wywołania** w `src/`; `splash_art` jest osiągalny tylko przez tę funkcję. `lib.rs` woła wyłącznie `draw_arp_overlay` (721). Handoff zakazuje splashu (`HANDOFF…:112`: „Brak splashu przy starcie"; `HANDOFF…:141` pkt 7 zakazów). INFERRED: LTO i tak może usunąć martwy kod z `.so`, ale w rewritie należy go usunąć ze źródła, żeby nie mógł wrócić.

### 2.8 Pozostałe (czysta logika)

`arp.rs`, `acordator.rs`, `melody.rs`, `mutator.rs`, `orbitator.rs`, `playback.rs`, `scheduler.rs`, `transport_playback.rs` — PROVEN: none of these contains `unsafe`, I/O, threads, atomics, `#[no_mangle]`, `cfg(feature)` or SDK calls. Wszystkie transpozycje clampowane do [-24,+24] (np. `acordator.rs:208`, `orbitator.rs:244`, `mutator.rs:377`, `melody.rs:919`). Testy jednostkowe sprawdzają te granice. Zostawić jako wzorzec do przepisania 1:1.

---

## 3. FEATURE GATES — PEŁNA MAPA I ODPOWIEDŹ NA PUNKT 2.3 HANDOFFU

### 3.1 Gdzie dokładnie występuje `#[cfg(feature = ...)]`

**`Cargo.toml:9-13` (PROVEN):**
```toml
 9: [features]
10: emulator-audio = []
11: # Staging gates for 0.0.6: expose engines one at a time until later stages.
12: stage-arp-only = []
13: stage-melody-only = []
```
Brak `default = [...]`. `MPC_MOD_FEATURES` nie jest zmienną kodu — to tylko env czytana przez `vendor/mpc-sample-sdk/build-mod.sh:123-125`:
```sh
121: echo "== cargo build ($package) =="
122: set -- --release --manifest-path "$manifest" --target "$target" -p "$package" --lib
123: if [ -n "${MPC_MOD_FEATURES:-}" ]; then
124:     set -- "$@" --features "$MPC_MOD_FEATURES"
125: fi
126: cargo build "$@"
```

**Wszystkie bramki w `src/` (PROVEN — 18 linii, wszystkie w `lib.rs`):**

| Linia `lib.rs` | Co jest bramkowane | Skutek braku feature'u |
|---|---|---|
| 128 | `native_audio::begin_press(pad)` | BRAK skutku (funkcja to no-op) |
| 183 | `stage-arp-only`: cap trybu do `Arp` | (feature staging — ma być WYŁĄCZONY) |
| 187 | `stage-melody-only`: cap trybu do `Melody` | (feature staging — ma być WYŁĄCZONY) |
| 255–259 | `set_mode(Off)` + `panic()` w `reset_engine_state` | Brak wyciszenia przy resecie |
| 335–336 | `native_audio::install()` w `on_start` | **Brak patcha render → brak nut** |
| 478–482 | `panic()` + `set_mode(mode)` przy SHIFT+B1 | Brak zmiany trybu w audio |
| 496 | `set_melody_bars` | Brak aktualizacji param w audio |
| 501 | `set_mutator_intensity` | j.w. |
| 506 | `set_chord_mood` | j.w. |
| 511 | `set_orbit_atmosphere` | j.w. |
| 515 | `set_pattern` | j.w. |
| 524 | `set_melody_variant` | j.w. |
| 529 | `set_mutator_frozen` | j.w. |
| 534 | `set_voicing_style` | j.w. |
| 539 | `set_orbit_dynamics` | j.w. |
| 543 | `set_additional_octaves` | j.w. |
| 576–577 | `set_rate` przy DATA | j.w. |
| 583–586 | `note_interceptor()` → `Some(native_audio::capture)` | **Brak interceptora → mod nie generuje nut** |

`emulator-audio`: 16 bramek. `stage-*`: 2 bramki. **Żadnej bramki feature'owej w innych plikach `src/`.** `probe.rs:15,121,136` używa tylko `cfg(target_os/target_arch)`, nie feature'ów.

### 3.2 Co jest włączone TYLKO w emulatorze, a co w buildzie fizycznym

**PROVEN — w buildzie bez `emulator-audio` nadal działa:**
- całe UI: SHIFT+B1..B3, tryby `Off/Arp/Melody/Mutator/Acordator/Orbitator`, HOLD, nakładki, LED-y (`lib.rs` poza bramkami);
- `observe_surface` i `surface_observer` (linia 588, bez warunku);
- `tempo::start_monitor()` (wątek!) i `diag::tick()`;
- `probe::button_event`, `probe::on_paint`, eksport `g_probe`;
- **hooki SDK** — `loader::install()` woła `sites::install(base)` bez warunku (`loader.rs:209`), co patchuje 18 site'ów firmware'u i vtable paint (`sites.rs:694-730`). Również ALSA-interpozycja (`audio.rs`, 4 eksporty `snd_pcm_*`) jest zawsze w `.so` przez `keep_exports()` (`loader.rs:207`, `audio.rs:839-851`).
- `native_audio.rs` jest skompilowany (wszystkie statics i `#[no_mangle] g_arp_*` są w binarium) — tylko nieużywany.

**PROVEN — tylko z `emulator-audio`:**
- `native_audio::install()` → patch `RENDER_OFFSET` + trampoliny;
- `note_interceptor` → `capture()` → `PADS[16]`, `SERIAL`, konsumpcja nut padów;
- `render()` → `State::tick()` → `send()` → `pitch()`/`dispatch()`/`kill_all_voices()` w firmware;
- `transport_clock::snapshot()` i cała ścieżka `TransportPlayback` w audio;
- `diag::note_on()` rośnie (`steps` w heartbeat) — stąd „steps=0 bez feature'u".

**WNIOSKOWANY (INFERRED, ale silnie ugruntowany) powód nazwy „emulator-audio":** feature powstał jako eksperyment audio testowany w virtual-mpc (`docs/AUDIO-EXPERIMENTS.md`, `scripts/build-emulator-audio.sh`), a nie jako detekcja emulatora. W kodzie **nie ma ani jednej linii** sprawdzającej, czy proces działa w VM. Jedynym strażnikiem jest zgodność sygnatur firmware 1.3.0.12.

### 3.3 ODPOWIEDŹ NA PYTANIE 2.3 — czy fizyczny build ma mieć `emulator-audio`?

**Krótka odpowiedź: NIE WOLNO go traktować jako opcjonalnego „dodatku emulatorowego". Fizyczny build MUSI mieć włączoną ścieżkę generowania nut — ale w rewritie należy usunąć mylącą nazwę feature'u (albo uczynić ścieżkę audio bezwarunkową), a przed wdrożeniem udowodnić cleanup przy unloadzie. Obecna paczka bez tego feature'u jest niekompletna funkcjonalnie, a obecna paczka z featurem jest nieudowodniona bezpiecznie.**

Rozumowanie wyłącznie z kodu/dokumentów:

1. **Usunięcie feature'u = zmiana produktu, nie „uspokojenie" buildu.** `lib.rs:583-586` + `event.rs:300-307` + `sites.rs:682-691` dowodzą, że bez interceptora nie ma ani jednej nuty. Handoff wymaga zachowania potwierdzonych zachowań 0.0.6.0 (`HANDOFF…:87`), a potwierdzonym zachowaniem było właśnie generowanie nut (soak: `steps` 107 → 11 000+, `HANDOFF-0.0.6.0-LIVE.md:95`). Wyrzucenie feature'u dałoby mod, który tylko rysuje aligatora. PROVEN.
2. **BACKLOG.md:43 mówi o weryfikacji „bez flag emulatorowych" — ale to oczekiwanie nie jest osiągalne przez samo wyłączenie flagi.** Osiągalne jest przez **przepisanie**: feature nazwany `native-audio` (albo brak feature'u i bezwarunkowy audio z guardem firmware) daje fizyczny build bez flagi o nazwie „emulator", przy zachowaniu dźwięku. To jest jedyna interpretacja zgodna jednocześnie z kodem i z BACKLOGIEM.
3. **`stage-arp-only` / `stage-melody-only` MUSZĄ pozostać WYŁĄCZONE** — `lib.rs:183-190` ograniczają cykl trybów; `CHANGELOG.md:17` i `HANDOFF-0.0.6.0-LIVE.md:110` potwierdzają „build bez bramek". PROVEN.
4. **Samo włączenie feature'u nie rozwiązuje kanonu nr 3.** Patchowanie jest bez rollbacku (`hook.rs:364-373`), głosy nie są zabijane w `Drop` (`lib.rs:750-754`), wątek tempo nie ma stopu (`tempo.rs:38-66`). Nawet build bez `emulator-audio` patchuje firmware przez SDK (`loader.rs:209`) — więc problem „jak przed" nie znika przez wyłączenie feature'u. PROVEN.

### 3.4 JASNA REKOMENDACJA FEATURE'ÓW DLA v0.0.7.0 (BUILD FIZYCZNY)

| Feature | Decyzja | Dlaczego |
|---|---|---|
| `emulator-audio` | **Włączyć w buildzie funkcjonalnym (albo usunąć bramkę i nazwę w rewritie)** | Jedyny przełącznik nut; bez niego mod nie arpegiuje. Nazwa to relikt testów w emulatorze, nie detekcja środowiska. |
| `stage-arp-only` | **WYŁĄCZONY** | Ogranicza `next_engine_mode` do `Arp`; nie był w zwalidowanym buildzie. |
| `stage-melody-only` | **WYŁĄCZONY** | j.w., ogranicza do `Melody`. |
| `disable-repaint-pump` | **NIE DEKLAROWAĆ** | Nie istnieje w obecnym `Cargo.toml`; pojawia się tylko w starych fingerprintach w `led-0049/source/target/…` — artefakt historii, nie feature v0.0.6.0. |

**Dodatkowy warunek proceduralny (PROVEN z `scripts/build.sh:63-71`):** obecny skrypt pakujący nie ustawia `MPC_MOD_FEATURES`. W v0.0.7.0 trzeba to naprawić w samym skrypcie (np. jawnie `--features native-audio` / bezwarunkowa ścieżka), żeby_build nie zależał od pamięci operatora o env-varze. `scripts/build-emulator-audio.sh` pozostaje narzędziem laboratoryjnym i **nie jest** przepisem na paczkę fizyczną.

---

## 4. CYKL ŻYCIA: CO MOŻE ZOSTAĆ PO WYJĘCIU KARTY

### 4.1 Patchowanie firmware'u — brak przywrócenia

SDK `hook.rs` (PROVEN):
```rust
364:     /// Drop every prepared hook and release the code pages.
365:     ///
366:     /// Does nothing once a branch was published: live branches must keep their
367:     /// trampolines mapped.
368:     pub fn abort(&mut self) {
369:         if self.used {
370:             return;
371:         }
372:         self.discard_pages();
373:     }
```
`commit()` (`hook.rs:329-362`) ustawia `used = true` po sukcesie. `patch_site()` (398-421) robi `mprotect(PROT_READ|WRITE|EXEC)` i wpisuje 4-bajtowy branch pod `site`. `map_near()` (425-447) mapuje strony relay/trampoline w `0x0790_0000..0x07f8_0000`. **`abort()` po udanym `commit()` nie przywraca ani jednego site'u.** PROVEN.

`sites::install_paint()` (694-730) dodatkowo podmienia slot vtable w `base+0x6e2_12a0 + 27*8` na `page_paint`:
```rust
714:     let unlocked = unsafe {
715:         libc::mprotect(page as *mut libc::c_void, 0x1000,
716:                        libc::PROT_READ | libc::PROT_WRITE,
717:         )
718:     } == 0;
...
725:     unsafe {
726:         *slot = page_paint as *const () as usize;
727:         libc::mprotect(page as *mut libc::c_void, 0x1000, libc::PROT_READ);
728:     }
```
PROVEN: brak zapisu oryginału z powrotem w żadnej ścieżce.

**Wniosek (PROVEN dla kodu / INFERRED dla skutku):** po udanym loadzie firmware w RAM ma zmienione co najmniej ~19 miejsc (18 site'ów SDK + 1 render + vtable paint). Zmiany znikają dopiero przy wyjściu procesu ac50 / reboocie. Jeśli po wyjęciu karty ac50 nadal działa z `LD_PRELOAD=/run/arpeggigator.so` (a autorun.sh z `build-mod.sh:228-231` nic nie sprząta), to: (a) mod pozostaje aktywny mimo braku karty; (b) ewentualne wyczyszczenie `/run` w trakcie pracy ac50 zostawi branch'e wskazujące na zwolniony kod → SIGSEGV. **Nie da się z kodu src udowodnić stanu „jak przed".**

### 4.2 Głosy audio

PROVEN: `Drop` nie woła `panic()` (`lib.rs:750-754`). `panic()` jest flagą konsumowaną dopiero w `State::tick` (`native_audio.rs:358-393`). `kill_all_voices()` jest wołany tylko z `tick()` (390, 509, 592). Jeśli render zamilknie przed konsumpcją, głos pozostaje brzmiący. Zgadza się z `HANDOFF-0.0.6.0-LIVE.md:165` i `BOARDING…:72` („Unload-time voice cleanup … odroczone").

### 4.3 Wątki

| Wątek | Gdzie | Stop? | Ryzyko |
|---|---|---|---|
| `arpeggigator-tempo` | `tempo.rs:33-67`, start z `on_start` (`lib.rs:334`) | **BRAK** (pętla `loop`, brak join/stop) | czyta `/data` co 500 ms na zawsze; kod w .so |
| `arpeggigator-repaint-cadence` | `probe.rs:192-231` | samo wygaśnięcie po ~1,25 s | **NIEUŻYWANY** — nie wolno podpiąć w rewritie (klasa SEGV `mod-repaint`) |
| wątki firmware/ALSA | SDK, `audio.rs` (taps przez `snd_pcm_*`) | poza src | interpozycja jest zawsze w `.so`; tap flagi domyślnie false (`audio.rs:13-14`) |

PROVEN: brak jakiegokolwiek `JoinHandle`, `AtomicBool` stopującego wątek tempo ani `Drop` dla niego. `MONITOR_STARTED` (`tempo.rs:20`) jest jednokierunkowy.

### 4.4 Zmienne globalne, mutexy, atomiki, hooki — pełna inwentaryzacja

- **Atomiki w `src/`:** `lib.rs` 11 (50-64, 92), `diag.rs` 11 (17-27), `native_audio.rs` ~25 (37-49, 75-88, 144-153), `probe.rs` 5 (109-114) + 20 pól w `ProbeDiagnostics` (23-42), `tempo.rs` 3 (20-22). **Bez `static mut` w `src/` poza `native_audio.rs:137` (`STATE`).**
- **`Mutex` w `src/`:** tylko `diag::FILE_LOCK` (`diag.rs:12`). W SDK: `MODULE`, `AUDIO`, `hook::ENGINE`, `seed::BORROW`, `audio` OnceLock-i — wszystkie z `try_lock` / OnceLock, bez czekania na ścieżce RT (PROVEN z `loader.rs:140-162`).
- **`static mut`:** `native_audio::STATE` (137), SDK `sites::ORIGINALS` (41), `sites::ORIGINAL_PAINT` (651), SDK `audio` (`G_OUT_DELAY` itd., `audio.rs:240-350`). PROVEN.
- **`#[no_mangle]` / `export_name`:** 8× `g_arp_*` (`native_audio.rs:75-88`), `g_probe` (`probe.rs:104-107`), 4× `snd_pcm_*` + `g_outtap*`, `g_buf_*` w SDK `audio.rs`. PROVEN.
- **Konstruktory:** `mpc_sample_sdk::export_mod!` → `.init_array` (`loader.rs:228-247`). Brak destruktora / `.fini_array`. PROVEN.
- **SysEx:** `src/` NIE wysyła SysEx. `probe.rs:3-5` wprost mówi, że nie woła `sites::send_note_on/off`. `pads-off.sh` zapisuje surowe bajty MIDI na `/dev/snd/midiC…D0` — ale to skrypt paczki, nie `src/`, i działa w `ExecStopPost` przy zatrzymaniu ac50. PROVEN.
- **Trwałe zmiany w `/etc`:** z `src/` — ŻADNE. Autorun zapisuje tylko `/run/systemd/system/ac50.service.d/*.conf` i `/run/*.so` (`build-mod.sh:228-244`, `package-pad-cleanup.py:16-18`) — tmpfs, znika przy restarcie. PROVEN dla buildu; INFERRED, że fizyczna MPC zachowuje się tak samo (brak testu fizycznego).

---

## 5. LISTA DO „PRZEPISANIA NA NOWO" vs „ZOSTAWIĆ Z v0.0.6.0"

### 5.1 Pisać od zera / gruntownie przepisać (bezpieczność sprzętu)

1. **`native_audio.rs` (całość).** Wszystkie twarde offsety, `transmute` do funkcji firmware'u, `kill_all_voices`, fałszowanie `PadPacket`, `static mut STATE`. Nowa wersja musi: (a) weryfikować hash firmware'u przed jakimkolwiek patchowaniem (nie tylko sygnatury 4 słów), (b) mieć jawny `uninstall()` przywracający site'y i zwalniający głosy, (c) nie zakładać, że „hooki żyją cały proces".
2. **Obsługa unloadu w `lib.rs` (`Drop`, 750-754).** Dodać wyciszenie głosów i zatrzymanie silnika; rozważyć `panic() + kill_all_voices()` wywołane bezpośrednio (nie przez flagę). Bez tego kanon nr 3 pozostaje nieudowodniony.
3. **`tempo.rs` — zarządzanie wątkiem.** Wprowadzić `AtomicBool` stop + `JoinHandle` (albo `thread::scope` / integrację z istniejącym wątkiem UI zamiast nowego wątku).
4. **Nazwa i bramka feature'u.** Zamiast `emulator-audio` → `native-audio` (lub ścieżka bezwarunkowa). W `(Cargo.toml, build.sh, build-mod.sh, build-emulator-audio.sh)` ustalić JEDEN kanoniczny przepis, żeby paczka zawsze miała audio i żeby nie zależało to od env operatora.
5. **Kompilacyjne usunięcie martwego kodu:** `probe::capture_and_schedule` + `draw_boot_splash` + `splash_art.rs` (albo zostawić dane, ale usunąć możliwość wywołania). Handoff zakazuje splashu i wątka repaintu.
6. **Weryfikacja firmware'u:** rozszerzyć 38-słowny `FIRMWARE_SITES` i sygnatury `native_audio` o pełny hash pierwszych 4 KiB (`44507c7a…`, zgodnie z `--firmware-sha256` w `build.sh:70`) sprawdzany w loadzie, nie tylko w skrypcie pakującym.
7. **Dokumentacja/audyt skutków ubocznych dispatchu** (voice matching, RECORD, lock ordering) — otwarte w `BOARDING…:71`.

### 5.2 Zostawić z v0.0.6.0 (sprawdzone, czyste, bezpieczne)

1. **`arp.rs`, `melody.rs`, `mutator.rs`, `acordator.rs`, `orbitator.rs`** — silniki i tablice nut. Czysta logika, testy graniczne [-24,+24]. Przepisać 1:1 (zmienić tylko stylistykę, nie zachowanie).
2. **`scheduler.rs`, `playback.rs`, `transport_playback.rs`, `transport_clock.rs`** — maszyny stanów i timers (transport_clock zachować jako *read-only* z guardami).
3. **`lib.rs`: logika UI** — `observe_surface` (bez `begin_press`), `on_button`, `on_encoder`, `on_pad`, `on_paint`, `sync_navigation`, `reset_engine_state`, throttle repaint 33 ms na wątku UI, `IS_PAINTING`/`UiEventGuard`. To jest dokładnie to, co przeszło soaki 0.0.6.0/UI-fix.
4. **`diag.rs`** — atomiki + rotacja logu z mutexem (bez I/O na RT).
5. **`probe.rs`: tylko `g_probe` + `button_event` + `on_paint`** — eksport i obserwacja; bez wątka.
6. **`splash.rs`: tylko `draw_arp_overlay` i funkcje nakładki** (bez `draw_boot_splash`).
7. **Schemat paczki:** firmware guard, `--require-symbol g_probe`, `package-pad-cleanup.py`, drop-iny wyłącznie w `/run`, `pads-off.sh` w `ExecStopPost` — zachować, ale poddać osobnemu audytowi (Krystyna), bo to poza zakresem tego raportu.

---

## 6. WERDYKT BEZPIECZEŃSTWA KODU

| Kryterium | Werdykt |
|---|---|
| Kod czystej logiki (13/16 plików) | **BEZPIECZNY** — brak efektów ubocznych, testy graniczne |
| Ścieżka UI (`lib.rs` poza bramkami, `splash` overlay, `diag`) | **BEZPIECZNA WARUNKOWO** — zgodna z kontraktem UI-thread; brak twardego dowodu na fizycznym MPC |
| Runtime patching firmware'u (`native_audio::install`, SDK `sites`) | **RYZYKO WYSOKIE** — działa tylko przy zgodnym firmware, ale bez rollbacku |
| Wyciszenie głosów przy unloadzie | **NIE ZAIMPLEMENTOWANE** — `Drop` tylko gasi diody |
| Wątek `arpeggigator-tempo` | **RYZYKO WYSOKIE** — brak stopu/join |
| `probe::capture_and_schedule` | **MARTWY, ALE NIEBEZPIECZNY** — nie wolno podpiąć |
| Możliwość udowodnienia „po wyjęciu karty jak przed" | **NIEUDOWODNIONE** — brak mechanizmu restore w kodzie |
| Build bez `emulator-audio` | **FUNKCJONALNIE NIEKOMPLETNY** (brak nut) |
| Build z `emulator-audio` na fizycznej MPC | **NIE ZATWIERDZONY** — brak testu fizycznego + brak cleanupu |

### WERDYKT KOŃCOWY

**Kod v0.0.6.0 nie nadaje się do bezpośredniego wdrożenia na fizyczną MPC Sample w wersji v0.0.7.0.** Nie dlatego, że jest złośliwy czy losowo niestabilny — soaki 30 min i 82/82 testów temu zaprzeczają — ale dlatego, że:

- zawiera bezrollbackowe patchowanie kodu firmware'u i wywołania jego funkcji po twardych offsetach,
- nie ma ścieżki unloadu, która udowodniłaby powrót do stanu sprzed włożenia karty (kanon nr 3),
- pozostawia odłączony wątek i potencjalnie brzmiący głos,
- a jego „emulatorowy" feature jest w istocie głównym przełącznikiem funkcjonalności, więc nie można go po prostu wyłączyć.

Zgodnie z `HANDOFF-0.0.7.0-PLAN.md:107` i `PROMPT…:39`: **NIE NADAJE SIĘ DO DEPLOYU** dopóki punkty 5.1(1–7) nie zostaną zaimplementowane i zweryfikowane (w tym fizyczny test guarda firmware i test wyjęcia karty). Dopiero po tym można wystawić paczkę z feature'em audio (pod nową, nie-mylącą nazwą) i z `stage-*` wyłączonymi.

**Nie zmieniono żadnego pliku `src/`. Nie dotknięto maszyn wirtualnych, ac50, /run ani fizycznej MPC.** Ten raport jest jedynym nowym plikiem.

---

## ZAŁĄCZNIK A — komendy weryfikacyjne (wyniki PROVEN)

```
$ git rev-parse HEAD
b912eed1028c9b9c54a166b36433cd349e782be0
$ git status --short | grep -v '^??'      # puste → tracked czyste
$ git tag -l | tail -3
v0.0.5.2
v0.0.6.0
v0.0.6.1
v0.0.6.2
$ date -u
Wed Nov 18 17:34:19 UTC 2026
$ wc -l src/*.rs   → 10105 linii w 16 plikach
$ grep -n 'feature = ' src/*.rs
   lib.rs: 128, 183, 187, 255, 335, 478, 496, 501, 506, 511, 515, 524,
           529, 534, 539, 543, 576, 583
$ grep -rn 'capture_and_schedule\|source_pad_released' src/
   → wyłącznie definicje w probe.rs; brak wywołań
$ grep -rn 'draw_boot_splash' src/
   → wyłącznie definicja w splash.rs:1275; brak wywołań
```

## ZAŁĄCZNIK B — pełne cytaty newralgicznych fragmentów z numerami linii

Wszystkie cytaty zostały przytoczone w §2 i §3 wprost z plików (`read` z numeracją linii). Kluczowe z nich, zebrawszy:

- `lib.rs:321-340` — `on_start`: `tempo::start_monitor()` zawsze, `native_audio::install()` pod cfg.
- `lib.rs:583-586` — `note_interceptor()` pod cfg → `Some(native_audio::capture)`.
- `lib.rs:750-756` — `Drop` gasi pady; `export_mod!`.
- `native_audio.rs:16-35` — offsety + `kill_all_voices`.
- `native_audio.rs:267-316` — `install()` (guard sygnatur, `HookEngine`, `commit`).
- `native_audio.rs:320-343` — `render()` (dostęp do `STATE`, forward do oryginału).
- `native_audio.rs:358-393` — konsumpcja `PANIC`.
- `native_audio.rs:653-694` — `send()` (transmute + `pitch` + `dispatch` + `PAD_SOURCE`).
- `hook.rs:364-373` — `abort()` nie przywraca opublikowanych branchy.
- `sites.rs:694-730` — podmiana slotu vtable paint.
- `loader.rs:196-223`, `228-247` — `install()` i `.init_array`.
- `tempo.rs:28-72` — wątek bez stopu.
- `probe.rs:104-107`, `192-231` — `g_probe` i martwy wątek repaintu.
- `diag.rs:14-15`, `89-108` — rotacja logu w `/tmp`.
- `Cargo.toml:9-13`, `scripts/build.sh:63-71`, `build-mod.sh:121-126` — feature/i jego brak w kanonicznym buildzie.
