//! ArpeggiGator — standalone arpeggiator mod for AKAI MPC Sample 1.3.0.12.
//!
//! The GUI prototype keeps native Sample and 16 Levels screens intact, replacing
//! softkey captions only while SHIFT is held. The opt-in emulator build
//! runs generated pad retriggers on the native audio thread.

mod acordator;
mod arp;
mod melody;
mod mutator;
mod native_audio;
mod orbitator;
mod playback;
mod probe;
mod scheduler;
mod splash;
pub(crate) mod splash_art;
mod tempo;
mod transport_clock;
mod transport_playback;

pub use acordator::{AcordatorEngine, ChordMood, VoicingStyle};
pub use arp::{ArpEngine, Pattern, Step, StepEngine};
pub use melody::{BarLength, MelodyEngine, MelodyVariant};
pub use mutator::{MutatorEngine, MutationIntensity};
pub use orbitator::{OrbitAtmosphere, OrbitDynamics, OrbitatorEngine};
pub use playback::{EngineMode, Playback};
pub use scheduler::{ArpScheduler, ScheduledEvent, StepRate};

use core::sync::atomic::{AtomicBool, AtomicI64, AtomicU32, Ordering};

use mpc_sample_sdk::{
    ButtonEvent, ButtonFlow, EncoderEvent, EncoderFlow, Mod, PadAction, PadEvent, PadFlow,
    PaintEvent, SurfaceEvent, draw,
    event::button,
    platform::{clock, lighting, page},
};

/// Confirmed control-surface values: B1=12, B2=13, B3=14.
const BUTTON_B1: u32 = 12;
const BUTTON_B2: u32 = 13;
const BUTTON_B3: u32 = 14;
const BUTTON_PAD_FX: u32 = 1;
const BUTTON_SAMPLE: u32 = 6;
const BUTTON_SEQ: u32 = 7;
const BUTTON_SAMPLE_SELECT: u32 = 36;
const BUTTON_MUTE: u32 = 43;
const BUTTON_SAMPLE_RECORD: u32 = 73;
static DASHBOARD_REFRESH_UNTIL_MS: AtomicI64 = AtomicI64::new(0);
const PREVIEW_ENABLED: u32 = 1 << 16;
const PAD_MASK: u32 = PREVIEW_ENABLED - 1;
const HOLD_ENABLED: u32 = 1 << 17;
const MODE_MELODY: u32 = 1 << 18;
static PHYSICAL_PADS: AtomicU32 = AtomicU32::new(0);
static SHORTCUT_PADS: AtomicU32 = AtomicU32::new(0);
static PREVIEW_STATE: AtomicU32 = AtomicU32::new(0);
static SHIFT_ACTIVE: AtomicBool = AtomicBool::new(false);
pub(crate) static IN_16_LEVELS_ACTIVE: AtomicBool = AtomicBool::new(false);
pub(crate) static BACKGROUND_PLAYBACK_ACTIVE: AtomicBool = AtomicBool::new(false);
static LAST_STOP_MS: AtomicI64 = AtomicI64::new(0);
static SPLASH_ACTIVE: AtomicBool = AtomicBool::new(true);
static IS_PAINTING: AtomicBool = AtomicBool::new(false);
static IS_DISPATCHING_EVENT: AtomicBool = AtomicBool::new(false);

struct PaintGuard;
impl PaintGuard {
    fn enter() -> Self {
        IS_PAINTING.store(true, Ordering::Release);
        Self
    }
}
impl Drop for PaintGuard {
    fn drop(&mut self) {
        IS_PAINTING.store(false, Ordering::Release);
    }
}

struct UiEventGuard;
impl UiEventGuard {
    fn enter() -> Self {
        IS_DISPATCHING_EVENT.store(true, Ordering::Release);
        Self
    }
}
impl Drop for UiEventGuard {
    fn drop(&mut self) {
        IS_DISPATCHING_EVENT.store(false, Ordering::Release);
    }
}

fn repaint_component() -> usize {
    let screen = page::screen();
    if !page::is_sample_page(screen) {
        return 0;
    }
    if SPLASH_ACTIVE.load(Ordering::Acquire) {
        return screen;
    }
    if !IN_16_LEVELS_ACTIVE.load(Ordering::Acquire) {
        return 0;
    }
    if IS_PAINTING.load(Ordering::Acquire) || IS_DISPATCHING_EVENT.load(Ordering::Acquire) {
        return 0;
    }
    let now = clock::now_ms();
    if now < DASHBOARD_REFRESH_UNTIL_MS.load(Ordering::Acquire)
        || PREVIEW_STATE.load(Ordering::Acquire) & PAD_MASK != 0
    {
        screen
    } else {
        0
    }
}

// Runs on the input thread before the SDK's non-blocking module lock.
// Only scalar event fields and atomics cross the callback boundary.
fn observe_surface(event: SurfaceEvent) {
    // The physical pads use MIDI channel 10; ordinary buttons use channel 1.
    if event.status & 0x0f != 9 {
        return;
    }
    let Ok(pad) = u32::try_from(event.code) else {
        return;
    };
    if pad >= 16 {
        return;
    }
    let bit = 1_u32 << pad;
    match (event.kind, event.velocity) {
        (0x90, 1..) => {
            let was_down = PHYSICAL_PADS.fetch_or(bit, Ordering::AcqRel) & bit != 0;
            if !was_down {
                if SHIFT_ACTIVE.load(Ordering::Acquire) {
                    SHORTCUT_PADS.fetch_or(bit, Ordering::AcqRel);
                    return;
                }
                let is_background = BACKGROUND_PLAYBACK_ACTIVE.load(Ordering::Acquire)
                    && !IN_16_LEVELS_ACTIVE.load(Ordering::Acquire)
                    && (PREVIEW_STATE.load(Ordering::Acquire) & PREVIEW_ENABLED != 0);
                if is_background {
                    let note = if event.note != 0 {
                        event.note
                    } else {
                        36 + pad as u8
                    };
                    native_audio::set_active_sample_pad(pad as u8, note);
                    return;
                }
                #[cfg(feature = "emulator-audio")]
                native_audio::begin_press(pad as usize);
                let _ = PREVIEW_STATE.try_update(Ordering::AcqRel, Ordering::Acquire, |state| {
                    (state & PREVIEW_ENABLED != 0).then_some(if state & MODE_MELODY != 0 {
                        if state & HOLD_ENABLED != 0 {
                            if state & PAD_MASK == bit {
                                state & !bit
                            } else {
                                (state & !PAD_MASK) | bit
                            }
                        } else {
                            (state & !PAD_MASK) | bit
                        }
                    } else if state & HOLD_ENABLED != 0 {
                        state ^ bit
                    } else {
                        state | bit
                    })
                });
            }
        }
        (0x80, _) | (0x90, 0) => {
            PHYSICAL_PADS.fetch_and(!bit, Ordering::AcqRel);
            if SHORTCUT_PADS.fetch_and(!bit, Ordering::AcqRel) & bit != 0 {
                return;
            }
            if BACKGROUND_PLAYBACK_ACTIVE.load(Ordering::Acquire)
                && !IN_16_LEVELS_ACTIVE.load(Ordering::Acquire)
            {
                return;
            }
            let _ = PREVIEW_STATE.try_update(Ordering::AcqRel, Ordering::Acquire, |state| {
                (state & PREVIEW_ENABLED != 0).then_some(if state & HOLD_ENABLED != 0 {
                    state
                } else {
                    state & !bit
                })
            });
        }
        _ => return,
    }
    // Also repaint after the last latched pad is toggled off.
    DASHBOARD_REFRESH_UNTIL_MS.store(clock::now_ms() + 400, Ordering::Release);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
enum Screen {
    #[default]
    Sample,
    Other,
}

#[derive(Default)]
struct ArpeggiGator {
    engine: ArpEngine,
    melody_engine: MelodyEngine,
    mutator_engine: MutatorEngine,
    acordator_engine: AcordatorEngine,
    chord_mood: ChordMood,
    voicing_style: VoicingStyle,
    orbitator_engine: OrbitatorEngine,
    orbit_atmosphere: OrbitAtmosphere,
    orbit_dynamics: OrbitDynamics,
    scheduler: ArpScheduler,
    mode: EngineMode,
    melody_bars: BarLength,
    melody_variant: MelodyVariant,
    mutator_intensity: MutationIntensity,
    mutator_variant: MelodyVariant,
    enabled: bool,
    current_screen: Screen,
    in_16_levels: bool,
    additional_notes: u8,
    consumed_buttons: u8,
    consumed_data: bool,
    led_mask: u32,
    loop_audition_active: bool,
    splash_until_ms: i64,
}

impl ArpeggiGator {
    fn is_16_levels_active(&self) -> bool {
        if self.current_screen != Screen::Sample {
            return false;
        }
        if lighting::has_native_lighting() {
            self.in_16_levels && lighting::is_16_levels_lit()
        } else {
            self.in_16_levels
        }
    }

    fn should_continue_in_background(&self) -> bool {
        let state = PREVIEW_STATE.load(Ordering::Acquire);
        let is_holding = (state & HOLD_ENABLED != 0) && (state & PAD_MASK != 0);
        let is_transport_playing = native_audio::is_transport_playing();
        self.mode != EngineMode::Off && self.enabled && (is_holding || is_transport_playing)
    }

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
        lighting::restore_pad_rgbs();
        self.led_mask = 0;
    }

    fn sync_navigation(&mut self, sample_page: bool, chop_mode: bool) {
        if sample_page {
            if self.current_screen != Screen::Sample {
                self.current_screen = Screen::Sample;
                self.in_16_levels = false;
                self.reset_engine_state();
            }
            if chop_mode {
                self.in_16_levels = false;
                self.reset_engine_state();
            }
            if lighting::has_native_lighting() {
                let native_16_levels = lighting::is_16_levels_lit();
                if !native_16_levels && self.in_16_levels {
                    self.in_16_levels = false;
                    if self.should_continue_in_background() {
                        BACKGROUND_PLAYBACK_ACTIVE.store(true, Ordering::Release);
                    } else {
                        self.reset_engine_state();
                        BACKGROUND_PLAYBACK_ACTIVE.store(false, Ordering::Release);
                    }
                }
            }
        } else {
            self.current_screen = Screen::Other;
            self.in_16_levels = false;
            self.reset_engine_state();
        }
        IN_16_LEVELS_ACTIVE.store(self.is_16_levels_active(), Ordering::Release);
    }
}

