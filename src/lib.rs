//! ArpeggiGator — standalone arpeggiator mod for AKAI MPC Sample 1.3.0.12.
//!
//! The first development increment wires native controls and the startup mark,
//! and adds host-testable pattern and timing engines. Automatic pad triggering
//! remains gated until live-input lifetime and UI-thread dispatch are validated.

mod arp;
mod probe;
mod scheduler;
mod splash;
mod tempo;

pub use arp::{ArpEngine, Pattern, Step};
pub use scheduler::{ArpScheduler, ScheduledEvent, StepRate};

use core::sync::atomic::{AtomicI64, Ordering};

use mpc_sample_sdk::{
    ButtonEvent, ButtonFlow, Mod, PadAction, PadEvent, PadFlow, PaintEvent, draw, event::button,
    platform::clock,
};

/// Confirmed control-surface values: B1=12, B2=13, B3=14.
const BUTTON_B1: u32 = 12;
const BUTTON_B2: u32 = 13;
static SPLASH_UNTIL_MS: AtomicI64 = AtomicI64::new(0);

fn splash_component() -> usize {
    if clock::now_ms() < SPLASH_UNTIL_MS.load(Ordering::Acquire) {
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
}

impl Mod for ArpeggiGator {
    fn on_start(&mut self) {
        SPLASH_UNTIL_MS.store(clock::now_ms() + 2_200, Ordering::Release);
        tempo::start_monitor();
        draw::show_toast("ArpeggiGator");
        // SAFETY: splash_component returns only the live component last
        // recorded by the SDK paint hook, or zero when none is available.
        unsafe { draw::start_repaint_pump(splash_component) };
    }

    fn on_button(&mut self, event: ButtonEvent) -> ButtonFlow {
        if event.code == button::LEVELS && event.pressed {
            self.in_16_levels = !self.in_16_levels;
            if !self.in_16_levels {
                self.enabled = false;
                self.engine.clear();
            }
            return ButtonFlow::Forward;
        }

        if !self.in_16_levels || !event.shift || !event.pressed {
            return ButtonFlow::Forward;
        }

        match event.code {
            BUTTON_B1 => {
                self.enabled = !self.enabled;
                if !self.enabled {
                    self.engine.clear();
                }
                draw::show_toast(if self.enabled {
                    "ArpeggiGator: ARP ON"
                } else {
                    "ArpeggiGator: ARP OFF"
                });
                ButtonFlow::Consume
            }
            BUTTON_B2 => {
                let pattern = self.engine.cycle_pattern();
                draw::show_toast(pattern.label());
                ButtonFlow::Consume
            }
            _ => ButtonFlow::Forward,
        }
    }

    fn on_pad(&mut self, event: PadEvent) -> PadFlow {
        if !self.enabled || !self.in_16_levels {
            return PadFlow::Forward;
        }
        if let Ok(pad) = u8::try_from(event.pad) {
            match event.action {
                PadAction::Press => {
                    self.engine.press(pad, event.velocity);
                    probe::capture_and_schedule(event);
                }
                PadAction::Release => {
                    self.engine.release(pad);
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
        applied
    }

    fn on_paint(&mut self, graphics: &mut mpc_sample_sdk::draw::Graphics, event: PaintEvent) {
        if !event.sample_page {
            return;
        }
        // The probe runs only on experiment/repaint-dispatch-probe.
        probe::on_paint();
        if clock::now_ms() < SPLASH_UNTIL_MS.load(Ordering::Acquire) {
            // SAFETY: on_paint supplies the live Graphics object on the app's
            // paint path.
            unsafe { splash::draw(graphics) };
            return;
        }
        if self.enabled && self.in_16_levels {
            let tempo = tempo::current_bpm()
                .map(|bpm| format!("{bpm:.1}"))
                .unwrap_or_else(|| "--.-".to_string());
            let line = format!("ARP {} {} BPM", self.engine.pattern().label(), tempo);
            // SAFETY: this is the live Graphics handle from on_paint.
            unsafe {
                graphics.colour(0xdd12072b);
                graphics.rect(206, 8, 106, 20);
                graphics.colour(0xffffd36b);
                graphics.text(&line, 210, 11, 100, 14, 8.0);
            }
        }
    }
}

mpc_sample_sdk::export_mod!(ArpeggiGator);
