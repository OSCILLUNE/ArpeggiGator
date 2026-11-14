//! Host-testable arpeggiator playback state, independent of native audio.
//!
//! This layer owns only scalar pad state and timing. Its events are commands
//! for a future verified output adapter; it does not call firmware or MIDI.

use crate::{ArpEngine, ArpScheduler, Pattern, ScheduledEvent, StepRate};

/// Playback state and exact note-off handoff for a single arpeggiator.
#[derive(Default)]
pub struct Playback {
    engine: ArpEngine,
    scheduler: ArpScheduler,
    physical: u16,
    selected: u16,
    enabled: bool,
    hold: bool,
    bpm: Option<f32>,
}

impl Playback {
    /// Enable or disable playback, returning any note that must be released.
    pub fn set_enabled(&mut self, enabled: bool) -> Option<ScheduledEvent> {
        if self.enabled == enabled {
            return None;
        }
        self.enabled = enabled;
        if enabled {
            return None;
        }
        self.hold = false;
        self.clear()
    }

    /// Disable HOLD by clearing the latched notes and releasing any voice.
    pub fn set_hold(&mut self, hold: bool) -> Option<ScheduledEvent> {
        if !self.enabled || self.hold == hold {
            return None;
        }
        self.hold = hold;
        if hold { None } else { self.clear() }
    }

    /// Set a new pattern, restarting its sequence.
    pub fn set_pattern(&mut self, pattern: Pattern) {
        self.engine.set_pattern(pattern);
    }

    /// Append the given number of octave-transposed first notes.
    pub fn set_additional_octaves(&mut self, octaves: u8) {
        self.engine.set_additional_octaves(octaves);
    }

    /// Change division. A sounding note must be released before restarting.
    pub fn set_rate(&mut self, rate: StepRate) -> Option<ScheduledEvent> {
        if self.scheduler.rate() == rate {
            return None;
        }
        let off = self.scheduler.stop();
        self.bpm = None;
        let _ = self.scheduler.set_rate(rate);
        off
    }

    /// Handle one physical pad press, ignoring repeats while still down.
    pub fn press(&mut self, pad: u8, velocity: f32) -> Option<ScheduledEvent> {
        let bit = bit(pad)?;
        if self.physical & bit != 0 {
            return None;
        }
        self.physical |= bit;
        if !self.enabled {
            return None;
        }
        if self.hold && self.selected & bit != 0 {
            self.selected &= !bit;
            self.engine.release(pad);
            return self.stop_if_empty();
        }
        self.selected |= bit;
        self.engine.press(pad, velocity);
        None
    }

    /// Handle a physical release; HOLD retains selected pads.
    pub fn release(&mut self, pad: u8) -> Option<ScheduledEvent> {
        let bit = bit(pad)?;
        self.physical &= !bit;
        if !self.enabled || self.hold {
            return None;
        }
        self.selected &= !bit;
        self.engine.release(pad);
        self.stop_if_empty()
    }

    /// Active source-pad bit mask, including latched pads.
    #[must_use]
    pub const fn selected(&self) -> u16 {
        self.selected
    }

    /// Emit due commands. On BPM change, release the old voice before a new one.
    /// Invalid or unavailable BPM stops playback rather than inventing a tempo.
    pub fn tick(&mut self, now_us: u64, bpm: Option<f32>) -> [Option<ScheduledEvent>; 2] {
        if !self.enabled || self.selected == 0 {
            return [self.scheduler.stop(), None];
        }
        let valid_bpm = bpm.filter(|bpm| bpm.is_finite() && (20.0..=400.0).contains(bpm));
        let Some(bpm) = valid_bpm else {
            self.bpm = None;
            return [self.scheduler.stop(), None];
        };
        let mut old_off = None;
        if self.scheduler.is_running() && self.bpm != Some(bpm) {
            old_off = self.scheduler.stop();
        }
        if !self.scheduler.is_running() {
            if !self.scheduler.start(now_us, bpm) {
                return [old_off, None];
            }
            self.bpm = Some(bpm);
        }
        let events = self.scheduler.tick(now_us, &mut self.engine);
        if old_off.is_some() {
            // Restart begins with a NoteOn; preserve the preceding NoteOff.
            [old_off, events.into_iter().flatten().next()]
        } else {
            events
        }
    }