impl Mod for ArpeggiGator {
    fn on_start(&mut self) {
        self.current_screen = Screen::Sample;
        self.in_16_levels = false;
        self.led_mask = 0;
        self.loop_audition_active = false;
        IN_16_LEVELS_ACTIVE.store(false, Ordering::Release);
        SPLASH_ACTIVE.store(true, Ordering::Release);
        DASHBOARD_REFRESH_UNTIL_MS.store(0, Ordering::Release);
        PREVIEW_STATE.store(0, Ordering::Release);
        PHYSICAL_PADS.store(0, Ordering::Release);
        SHORTCUT_PADS.store(0, Ordering::Release);
        SHIFT_ACTIVE.store(false, Ordering::Release);
        tempo::start_monitor();
        #[cfg(feature = "emulator-audio")]
        native_audio::install();
        // SAFETY: repaint_component returns only the live component last
        // recorded by the SDK paint hook, or zero when none is available.
        unsafe { draw::start_repaint_pump(repaint_component) };
    }

    fn on_button(&mut self, event: ButtonEvent) -> ButtonFlow {
        let _guard = UiEventGuard::enter();
        let mut action = 0;
        if event.code == 74 {
            if self.is_16_levels_active() {
                // In 16 Levels, LOOP button must not trigger background loop audition.
                return ButtonFlow::Consume;
            }
            if event.pressed {
                self.loop_audition_active = !self.loop_audition_active;
            }
        }
        if event.pressed && (event.code == 81 || event.code == button::LEVELS) {
            if self.loop_audition_active {
                unsafe {
                    let _ = mpc_sample_sdk::sites::replay_button(&event.bytes, 74);
                }
                self.loop_audition_active = false;
            }
        }
        if event.code == 81 && event.pressed {
            let now = clock::now_ms();
            let prev = LAST_STOP_MS.swap(now, Ordering::AcqRel);
            let is_double_stop = (now - prev) >= 0 && (now - prev) <= 650;
            if is_double_stop {
                LAST_STOP_MS.store(0, Ordering::Release);
                self.reset_engine_state();
            }
        }
        if event.code == button::SHIFT {
            SHIFT_ACTIVE.store(event.pressed, Ordering::Release);
            DASHBOARD_REFRESH_UNTIL_MS.store(clock::now_ms() + 500, Ordering::Release);
        }

        let flow = if event.code == button::DATA && !event.pressed && self.consumed_data {
            self.consumed_data = false;
            ButtonFlow::Consume
        } else if event.code == button::DATA
            && event.pressed
            && self.enabled
            && self.is_16_levels_active()
            && !event.shift
        {
            self.consumed_data = true;
            let _ = PREVIEW_STATE.try_update(Ordering::AcqRel, Ordering::Acquire, |state| {
                (state & PREVIEW_ENABLED != 0).then_some(if state & HOLD_ENABLED != 0 {
                    // Disabling HOLD removes all latched pads.
                    if state & MODE_MELODY != 0 {
                        PREVIEW_ENABLED | MODE_MELODY
                    } else {
                        PREVIEW_ENABLED
                    }
                } else {
                    state | HOLD_ENABLED
                })
            });
            action = 7;
            ButtonFlow::Consume
        } else if event.code == button::LEVELS && event.pressed {
            if !event.shift {
                if self.current_screen == Screen::Sample {
                    let entering = !self.in_16_levels;
                    self.in_16_levels = entering;
                    IN_16_LEVELS_ACTIVE.store(self.is_16_levels_active(), Ordering::Release);
                    action = if entering { 4 } else { 5 };
                    if entering {
                        BACKGROUND_PLAYBACK_ACTIVE.store(false, Ordering::Release);
                        LAST_STOP_MS.store(0, Ordering::Release);
                    } else if self.should_continue_in_background() {
                        BACKGROUND_PLAYBACK_ACTIVE.store(true, Ordering::Release);
                    } else {
                        self.reset_engine_state();
                        BACKGROUND_PLAYBACK_ACTIVE.store(false, Ordering::Release);
                    }
                }
            }
            ButtonFlow::Forward
        } else if event.pressed
            && matches!(
                event.code,
                BUTTON_SEQ | BUTTON_PAD_FX | BUTTON_SAMPLE_SELECT | BUTTON_MUTE | BUTTON_SAMPLE_RECORD
            )
        {
            self.current_screen = Screen::Other;
            self.in_16_levels = false;
            IN_16_LEVELS_ACTIVE.store(false, Ordering::Release);
            self.reset_engine_state();
            ButtonFlow::Forward
        } else if event.pressed && event.code == BUTTON_SAMPLE {
            if self.current_screen == Screen::Other {
                self.current_screen = Screen::Sample;
                self.in_16_levels = false;
                IN_16_LEVELS_ACTIVE.store(false, Ordering::Release);
                self.reset_engine_state();
            }
            ButtonFlow::Forward
        } else {
            let bit = match event.code {
                BUTTON_B1 => 1,
                BUTTON_B2 => 2,
                BUTTON_B3 => 4,
                _ => 0,
            };
            if bit == 0 {
                ButtonFlow::Forward
            } else if !event.pressed && self.consumed_buttons & bit != 0 {
                self.consumed_buttons &= !bit;
                ButtonFlow::Consume
            } else if !self.is_16_levels_active() || !event.shift || !event.pressed {
                ButtonFlow::Forward
            } else if bit == 1 {
                self.consumed_buttons |= bit;
                let hold_bit = PREVIEW_STATE.load(Ordering::Acquire) & HOLD_ENABLED;
                self.mode = match self.mode {
                    EngineMode::Off => EngineMode::Arp,
                    EngineMode::Arp => EngineMode::Melody,
                    EngineMode::Melody => EngineMode::Mutator,
                    EngineMode::Mutator => EngineMode::Acordator,
                    EngineMode::Acordator => EngineMode::Orbitator,
                    EngineMode::Orbitator => EngineMode::Off,
                };
                self.enabled = self.mode != EngineMode::Off;
                self.engine.clear();
                self.melody_engine.clear();
                self.mutator_engine.clear();
                self.acordator_engine.clear();
                self.orbitator_engine.clear();
                if self.mode == EngineMode::Off {
                    PREVIEW_STATE.store(0, Ordering::Release);
                } else if self.mode == EngineMode::Arp {
                    PREVIEW_STATE.store(PREVIEW_ENABLED | hold_bit, Ordering::Release);
                } else {
                    PREVIEW_STATE
                        .store(PREVIEW_ENABLED | MODE_MELODY | hold_bit, Ordering::Release);
                }
                #[cfg(feature = "emulator-audio")]
                {
                    native_audio::panic();
                    native_audio::set_mode(self.mode);
                }
                action = match self.mode {
                    EngineMode::Arp => 1,
                    EngineMode::Melody => 8,
                    EngineMode::Mutator => 9,
                    EngineMode::Acordator => 10,
                    EngineMode::Orbitator => 11,
                    EngineMode::Off => 2,
                };
                ButtonFlow::Consume
            } else if bit == 2 {
                self.consumed_buttons |= bit;
                if self.mode == EngineMode::Melody {
                    self.melody_bars = self.melody_bars.cycle();
                    #[cfg(feature = "emulator-audio")]
                    native_audio::set_melody_bars(self.melody_bars);
                } else if self.mode == EngineMode::Mutator {
                    self.mutator_intensity = self.mutator_intensity.cycle();
                    self.mutator_engine.set_intensity(self.mutator_intensity);
                    #[cfg(feature = "emulator-audio")]
                    native_audio::set_mutator_intensity(self.mutator_intensity);
                } else if self.mode == EngineMode::Acordator {
                    self.chord_mood = self.chord_mood.cycle();
                    self.acordator_engine.set_mood(self.chord_mood);
                    #[cfg(feature = "emulator-audio")]
                    native_audio::set_chord_mood(self.chord_mood);
                } else if self.mode == EngineMode::Orbitator {
                    self.orbit_atmosphere = self.orbit_atmosphere.cycle();
                    self.orbitator_engine.set_atmosphere(self.orbit_atmosphere);
                    #[cfg(feature = "emulator-audio")]
                    native_audio::set_orbit_atmosphere(self.orbit_atmosphere);
                } else {
                    let _pattern = self.engine.cycle_pattern();
                    #[cfg(feature = "emulator-audio")]
                    native_audio::set_pattern(_pattern);
                }
                action = 3;
                ButtonFlow::Consume
            } else {
                self.consumed_buttons |= bit;
                if self.mode == EngineMode::Melody {
                    self.melody_variant = self.melody_variant.cycle();
                    #[cfg(feature = "emulator-audio")]
                    native_audio::set_melody_variant(self.melody_variant);
                } else if self.mode == EngineMode::Mutator {
                    self.mutator_variant = self.mutator_variant.cycle();
                    self.mutator_engine.set_variant(self.mutator_variant);
                    #[cfg(feature = "emulator-audio")]
                    native_audio::set_mutator_variant(self.mutator_variant);
                } else if self.mode == EngineMode::Acordator {
                    self.voicing_style = self.voicing_style.cycle();
                    self.acordator_engine.set_style(self.voicing_style);
                    #[cfg(feature = "emulator-audio")]
                    native_audio::set_voicing_style(self.voicing_style);
                } else if self.mode == EngineMode::Orbitator {
                    self.orbit_dynamics = self.orbit_dynamics.cycle();
                    self.orbitator_engine.set_dynamics(self.orbit_dynamics);
                    #[cfg(feature = "emulator-audio")]
                    native_audio::set_orbit_dynamics(self.orbit_dynamics);
                } else {
                    self.additional_notes = (self.additional_notes + 1) % 4;
                    #[cfg(feature = "emulator-audio")]
                    native_audio::set_additional_octaves(self.additional_notes);
                }
                action = 6;
                ButtonFlow::Consume
            }
        };

        if action != 0 {
            DASHBOARD_REFRESH_UNTIL_MS.store(clock::now_ms() + 500, Ordering::Release);
        }
        probe::button_event(
            event.code,
            event.pressed,
            event.shift,
            self.is_16_levels_active(),
            action,
        );
        flow
    }

