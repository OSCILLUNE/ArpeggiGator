//! ArpeggiGator — standalone arpeggiator mod for AKAI MPC Sample 1.3.0.12.
//!
//! The GUI prototype keeps native Sample and 16 Levels screens intact, replacing
//! softkey captions only while SHIFT is held. Generated pad retriggers remain
//! gated until input lifetimes and UI-thread dispatch are validated.

mod arp;
mod probe;
mod scheduler;
mod splash;
mod tempo;

pub use arp::{ArpEngine, Pattern, Step};
pub use scheduler::{ArpScheduler, ScheduledEvent, StepRate};

use core::sync::atomic::{AtomicBool, AtomicI64, Ordering};

use mpc_sample_sdk::{
    ButtonEvent, ButtonFlow, Mod, PadAction, PadEvent, PadFlow, PaintEvent, draw, event::button,
    platform::clock,
};

/// Confirmed control-surface values: B1=12, B2=13, B3=14.
const BUTTON_B1: u32 = 12;
const BUTTON_B2: u32 = 13;
const BUTTON_B3: u32 = 14;
static DASHBOARD_REFRESH_UNTIL_MS: AtomicI64 = AtomicI64::new(0);
static PAD_PREVIEW_ACTIVE: AtomicBool = AtomicBool::new(false);
static SHIFT_ACTIVE: AtomicBool = AtomicBool::new(false);

fn repaint_component() -> usize {
    let now = clock::now_ms();
    if now < DASHBOARD_REFRESH_UNTIL_MS.load(Ordering::Acquire)
        || PAD_PREVIEW_ACTIVE.load(Ordering::Acquire)
    {
        draw::painted()
    } else {
        0
    }
}

#[derive(Default)]
struct ArpeggiGator {
    engine: ArpEngine,
    enabled: bool,
    in_16_levels: bool,
    additional_notes: u8,
    consumed_buttons: u8,
}

impl Mod for ArpeggiGator {
    fn on_start(&mut self) {
        DASHBOARD_REFRESH_UNTIL_MS.store(0, Ordering::Release);
        PAD_PREVIEW_ACTIVE.store(false, Ordering::Release);
        SHIFT_ACTIVE.store(false, Ordering::Release);
        tempo::start_monitor();
        // SAFETY: repaint_component returns only the live component last
        // recorded by the SDK paint hook, or zero when none is available.
        unsafe { draw::start_repaint_pump(repaint_component) };
    }

    fn on_button(&mut self, event: ButtonEvent) -> ButtonFlow {
        let mut action = 0;
        if event.code == button::SHIFT {
            SHIFT_ACTIVE.store(event.pressed, Ordering::Release);
            DASHBOARD_REFRESH_UNTIL_MS.store(clock::now_ms() + 500, Ordering::Release);
        }

        let flow = if event.code == button::LEVELS && event.pressed {
            self.in_16_levels = !self.in_16_levels;
            action = if self.in_16_levels { 4 } else { 5 };
            if !self.in_16_levels {
                self.enabled = false;
                self.engine.clear();
                PAD_PREVIEW_ACTIVE.store(false, Ordering::Release);
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
            } else if !self.in_16_levels || !event.shift || !event.pressed {
                ButtonFlow::Forward
            } else if bit == 1 {
                self.consumed_buttons |= bit;
                self.enabled = !self.enabled;
                if !self.enabled {
                    self.engine.clear();
                    PAD_PREVIEW_ACTIVE.store(false, Ordering::Release);
                }
                action = if self.enabled { 1 } else { 2 };
                ButtonFlow::Consume
            } else if bit == 2 {
                self.consumed_buttons |= bit;
                self.engine.cycle_pattern();
                action = 3;
                ButtonFlow::Consume
            } else {
                self.consumed_buttons |= bit;
                self.additional_notes = (self.additional_notes + 1) % 3;
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
            self.in_16_levels,
            action,
        );
        flow
    }

    fn on_pad(&mut self, event: PadEvent) -> PadFlow {
        if !self.enabled || !self.in_16_levels {
            return PadFlow::Forward;
        }
        if let Ok(pad) = u8::try_from(event.pad) {
            match event.action {
                PadAction::Press => {
                    self.engine.press(pad, event.velocity);
                    PAD_PREVIEW_ACTIVE.store(self.engine.held_count() > 0, Ordering::Release);
                    probe::capture_and_schedule(event);
                }
                PadAction::Release => {
                    self.engine.release(pad);
                    PAD_PREVIEW_ACTIVE.store(self.engine.held_count() > 0, Ordering::Release);
                    DASHBOARD_REFRESH_UNTIL_MS.store(clock::now_ms() + 400, Ordering::Release);
                    probe::source_pad_released(event);
                }
            }
        }
        // Physical strikes remain audible. Scheduled retriggers will be
        // connected after the live input object and tempo clock are validated.
        PadFlow::Forward
    }

    fn on_pitch(&mut self, applied: i32) -> i32 {
        self.in_16_levels = true;
        DASHBOARD_REFRESH_UNTIL_MS.store(clock::now_ms() + 500, Ordering::Release);
        applied
    }

    fn on_paint(&mut self, graphics: &mut mpc_sample_sdk::draw::Graphics, event: PaintEvent) {
        if event.sample_page {
            probe::on_paint();
        }
        let shift_active = SHIFT_ACTIVE.load(Ordering::Acquire);
        let show_shift_menu = self.in_16_levels && shift_active;
        let show_mascot = self.in_16_levels && self.enabled;
        if !show_shift_menu && !show_mascot {
            return;
        }
        let now = clock::now_ms();
        // SAFETY: on_paint supplies the live Graphics object on the app's
        // paint path. The overlay only paints the three SHIFT labels and the
        // mascot after the user explicitly enables ARP in 16 Levels.
        unsafe {
            splash::draw_arp_overlay(
                graphics,
                show_shift_menu,
                self.enabled,
                self.engine.pattern().label(),
                self.additional_notes + 1,
                self.engine.held_count() as u8,
                now as u64,
            )
        }
    }
}

mpc_sample_sdk::export_mod!(ArpeggiGator);