    fn stop_if_empty(&mut self) -> Option<ScheduledEvent> {
        if self.selected == 0 {
            self.bpm = None;
            self.scheduler.stop()
        } else {
            None
        }
    }

    fn clear(&mut self) -> Option<ScheduledEvent> {
        self.selected = 0;
        self.engine.clear();
        self.bpm = None;
        self.scheduler.stop()
    }
}

fn bit(pad: u8) -> Option<u16> {
    (pad < 16).then(|| 1_u16 << pad)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Step;

    #[test]
    fn hold_toggles_any_number_of_pads_and_releases_the_last_voice() {
        let mut p = Playback::default();
        p.set_enabled(true);
        p.set_hold(true);
        p.press(0, 0.8);
        p.release(0);
        p.press(1, 0.7);
        p.release(1);
        assert_eq!(p.selected(), 0b11);
        assert!(matches!(
            p.tick(0, Some(120.0))[0],
            Some(ScheduledEvent::NoteOn(_))
        ));
        p.press(1, 1.0);
        p.release(1);
        assert_eq!(p.selected(), 0b01);
        assert_eq!(p.press(0, 1.0), Some(ScheduledEvent::NoteOff { pad: 0 }));
        assert_eq!(p.selected(), 0);
        assert_eq!(p.tick(10, Some(120.0)), [None, None]);
    }

    #[test]
    fn disabling_hold_or_arp_always_returns_the_active_note_off() {
        for disable_hold in [false, true] {
            let mut p = Playback::default();
            p.set_enabled(true);
            p.set_hold(true);
            p.press(3, 0.6);
            p.release(3);
            let _ = p.tick(0, Some(120.0));
            let off = if disable_hold {
                p.set_hold(false)
            } else {
                p.set_enabled(false)
            };
            assert_eq!(off, Some(ScheduledEvent::NoteOff { pad: 3 }));
            assert_eq!(p.selected(), 0);
        }
    }

    #[test]
    fn tempo_change_releases_old_voice_before_restarting_and_invalid_tempo_stops() {
        let mut p = Playback::default();
        p.set_enabled(true);
        p.press(2, 0.5);
        let _ = p.tick(0, Some(120.0));
        let changed = p.tick(1_000, Some(90.0));
        assert_eq!(changed[0], Some(ScheduledEvent::NoteOff { pad: 2 }));
        assert!(matches!(changed[1], Some(ScheduledEvent::NoteOn(_))));
        assert_eq!(
            p.tick(2_000, None),
            [Some(ScheduledEvent::NoteOff { pad: 2 }), None]
        );
        assert_eq!(p.tick(3_000, None), [None, None]);
    }

    #[test]
    fn two_pads_follow_the_selected_grid_and_pair_every_note() {
        let mut p = Playback::default();
        p.set_enabled(true);
        p.set_rate(StepRate::Eighth);
        p.press(0, 0.8);
        p.press(2, 0.7);
        assert!(matches!(
            p.tick(0, Some(120.0))[0],
            Some(ScheduledEvent::NoteOn(Step { pad: 0, .. }))
        ));
        assert_eq!(
            p.tick(125_000, Some(120.0)),
            [Some(ScheduledEvent::NoteOff { pad: 0 }), None]
        );
        assert!(matches!(
            p.tick(250_000, Some(120.0))[0],
            Some(ScheduledEvent::NoteOn(Step { pad: 2, .. }))
        ));
        p.release(0);
        assert_eq!(p.release(2), Some(ScheduledEvent::NoteOff { pad: 2 }));
        assert_eq!(p.tick(300_000, Some(120.0)), [None, None]);
    }

    #[test]
    fn changing_rate_stops_current_voice_and_restarts_on_next_tick() {
        let mut p = Playback::default();
        p.set_enabled(true);
        p.press(5, 0.5);
        let _ = p.tick(0, Some(120.0));
        assert_eq!(
            p.set_rate(StepRate::Eighth),
            Some(ScheduledEvent::NoteOff { pad: 5 })
        );
        assert!(matches!(
            p.tick(100, Some(120.0))[0],
            Some(ScheduledEvent::NoteOn(_))
        ));
    }
}