    fn on_encoder(&mut self, event: EncoderEvent) -> EncoderFlow {
        let _guard = UiEventGuard::enter();
        if !self.enabled || !self.is_16_levels_active() || SHIFT_ACTIVE.load(Ordering::Acquire) {
            return EncoderFlow::Forward;
        }
        let rate = self.scheduler.rate().shifted(event.delta);
        if self.scheduler.set_rate(rate) {
            #[cfg(feature = "emulator-audio")]
            native_audio::set_rate(rate);
            DASHBOARD_REFRESH_UNTIL_MS.store(clock::now_ms() + 500, Ordering::Release);
        }
        EncoderFlow::Consume
    }

    #[cfg(feature = "emulator-audio")]
    fn note_interceptor(&self) -> Option<mpc_sample_sdk::NoteInterceptor> {
        Some(native_audio::capture)
    }

    fn surface_observer(&self) -> Option<fn(SurfaceEvent)> {
        Some(observe_surface)
    }

    fn on_pad(&mut self, event: PadEvent) -> PadFlow {
        if event.action == PadAction::Press && SHIFT_ACTIVE.load(Ordering::Acquire) {
            self.current_screen = Screen::Other;
            self.in_16_levels = false;
            IN_16_LEVELS_ACTIVE.store(false, Ordering::Release);
            self.reset_engine_state();
        }
        if !self.is_16_levels_active() {
            if event.action == PadAction::Press && self.current_screen == Screen::Sample {
                if let Ok(pad) = u8::try_from(event.pad) {
                    if pad < 16 {
                        native_audio::set_active_sample_pad(pad, 36 + pad);
                    }
                }
            }
            return PadFlow::Forward;
        }
        if !self.enabled {
            return PadFlow::Forward;
        }
        if let Ok(pad) = u8::try_from(event.pad) {
            if pad < 16 && SHORTCUT_PADS.load(Ordering::Acquire) & (1 << pad) != 0 {
                return PadFlow::Forward;
            }
            match event.action {
                PadAction::Press => {
                    match self.mode {
                        EngineMode::Melody => self.melody_engine.press(pad, event.velocity),
                        EngineMode::Mutator => self.mutator_engine.press(pad, event.velocity),
                        EngineMode::Acordator => self.acordator_engine.press(pad, event.velocity),
                        EngineMode::Orbitator => self.orbitator_engine.press(pad, event.velocity),
                        _ => self.engine.press(pad, event.velocity),
                    }
                }
                PadAction::Release => {
                    match self.mode {
                        EngineMode::Melody => self.melody_engine.release(pad),
                        EngineMode::Mutator => self.mutator_engine.release(pad),
                        EngineMode::Acordator => self.acordator_engine.release(pad),
                        EngineMode::Orbitator => self.orbitator_engine.release(pad),
                        _ => self.engine.release(pad),
                    }
                    if self.enabled && self.is_16_levels_active() {
                        return PadFlow::Consume;
                    }
                }
            }
        }
        // Physical strikes remain audible. Scheduled retriggers will be
        // connected after the live input object and tempo clock are validated.
        PadFlow::Forward
    }

    fn on_pitch(&mut self, applied: i32) -> i32 {
        self.current_screen = Screen::Sample;
        self.in_16_levels = true;
        IN_16_LEVELS_ACTIVE.store(self.is_16_levels_active(), Ordering::Release);
        DASHBOARD_REFRESH_UNTIL_MS.store(clock::now_ms() + 500, Ordering::Release);
        applied
    }

