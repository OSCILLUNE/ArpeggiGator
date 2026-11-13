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

use core::sync::atomic::{AtomicBool, AtomicI64, AtomicU32, Ordering};

use mpc_sample_sdk::{
    ButtonEvent, ButtonFlow, Mod, PadAction, PadEvent, PadFlow, PaintEvent, SurfaceEvent, draw,
    event::button, platform::clock,
};

/// Confirmed control-surface values: B1=12, B2=13, B3=14.
const BUTTON_B1: u32 = 12;
const BUTTON_B2: u32 = 13;
const BUTTON_B3: u32 = 14;
static DASHBOARD_REFRESH_UNTIL_MS: AtomicI64 = AtomicI64::new(0);
const PREVIEW_ENABLED: u32 = 1 << 16;
const PAD_MASK: u32 = PREVIEW_ENABLED - 1;
static PREVIEW_STATE: AtomicU32 = AtomicU32::new(0);
static SHIFT_ACTIVE: AtomicBool = AtomicBool::new(false);

fn repaint_component() -> usize {
    let now = clock::now_ms();
    if now < DASHBOARD_REFRESH_UNTIL_MS.load(Ordering::Acquire)
        || PREVIEW_STATE.load(Ordering::Acquire) & PAD_MASK != 0
    {
        draw::painted()
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
            let _ = PREVIEW_STATE.fetch_update(Ordering::AcqRel, Ordering::Acquire, |state| {
                (state & PREVIEW_ENABLED != 0).then_some(state | bit)
            });
        }
        (0x80, _) | (0x90, 0) => {
            let previous =
                PREVIEW_STATE.fetch_update(Ordering::AcqRel, Ordering::Acquire, |state| {
                    (state & PREVIEW_ENABLED != 0).then_some(state & !bit)
                });
            if let Ok(previous) = previous
                && previous & PAD_MASK == bit
            {
                DASHBOARD_REFRESH_UNTIL_MS.store(clock::now_ms() + 400, Ordering::Release);
            }
        }
        _ => {}
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
        PREVIEW_STATE.store(0, Ordering::Release);
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
                PREVIEW_STATE.store(0, Ordering::Release);
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
                    PREVIEW_STATE.store(0, Ordering::Release);
                }
                if self.enabled {
                    PREVIEW_STATE.store(PREVIEW_ENABLED, Ordering::Release);
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
                self.additional_notes = (self.additional_notes + 1) % 4;
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

    fn surface_observer(&self) -> Option<fn(SurfaceEvent)> {
        Some(observe_surface)
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
                self.additional_notes,
                (PREVIEW_STATE.load(Ordering::Acquire) & PAD_MASK).count_ones() as u8,
                now as u64,
            )
        }
    }
}

mpc_sample_sdk::export_mod!(ArpeggiGator);

#[cfg(test)]
mod animation_input_tests {
    use super::*;

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
    fn animation_tracks_physical_pads_through_release_and_mode_changes() {
        PREVIEW_STATE.store(0, Ordering::Release);
        observe_surface(surface(0, 0x99, 100));
        assert_eq!(PREVIEW_STATE.load(Ordering::Acquire) & PAD_MASK, 0);

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
        PREVIEW_STATE.store(0, Ordering::Release);
    }
}