    fn on_paint(&mut self, graphics: &mut mpc_sample_sdk::draw::Graphics, event: PaintEvent) {
        let _guard = PaintGuard::enter();
        self.sync_navigation(event.sample_page, event.chop_mode);
        if !event.sample_page {
            return;
        }
        probe::on_paint();
        let now = clock::now_ms();
        if self.splash_until_ms == 0 {
            self.splash_until_ms = now + 2_000;
            SPLASH_ACTIVE.store(true, Ordering::Release);
        }
        if now < self.splash_until_ms {
            // SAFETY: the SDK invokes this callback with the live UI graphics.
            unsafe { splash::draw_boot_splash(graphics) };
            return;
        } else if SPLASH_ACTIVE.load(Ordering::Acquire) {
            SPLASH_ACTIVE.store(false, Ordering::Release);
        }
        let preview = PREVIEW_STATE.load(Ordering::Acquire);
        let active = preview & PAD_MASK;
        let target_leds = if self.is_16_levels_active() { active } else { 0 };
        let changed = self.led_mask ^ target_leds;
        for pad in 0..16 {
            if changed & (1 << pad) != 0 {
                if target_leds & (1 << pad) != 0 {
                    let _ = lighting::send_pad_light(pad as u8, 0x00ff00, 1.0);
                } else {
                    lighting::restore_pad_rgb(pad as u8);
                }
            }
        }
        self.led_mask = target_leds;
        let shift_active = SHIFT_ACTIVE.load(Ordering::Acquire);
        let show_shift_menu = self.is_16_levels_active() && shift_active;
        let show_overlay = self.is_16_levels_active() && self.enabled;
        if !show_shift_menu && !show_overlay {
            return;
        }
        let b2_label = match self.mode {
            EngineMode::Melody => self.melody_bars.label(),
            EngineMode::Mutator => self.mutator_intensity.label(),
            EngineMode::Acordator => self.chord_mood.label(),
            EngineMode::Orbitator => self.orbit_atmosphere.label(),
            _ => self.engine.pattern().label(),
        };
        let b3_formatted;
        let b3_label = match self.mode {
            EngineMode::Melody => self.melody_variant.label(),
            EngineMode::Mutator => self.mutator_variant.label(),
            EngineMode::Acordator => self.voicing_style.label(),
            EngineMode::Orbitator => self.orbit_dynamics.label(),
            _ => {
                b3_formatted = format!("+{}", self.additional_notes);
                &b3_formatted
            }
        };
        // SAFETY: on_paint supplies the live Graphics object on the app's
        // paint path.
        unsafe {
            splash::draw_arp_overlay(
                graphics,
                show_shift_menu,
                self.enabled,
                self.mode,
                b2_label,
                b3_label,
                (
                    active.count_ones() as u8,
                    preview & HOLD_ENABLED != 0,
                    self.scheduler.rate().label(),
                ),
                now as u64,
            )
        }
    }
}

impl Drop for ArpeggiGator {
    fn drop(&mut self) {
        lighting::restore_pad_rgbs();
        for pad in 0..16 {
            let _ = lighting::set_pad_light(pad, 0, 0.0);
        }
    }
}

mpc_sample_sdk::export_mod!(ArpeggiGator);

#[cfg(test)]
mod animation_input_tests {
    use super::*;
    use std::sync::Mutex;

    static INPUT_TEST_LOCK: Mutex<()> = Mutex::new(());

    fn surface(code: i32, status: u8, velocity: u8) -> SurfaceEvent {
        SurfaceEvent {
            surface: 0,
            code,
            payload: 0,
            status,
            kind: status & 0xf0,
            note: 0,
            velocity,
        }
    }

    #[test]
    fn encoder_controls_rate_only_in_active_arp_without_shift() {
        let _guard = INPUT_TEST_LOCK.lock().unwrap();
        let mut module = ArpeggiGator::default();
        let turn = EncoderEvent {
            surface: 0,
            delta: 1,
        };
        SHIFT_ACTIVE.store(false, Ordering::Release);
        assert_eq!(module.on_encoder(turn), EncoderFlow::Forward);
        assert_eq!(module.scheduler.rate(), StepRate::Sixteenth);
        module.enabled = true;
        module.in_16_levels = true;
        assert_eq!(module.on_encoder(turn), EncoderFlow::Consume);
        assert_eq!(module.scheduler.rate(), StepRate::SixteenthTriplet);
        SHIFT_ACTIVE.store(true, Ordering::Release);
        assert_eq!(module.on_encoder(turn), EncoderFlow::Forward);
        assert_eq!(module.scheduler.rate(), StepRate::SixteenthTriplet);
        SHIFT_ACTIVE.store(false, Ordering::Release);
        module.in_16_levels = false;
        assert_eq!(module.on_encoder(turn), EncoderFlow::Forward);
    }

    #[test]
    fn animation_tracks_physical_pads_through_release_and_mode_changes() {
        let _guard = INPUT_TEST_LOCK.lock().unwrap();
        SHIFT_ACTIVE.store(false, Ordering::Release);
        PHYSICAL_PADS.store(0, Ordering::Release);
        SHORTCUT_PADS.store(0, Ordering::Release);
        BACKGROUND_PLAYBACK_ACTIVE.store(false, Ordering::Release);
        PREVIEW_STATE.store(0, Ordering::Release);
        observe_surface(surface(0, 0x99, 100));
        assert_eq!(PREVIEW_STATE.load(Ordering::Acquire) & PAD_MASK, 0);
        observe_surface(surface(0, 0x89, 0));

        PREVIEW_STATE.store(PREVIEW_ENABLED, Ordering::Release);
        observe_surface(surface(0, 0x99, 100));
        observe_surface(surface(1, 0x99, 100));
        assert_eq!(PREVIEW_STATE.load(Ordering::Acquire) & PAD_MASK, 0b11);
        // Other buttons, pressure, and out-of-range codes do not alter the pads.
        observe_surface(surface(0, 0x80, 0));
        observe_surface(surface(1, 0xa9, 64));
        observe_surface(surface(16, 0x99, 100));
        assert_eq!(PREVIEW_STATE.load(Ordering::Acquire) & PAD_MASK, 0b11);
        observe_surface(surface(0, 0x89, 0));
        assert_eq!(PREVIEW_STATE.load(Ordering::Acquire) & PAD_MASK, 0b10);
        observe_surface(surface(1, 0x99, 0));
        assert_eq!(PREVIEW_STATE.load(Ordering::Acquire) & PAD_MASK, 0);
        assert!(DASHBOARD_REFRESH_UNTIL_MS.load(Ordering::Acquire) >= clock::now_ms());

        // A fast tap leaves no pad held.
        observe_surface(surface(15, 0x99, 127));
        observe_surface(surface(15, 0x89, 0));
        assert_eq!(PREVIEW_STATE.load(Ordering::Acquire) & PAD_MASK, 0);
        PREVIEW_STATE.store(PREVIEW_ENABLED | HOLD_ENABLED, Ordering::Release);
        for pad in 0..4 {
            observe_surface(surface(pad, 0x99, 100));
            observe_surface(surface(pad, 0x89, 0));
        }
        assert_eq!(PREVIEW_STATE.load(Ordering::Acquire) & PAD_MASK, 0b1111);
        observe_surface(surface(1, 0x99, 100));
        observe_surface(surface(1, 0x89, 0));
        assert_eq!(PREVIEW_STATE.load(Ordering::Acquire) & PAD_MASK, 0b1101);
        // A shortcut must neither toggle an existing latch nor add a new one,
        // even if SHIFT is released before the pad.
        SHIFT_ACTIVE.store(true, Ordering::Release);
        observe_surface(surface(0, 0x99, 100));
        observe_surface(surface(15, 0x99, 100));
        assert_eq!(PREVIEW_STATE.load(Ordering::Acquire) & PAD_MASK, 0b1101);
        assert_eq!(SHORTCUT_PADS.load(Ordering::Acquire), 1 | (1 << 15));
        SHIFT_ACTIVE.store(false, Ordering::Release);
        observe_surface(surface(0, 0x89, 0));
        observe_surface(surface(15, 0x89, 0));
        assert_eq!(PREVIEW_STATE.load(Ordering::Acquire) & PAD_MASK, 0b1101);
        assert_eq!(SHORTCUT_PADS.load(Ordering::Acquire), 0);
        PREVIEW_STATE.store(0, Ordering::Release);
    }

    fn button_ev(code: u32, pressed: bool, shift: bool) -> ButtonEvent {
        ButtonEvent {
            event: 0,
            bytes: [0; 16],
            code,
            value: if pressed { button::PRESSED } else { 0 },
            pressed,
            shift,
            erase: false,
            surface: 0,
        }
    }

    #[test]
    fn shift_menu_and_softkeys_track_16_levels_and_navigation() {
        let _guard = INPUT_TEST_LOCK.lock().unwrap();
        let mut module = ArpeggiGator::default();
        module.on_start();

        // 1. Initial state: on Sample screen, 16 Levels inactive.
        assert!(!module.is_16_levels_active());
        // SHIFT+B1/B2/B3 must be forwarded to native firmware.
        assert_eq!(
            module.on_button(button_ev(BUTTON_B1, true, true)),
            ButtonFlow::Forward
        );
        assert!(!module.enabled);

        // 2. Press 16 LEVELS: enters 16 Levels.
        assert_eq!(
            module.on_button(button_ev(button::LEVELS, true, false)),
            ButtonFlow::Forward
        );
        assert!(module.is_16_levels_active());
        // Now SHIFT+B1 toggles ARP and is consumed.
        assert_eq!(
            module.on_button(button_ev(BUTTON_B1, true, true)),
            ButtonFlow::Consume
        );
        assert!(module.enabled);
        assert_eq!(
            module.on_button(button_ev(BUTTON_B1, false, true)),
            ButtonFlow::Consume
        );

        // 3. Press SEQ, then SAMPLE: switches to SEQ, then returns to SAMPLE.
        assert_eq!(
            module.on_button(button_ev(BUTTON_SEQ, true, false)),
            ButtonFlow::Forward
        );
        assert!(!module.is_16_levels_active());
        assert_eq!(module.current_screen, Screen::Other);

        // Outside 16 Levels, SHIFT+B1 is forwarded to native.
        assert_eq!(
            module.on_button(button_ev(BUTTON_B1, true, true)),
            ButtonFlow::Forward
        );

        // Press SAMPLE to return to Sample screen.
        assert_eq!(
            module.on_button(button_ev(BUTTON_SAMPLE, true, false)),
            ButtonFlow::Forward
        );
        assert_eq!(module.current_screen, Screen::Sample);
        // Returning from SEQ leaves 16 Levels OFF!
        assert!(!module.is_16_levels_active());

        // In normal Sample mode, SHIFT+B1 is forwarded to native (Loop Lock / Normalize).
        assert_eq!(
            module.on_button(button_ev(BUTTON_B1, true, true)),
            ButtonFlow::Forward
        );

        // 4. Press 16 LEVELS again: re-enters 16 Levels!
        assert_eq!(
            module.on_button(button_ev(button::LEVELS, true, false)),
            ButtonFlow::Forward
        );
        assert!(module.is_16_levels_active());

        // In re-entered 16 Levels, SHIFT+B1 is consumed by ArpeggiGator!
        assert_eq!(
            module.on_button(button_ev(BUTTON_B1, true, true)),
            ButtonFlow::Consume
        );
    }

    #[test]
    fn pressing_sample_while_in_16_levels_preserves_16_levels() {
        let _guard = INPUT_TEST_LOCK.lock().unwrap();
        let mut module = ArpeggiGator::default();
        module.on_start();

        // Enter 16 Levels.
        module.on_button(button_ev(button::LEVELS, true, false));
        assert!(module.is_16_levels_active());

        // Pressing SAMPLE while already on Sample page does not exit 16 Levels.
        assert_eq!(
            module.on_button(button_ev(BUTTON_SAMPLE, true, false)),
            ButtonFlow::Forward
        );
        assert!(module.is_16_levels_active());
    }

    #[test]
    fn pressing_16_levels_while_on_other_screen_is_ignored() {
        let _guard = INPUT_TEST_LOCK.lock().unwrap();
        let mut module = ArpeggiGator::default();
        module.on_start();

        // Navigate to PAD FX.
        module.on_button(button_ev(BUTTON_PAD_FX, true, false));
        assert_eq!(module.current_screen, Screen::Other);
        assert!(!module.is_16_levels_active());

        // 16 LEVELS button on other screens does not enter 16 Levels.
        module.on_button(button_ev(button::LEVELS, true, false));
        assert!(!module.is_16_levels_active());
        assert_eq!(module.current_screen, Screen::Other);
    }

    #[test]
    fn shift_levels_type_selection_does_not_toggle_16_levels() {
        let _guard = INPUT_TEST_LOCK.lock().unwrap();
        let mut module = ArpeggiGator::default();
        module.on_start();

        // Enter 16 Levels.
        module.on_button(button_ev(button::LEVELS, true, false));
        assert!(module.is_16_levels_active());

        // SHIFT + 16 LEVELS is TYPE selection; it must not exit 16 Levels.
        module.on_button(button_ev(button::LEVELS, true, true));
        assert!(module.is_16_levels_active());
    }

    #[test]
    fn shortcut_pad_press_exits_16_levels() {
        let _guard = INPUT_TEST_LOCK.lock().unwrap();
        let mut module = ArpeggiGator::default();
        module.on_start();

        // Enter 16 Levels.
        module.on_button(button_ev(button::LEVELS, true, false));
        assert!(module.is_16_levels_active());

        // Press SHIFT + PAD 16 (pad index 15, Project shortcut).
        SHIFT_ACTIVE.store(true, Ordering::Release);
        module.on_pad(PadEvent {
            action: PadAction::Press,
            input: 0,
            source: 0,
            pad: 15,
            properties: 0,
            velocity: 1.0,
        });
        assert_eq!(module.current_screen, Screen::Other);
        assert!(!module.is_16_levels_active());
        SHIFT_ACTIVE.store(false, Ordering::Release);
    }

    #[test]
    fn native_lighting_synchronizes_16_levels_state() {
        let _guard = INPUT_TEST_LOCK.lock().unwrap();
        LAST_STOP_MS.store(0, Ordering::Release);
        BACKGROUND_PLAYBACK_ACTIVE.store(false, Ordering::Release);
        lighting::reset_lighting();
        let mut module = ArpeggiGator::default();
        module.on_start();

        let unlit = u64::from(0.0_f32.to_bits()) << 32 | 0x00ff_ffff;
        let p1_lit = u64::from(1.0_f32.to_bits()) << 32 | 0x000a_000f;
        let p15_lit = u64::from(1.0_f32.to_bits()) << 32 | 0x0096_00ef;

        // Initial hardware state: outside 16 levels.
        lighting::observe(0x1000, 1, unlit);
        lighting::observe(0x1000, 15, unlit);
        assert!(lighting::has_native_lighting());
        assert!(!lighting::is_16_levels_lit());

        // Button LEVELS pressed to enter 16 Levels:
        module.on_button(button_ev(button::LEVELS, true, false));
        assert!(module.in_16_levels);

        // Hardware confirms 16 Levels by lighting chromatic purple pads:
        lighting::observe(0x1000, 1, p1_lit);
        lighting::observe(0x1000, 15, p15_lit);
        assert!(lighting::is_16_levels_lit());
        assert!(module.is_16_levels_active());

        // Native hardware turns off 16 Levels (e.g. shortcut pad dialog):
        lighting::observe(0x1000, 1, unlit);
        lighting::observe(0x1000, 15, unlit);
        assert!(!lighting::is_16_levels_lit());
        assert!(!module.is_16_levels_active());

        // When returning to Sample screen, navigation sync synchronizes in_16_levels:
        module.sync_navigation(true, false);
        assert!(!module.in_16_levels);

        // Next button LEVELS press correctly enters 16 levels (no inversion!):
        module.on_button(button_ev(button::LEVELS, true, false));
        assert!(module.in_16_levels);

        lighting::reset_lighting();
    }

    #[test]
    fn melody_mode_softkey_selection_and_pad_switching() {
        let _guard = INPUT_TEST_LOCK.lock().unwrap();
        LAST_STOP_MS.store(0, Ordering::Release);
        BACKGROUND_PLAYBACK_ACTIVE.store(false, Ordering::Release);
        lighting::reset_lighting();
        let mut module = ArpeggiGator::default();
        module.on_start();

        // Enter 16 Levels
        module.on_button(button_ev(button::LEVELS, true, false));
        assert!(module.is_16_levels_active());
        assert_eq!(module.mode, EngineMode::Off);

        // 1st SHIFT+B1 -> enters Arp
        module.on_button(button_ev(BUTTON_B1, true, true));
        module.on_button(button_ev(BUTTON_B1, false, true));
        assert_eq!(module.mode, EngineMode::Arp);
        assert!(module.enabled);

        // 2nd SHIFT+B1 -> enters Melody
        module.on_button(button_ev(BUTTON_B1, true, true));
        module.on_button(button_ev(BUTTON_B1, false, true));
        assert_eq!(module.mode, EngineMode::Melody);
        assert!(module.enabled);
        assert_eq!(PREVIEW_STATE.load(Ordering::Acquire) & MODE_MELODY, MODE_MELODY);

        // In Melody: SHIFT+B2 cycles bars
        assert_eq!(module.melody_bars, BarLength::Bars2);
        module.on_button(button_ev(BUTTON_B2, true, true));
        module.on_button(button_ev(BUTTON_B2, false, true));
        assert_eq!(module.melody_bars, BarLength::Bars4);
        module.on_button(button_ev(BUTTON_B2, true, true));
        module.on_button(button_ev(BUTTON_B2, false, true));
        assert_eq!(module.melody_bars, BarLength::Bars8);
        module.on_button(button_ev(BUTTON_B2, true, true));
        module.on_button(button_ev(BUTTON_B2, false, true));
        assert_eq!(module.melody_bars, BarLength::Bars16);

        // In Melody: SHIFT+B3 cycles melody variants MEL1..MEL9
        assert_eq!(module.melody_variant, MelodyVariant::Mel1);
        for expected in [
            MelodyVariant::Mel2,
            MelodyVariant::Mel3,
            MelodyVariant::Mel4,
            MelodyVariant::Mel5,
            MelodyVariant::Mel6,
            MelodyVariant::Mel7,
            MelodyVariant::Mel8,
            MelodyVariant::Mel9,
            MelodyVariant::Mel1,
        ] {
            module.on_button(button_ev(BUTTON_B3, true, true));
            module.on_button(button_ev(BUTTON_B3, false, true));
            assert_eq!(module.melody_variant, expected);
        }

        // In Melody with HOLD: test pad switching and single-pad latch
        // Turn HOLD on (click DATA wheel)
        module.on_button(button_ev(button::DATA, true, false));
        module.on_button(button_ev(button::DATA, false, false));
        assert_ne!(PREVIEW_STATE.load(Ordering::Acquire) & HOLD_ENABLED, 0);

        // Strike Pad 1
        observe_surface(surface(1, 0x99, 100));
        observe_surface(surface(1, 0x89, 0));
        assert_eq!(PREVIEW_STATE.load(Ordering::Acquire) & PAD_MASK, 1 << 1);

        // Strike Pad 5: voice steals! Pad 1 turns off, Pad 5 latches!
        observe_surface(surface(5, 0x99, 100));
        observe_surface(surface(5, 0x89, 0));
        assert_eq!(PREVIEW_STATE.load(Ordering::Acquire) & PAD_MASK, 1 << 5);

        // Strike Pad 5 again: toggles off!
        observe_surface(surface(5, 0x99, 100));
        observe_surface(surface(5, 0x89, 0));
        assert_eq!(PREVIEW_STATE.load(Ordering::Acquire) & PAD_MASK, 0);

        // 3rd SHIFT+B1 -> advances to Mutator
        module.on_button(button_ev(BUTTON_B1, true, true));
        module.on_button(button_ev(BUTTON_B1, false, true));
        assert_eq!(module.mode, EngineMode::Mutator);
        assert!(module.enabled);

        // In Mutator: SHIFT+B2 cycles intensity (Subtle -> Wild -> Chaos -> Subtle)
        assert_eq!(module.mutator_intensity, MutationIntensity::Subtle);
        module.on_button(button_ev(BUTTON_B2, true, true));
        module.on_button(button_ev(BUTTON_B2, false, true));
        assert_eq!(module.mutator_intensity, MutationIntensity::Wild);
        module.on_button(button_ev(BUTTON_B2, true, true));
        module.on_button(button_ev(BUTTON_B2, false, true));
        assert_eq!(module.mutator_intensity, MutationIntensity::Chaos);
        module.on_button(button_ev(BUTTON_B2, true, true));
        module.on_button(button_ev(BUTTON_B2, false, true));
        assert_eq!(module.mutator_intensity, MutationIntensity::Subtle);

        // In Mutator: SHIFT+B3 cycles variants
        assert_eq!(module.mutator_variant, MelodyVariant::Mel1);
        module.on_button(button_ev(BUTTON_B3, true, true));
        module.on_button(button_ev(BUTTON_B3, false, true));
        assert_eq!(module.mutator_variant, MelodyVariant::Mel2);

        // 4th SHIFT+B1 -> advances to Acordator
        module.on_button(button_ev(BUTTON_B1, true, true));
        module.on_button(button_ev(BUTTON_B1, false, true));
        assert_eq!(module.mode, EngineMode::Acordator);
        assert!(module.enabled);

        // In Acordator: SHIFT+B2 cycles mood (Warm -> Sad -> Space -> Cinema -> Warm)
        assert_eq!(module.chord_mood, ChordMood::Warm);
        module.on_button(button_ev(BUTTON_B2, true, true));
        module.on_button(button_ev(BUTTON_B2, false, true));
        assert_eq!(module.chord_mood, ChordMood::Sad);
        module.on_button(button_ev(BUTTON_B2, true, true));
        module.on_button(button_ev(BUTTON_B2, false, true));
        assert_eq!(module.chord_mood, ChordMood::Space);
        module.on_button(button_ev(BUTTON_B2, true, true));
        module.on_button(button_ev(BUTTON_B2, false, true));
        assert_eq!(module.chord_mood, ChordMood::Cinema);
        module.on_button(button_ev(BUTTON_B2, true, true));
        module.on_button(button_ev(BUTTON_B2, false, true));
        assert_eq!(module.chord_mood, ChordMood::Warm);

        // In Acordator: SHIFT+B3 cycles style (Strum -> Pulse -> Wave -> Strum)
        assert_eq!(module.voicing_style, VoicingStyle::Strum);
        module.on_button(button_ev(BUTTON_B3, true, true));
        module.on_button(button_ev(BUTTON_B3, false, true));
        assert_eq!(module.voicing_style, VoicingStyle::Pulse);
        module.on_button(button_ev(BUTTON_B3, true, true));
        module.on_button(button_ev(BUTTON_B3, false, true));
        assert_eq!(module.voicing_style, VoicingStyle::Wave);
        module.on_button(button_ev(BUTTON_B3, true, true));
        module.on_button(button_ev(BUTTON_B3, false, true));
        assert_eq!(module.voicing_style, VoicingStyle::Strum);

        // 5th SHIFT+B1 -> advances to Orbitator
        module.on_button(button_ev(BUTTON_B1, true, true));
        module.on_button(button_ev(BUTTON_B1, false, true));
        assert_eq!(module.mode, EngineMode::Orbitator);
        assert!(module.enabled);

        // In Orbitator: SHIFT+B2 cycles atmosphere (Ambient -> Space -> Dark -> Rhythm -> Ambient)
        assert_eq!(module.orbit_atmosphere, OrbitAtmosphere::Ambient);
        module.on_button(button_ev(BUTTON_B2, true, true));
        module.on_button(button_ev(BUTTON_B2, false, true));
        assert_eq!(module.orbit_atmosphere, OrbitAtmosphere::Space);
        module.on_button(button_ev(BUTTON_B2, true, true));
        module.on_button(button_ev(BUTTON_B2, false, true));
        assert_eq!(module.orbit_atmosphere, OrbitAtmosphere::Dark);
        module.on_button(button_ev(BUTTON_B2, true, true));
        module.on_button(button_ev(BUTTON_B2, false, true));
        assert_eq!(module.orbit_atmosphere, OrbitAtmosphere::Rhythm);
        module.on_button(button_ev(BUTTON_B2, true, true));
        module.on_button(button_ev(BUTTON_B2, false, true));
        assert_eq!(module.orbit_atmosphere, OrbitAtmosphere::Ambient);

        // In Orbitator: SHIFT+B3 cycles dynamics (Calm -> Flow -> Wild -> Calm)
        assert_eq!(module.orbit_dynamics, OrbitDynamics::Calm);
        module.on_button(button_ev(BUTTON_B3, true, true));
        module.on_button(button_ev(BUTTON_B3, false, true));
        assert_eq!(module.orbit_dynamics, OrbitDynamics::Flow);
        module.on_button(button_ev(BUTTON_B3, true, true));
        module.on_button(button_ev(BUTTON_B3, false, true));
        assert_eq!(module.orbit_dynamics, OrbitDynamics::Wild);
        module.on_button(button_ev(BUTTON_B3, true, true));
        module.on_button(button_ev(BUTTON_B3, false, true));
        assert_eq!(module.orbit_dynamics, OrbitDynamics::Calm);

        // 6th SHIFT+B1 -> returns to Off
        module.on_button(button_ev(BUTTON_B1, true, true));
        module.on_button(button_ev(BUTTON_B1, false, true));
        assert_eq!(module.mode, EngineMode::Off);
        assert!(!module.enabled);
        assert_eq!(PREVIEW_STATE.load(Ordering::Acquire), 0);

        lighting::reset_lighting();
    }

    #[test]
    fn melody_mode_navigating_away_and_returning_to_16_levels_resets_cleanly() {
        let _guard = INPUT_TEST_LOCK.lock().unwrap();
        LAST_STOP_MS.store(0, Ordering::Release);
        BACKGROUND_PLAYBACK_ACTIVE.store(false, Ordering::Release);
        lighting::reset_lighting();
        let mut module = ArpeggiGator::default();
        module.on_start();

        // 1. Enter 16 Levels
        module.on_button(button_ev(button::LEVELS, true, false));
        assert!(module.is_16_levels_active());

        // 2. Turn on Melody Mode (2x SHIFT+B1)
        module.on_button(button_ev(BUTTON_B1, true, true));
        module.on_button(button_ev(BUTTON_B1, false, true));
        module.on_button(button_ev(BUTTON_B1, true, true));
        module.on_button(button_ev(BUTTON_B1, false, true));
        assert_eq!(module.mode, EngineMode::Melody);
        assert!(module.enabled);

        // 3. User navigates away to another screen (e.g. SEQ button)
        module.on_button(button_ev(BUTTON_SEQ, true, false));
        assert_eq!(module.current_screen, Screen::Other);
        assert!(!module.is_16_levels_active());
        assert!(!module.enabled);
        assert_eq!(module.mode, EngineMode::Off);
        assert_eq!(PREVIEW_STATE.load(Ordering::Acquire), 0);

        // 4. User navigates back to SAMPLE screen
        module.on_button(button_ev(BUTTON_SAMPLE, true, false));
        assert_eq!(module.current_screen, Screen::Sample);
        assert!(!module.is_16_levels_active());
        assert!(!module.enabled);
        assert_eq!(module.mode, EngineMode::Off);

        // 5. User re-enters 16 Levels
        module.on_button(button_ev(button::LEVELS, true, false));
        assert!(module.is_16_levels_active());
        // Clean initial state: not enabled, mode is OFF!
        assert!(!module.enabled);
        assert_eq!(module.mode, EngineMode::Off);

        // 6. User presses SHIFT: mode must still be Off (no phantom gator!)
        SHIFT_ACTIVE.store(true, Ordering::Release);
        assert_eq!(module.mode, EngineMode::Off);
        assert!(!module.enabled);

        // 7. Pressing SHIFT+B1 cleanly turns on ARP mode first
        module.on_button(button_ev(BUTTON_B1, true, true));
        module.on_button(button_ev(BUTTON_B1, false, true));
        assert_eq!(module.mode, EngineMode::Arp);
        assert!(module.enabled);

        SHIFT_ACTIVE.store(false, Ordering::Release);
        lighting::reset_lighting();
    }

    #[test]
    fn loop_button_is_consumed_in_16_levels_and_cancelled_by_stop_or_levels() {
        let _guard = INPUT_TEST_LOCK.lock().unwrap();
        let mut module = ArpeggiGator::default();
        module.on_start();

        // 1. Outside 16 Levels: LOOP button (74) toggles loop audition state.
        assert_eq!(module.on_button(button_ev(74, true, false)), ButtonFlow::Forward);
        assert!(module.loop_audition_active);

        // 2. Pressing STOP (81) turns off loop audition state.
        assert_eq!(module.on_button(button_ev(81, true, false)), ButtonFlow::Forward);
        assert!(!module.loop_audition_active);

        // 3. Toggling LOOP on again, then entering 16 Levels (40): cancels loop audition.
        assert_eq!(module.on_button(button_ev(74, true, false)), ButtonFlow::Forward);
        assert!(module.loop_audition_active);
        assert_eq!(module.on_button(button_ev(button::LEVELS, true, false)), ButtonFlow::Forward);
        assert!(!module.loop_audition_active);
        assert!(module.is_16_levels_active());

        // 4. Inside 16 Levels: LOOP button (74) is consumed and does not activate audition!
        assert_eq!(module.on_button(button_ev(74, true, false)), ButtonFlow::Consume);
        assert!(!module.loop_audition_active);
    }

    #[test]
    fn repaint_component_only_targets_valid_screen_and_stops_outside_16_levels() {
        let _guard = INPUT_TEST_LOCK.lock().unwrap();
        let mut module = ArpeggiGator::default();
        module.on_start();

        // Screen is 0 initially (not a valid SamplePage) -> returns 0
        assert_eq!(repaint_component(), 0);

        // When not in 16 levels and splash inactive, returns 0 even if DASHBOARD_REFRESH is set
        SPLASH_ACTIVE.store(false, Ordering::Release);
        IN_16_LEVELS_ACTIVE.store(false, Ordering::Release);
        DASHBOARD_REFRESH_UNTIL_MS.store(clock::now_ms() + 10_000, Ordering::Release);
        assert_eq!(repaint_component(), 0);

        // Entering 16 levels updates IN_16_LEVELS_ACTIVE
        module.on_button(button_ev(button::LEVELS, true, false));
        assert!(IN_16_LEVELS_ACTIVE.load(Ordering::Acquire));

        // Exiting 16 levels resets IN_16_LEVELS_ACTIVE to false
        module.on_button(button_ev(BUTTON_SEQ, true, false));
        assert!(!IN_16_LEVELS_ACTIVE.load(Ordering::Acquire));
        assert_eq!(repaint_component(), 0);

        // When painting or dispatching events, repaint_component yields to prevent JUCE races
        IN_16_LEVELS_ACTIVE.store(true, Ordering::Release);
        SPLASH_ACTIVE.store(false, Ordering::Release);
        DASHBOARD_REFRESH_UNTIL_MS.store(clock::now_ms() + 10_000, Ordering::Release);
        IS_PAINTING.store(true, Ordering::Release);
        assert_eq!(repaint_component(), 0);
        IS_PAINTING.store(false, Ordering::Release);

        IS_DISPATCHING_EVENT.store(true, Ordering::Release);
        assert_eq!(repaint_component(), 0);
        IS_DISPATCHING_EVENT.store(false, Ordering::Release);
    }

    #[test]
    fn hold_persists_across_engine_modes_and_resets_in_off() {
        let _guard = INPUT_TEST_LOCK.lock().unwrap();
        LAST_STOP_MS.store(0, Ordering::Release);
        BACKGROUND_PLAYBACK_ACTIVE.store(false, Ordering::Release);
        lighting::reset_lighting();
        let mut module = ArpeggiGator::default();
        module.on_start();

        // 1. Enter 16 Levels
        module.on_button(button_ev(button::LEVELS, true, false));
        assert!(module.is_16_levels_active());

        // Initially HOLD is off
        assert_eq!(PREVIEW_STATE.load(Ordering::Acquire) & HOLD_ENABLED, 0);

        // Turn on ARP (SHIFT + B1)
        module.on_button(button_ev(BUTTON_B1, true, true));
        module.on_button(button_ev(BUTTON_B1, false, true));
        assert_eq!(module.mode, EngineMode::Arp);
        assert_eq!(PREVIEW_STATE.load(Ordering::Acquire) & HOLD_ENABLED, 0);

        // 2. Enable HOLD (click DATA wheel)
        module.on_button(button_ev(button::DATA, true, false));
        module.on_button(button_ev(button::DATA, false, false));
        assert_ne!(PREVIEW_STATE.load(Ordering::Acquire) & HOLD_ENABLED, 0);

        // 3. Cycle through active modes: Melody -> Mutator -> Acordator -> Orbitator
        for expected_mode in [
            EngineMode::Melody,
            EngineMode::Mutator,
            EngineMode::Acordator,
            EngineMode::Orbitator,
        ] {
            module.on_button(button_ev(BUTTON_B1, true, true));
            module.on_button(button_ev(BUTTON_B1, false, true));
            assert_eq!(module.mode, expected_mode);
            assert_ne!(
                PREVIEW_STATE.load(Ordering::Acquire) & HOLD_ENABLED,
                0,
                "HOLD should persist when transitioning to {:?}",
                expected_mode
            );
        }

        // 4. Next SHIFT + B1 transitions Orbitator -> Off
        module.on_button(button_ev(BUTTON_B1, true, true));
        module.on_button(button_ev(BUTTON_B1, false, true));
        assert_eq!(module.mode, EngineMode::Off);
        assert_eq!(PREVIEW_STATE.load(Ordering::Acquire), 0);
        assert_eq!(PREVIEW_STATE.load(Ordering::Acquire) & HOLD_ENABLED, 0);

        // 5. Next SHIFT + B1 transitions Off -> Arp: starts with default HOLD: OFF
        module.on_button(button_ev(BUTTON_B1, true, true));
        module.on_button(button_ev(BUTTON_B1, false, true));
        assert_eq!(module.mode, EngineMode::Arp);
        assert_eq!(PREVIEW_STATE.load(Ordering::Acquire) & HOLD_ENABLED, 0);

        lighting::reset_lighting();
    }

    #[test]
    fn live_sample_browsing_persists_hold_and_redirects_pads_until_double_stop() {
        let _guard = INPUT_TEST_LOCK.lock().unwrap();
        LAST_STOP_MS.store(0, Ordering::Release);
        BACKGROUND_PLAYBACK_ACTIVE.store(false, Ordering::Release);
        lighting::reset_lighting();
        let mut module = ArpeggiGator::default();
        module.on_start();

        // 1. Enter 16 Levels
        module.on_button(button_ev(button::LEVELS, true, false));
        assert!(module.is_16_levels_active());

        // 2. Turn on Mutator Mode (3x SHIFT+B1)
        for _ in 0..3 {
            module.on_button(button_ev(BUTTON_B1, true, true));
            module.on_button(button_ev(BUTTON_B1, false, true));
        }
        assert_eq!(module.mode, EngineMode::Mutator);
        assert!(module.enabled);

        // 3. Enable HOLD (click DATA wheel)
        module.on_button(button_ev(button::DATA, true, false));
        module.on_button(button_ev(button::DATA, false, false));
        assert_ne!(PREVIEW_STATE.load(Ordering::Acquire) & HOLD_ENABLED, 0);

        // 4. Latch Pad 0 in 16 Levels
        observe_surface(surface(0, 0x99, 100));
        observe_surface(surface(0, 0x89, 0));
        assert_ne!(PREVIEW_STATE.load(Ordering::Acquire) & PAD_MASK, 0);
        assert_eq!(PREVIEW_STATE.load(Ordering::Acquire) & 1, 1);

        // 5. User exits 16 Levels to main SAMPLE screen (press 16 LEVELS)
        module.on_button(button_ev(button::LEVELS, true, false));
        assert!(!module.is_16_levels_active());
        assert_eq!(module.current_screen, Screen::Sample);

        // Assert: ArpeggiGator continues running in background!
        assert!(BACKGROUND_PLAYBACK_ACTIVE.load(Ordering::Acquire));
        assert!(module.enabled);
        assert_eq!(module.mode, EngineMode::Mutator);
        assert_ne!(PREVIEW_STATE.load(Ordering::Acquire) & HOLD_ENABLED, 0);
        assert_eq!(PREVIEW_STATE.load(Ordering::Acquire) & 1, 1);

        // 6. On SAMPLE screen, user hits physical Pad 5 (selects sample slot 5)
        observe_surface(surface(5, 0x99, 100));
        observe_surface(surface(5, 0x89, 0));
        assert_eq!(native_audio::active_sample_pad(), Some(5));
        assert_eq!(native_audio::active_sample_note(), 41); // 36 + 5
        // Assert: latched 16 Levels chord was NOT corrupted or overwritten
        assert_eq!(PREVIEW_STATE.load(Ordering::Acquire) & 1, 1);
        assert_eq!(PREVIEW_STATE.load(Ordering::Acquire) & (1 << 5), 0);

        // 7. User hits physical Pad 9 (selects sample slot 9)
        observe_surface(surface(9, 0x99, 100));
        observe_surface(surface(9, 0x89, 0));
        assert_eq!(native_audio::active_sample_pad(), Some(9));
        assert_eq!(native_audio::active_sample_note(), 45); // 36 + 9

        // 8. Press STOP once: stops sequence only, background arpeggiator STILL PLAYS!
        assert_eq!(module.on_button(button_ev(81, true, false)), ButtonFlow::Forward);
        assert!(BACKGROUND_PLAYBACK_ACTIVE.load(Ordering::Acquire));
        assert!(module.enabled);
        assert_eq!(module.mode, EngineMode::Mutator);
        assert_ne!(PREVIEW_STATE.load(Ordering::Acquire) & HOLD_ENABLED, 0);

        // 9. Press STOP second time (double STOP): completely halts voices and resets engine!
        assert_eq!(module.on_button(button_ev(81, true, false)), ButtonFlow::Forward);
        assert!(!BACKGROUND_PLAYBACK_ACTIVE.load(Ordering::Acquire));
        assert!(!module.enabled);
        assert_eq!(module.mode, EngineMode::Off);
        assert_eq!(PREVIEW_STATE.load(Ordering::Acquire), 0);
        assert_eq!(native_audio::active_sample_pad(), None);

        lighting::reset_lighting();
    }

    #[test]
    fn live_sample_browsing_reentering_16_levels_resumes_seamlessly() {
        let _guard = INPUT_TEST_LOCK.lock().unwrap();
        LAST_STOP_MS.store(0, Ordering::Release);
        BACKGROUND_PLAYBACK_ACTIVE.store(false, Ordering::Release);
        lighting::reset_lighting();
        let mut module = ArpeggiGator::default();
        module.on_start();

        // 1. Enter 16 Levels, turn on ARP, enable HOLD, latch Pad 0
        module.on_button(button_ev(button::LEVELS, true, false));
        module.on_button(button_ev(BUTTON_B1, true, true));
        module.on_button(button_ev(BUTTON_B1, false, true));
        module.on_button(button_ev(button::DATA, true, false));
        module.on_button(button_ev(button::DATA, false, false));
        observe_surface(surface(0, 0x99, 100));
        observe_surface(surface(0, 0x89, 0));
        assert_eq!(module.mode, EngineMode::Arp);

        // 2. Exit to Sample screen
        module.on_button(button_ev(button::LEVELS, true, false));
        assert!(!module.is_16_levels_active());
        assert!(BACKGROUND_PLAYBACK_ACTIVE.load(Ordering::Acquire));

        // 3. Browse to Pad 3
        observe_surface(surface(3, 0x99, 100));
        observe_surface(surface(3, 0x89, 0));
        assert_eq!(native_audio::active_sample_pad(), Some(3));

        // 4. Re-enter 16 Levels
        module.on_button(button_ev(button::LEVELS, true, false));
        assert!(module.is_16_levels_active());
        assert!(!BACKGROUND_PLAYBACK_ACTIVE.load(Ordering::Acquire));
        assert!(module.enabled);
        assert_eq!(module.mode, EngineMode::Arp);

        // 5. Turn off HOLD via DATA wheel
        module.on_button(button_ev(button::DATA, true, false));
        module.on_button(button_ev(button::DATA, false, false));
        assert_eq!(PREVIEW_STATE.load(Ordering::Acquire) & HOLD_ENABLED, 0);

        // 6. Now exit 16 Levels: with HOLD disabled, engine resets cleanly
        module.on_button(button_ev(button::LEVELS, true, false));
        assert!(!module.is_16_levels_active());
        assert!(!BACKGROUND_PLAYBACK_ACTIVE.load(Ordering::Acquire));
        assert!(!module.enabled);
        assert_eq!(module.mode, EngineMode::Off);

        lighting::reset_lighting();
    }
}
