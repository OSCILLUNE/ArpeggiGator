//! Host-testable arpeggiator playback state, independent of native audio.
//!
//! This layer owns only scalar pad state and timing. Its events are commands
//! for a future verified output adapter; it does not call firmware or MIDI.

use crate::{
    ArpEngine, ArpScheduler, BarLength, ChordMood, MelodyEngine, MelodyVariant, MutatorEngine,
    MutationIntensity, OrbitAtmosphere, OrbitDynamics, OrbitatorEngine, Pattern, ScheduledEvent,
    StepRate, VoicingStyle, AcordatorEngine,
};

/// High-level engine mode: disabled, arpeggiator, melodic phrase sequencer, or generative modes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(usize)]
pub enum EngineMode {
    #[default]
    Off = 0,
    Arp = 1,
    Melody = 2,
    Mutator = 3,
    Acordator = 4,
    Orbitator = 5,
}

/// Playback state and exact note-off handoff for ARP, MELODY, and generative engines.
#[derive(Default)]
pub struct Playback {
    arp_engine: ArpEngine,
    melody_engine: MelodyEngine,
    mutator_engine: MutatorEngine,
    acordator_engine: AcordatorEngine,
    orbitator_engine: OrbitatorEngine,
    scheduler: ArpScheduler,
    physical: u16,
    selected: u16,
    mode: EngineMode,
    hold: bool,
    bpm: Option<f32>,
}

impl Playback {
    /// Set the active engine mode, releasing any sounding voice on transition.
    pub fn set_mode(&mut self, mode: EngineMode) -> Option<ScheduledEvent> {
        if self.mode == mode {
            return None;
        }
        let old_mode = self.mode;
        self.mode = mode;
        if mode == EngineMode::Off {
            self.hold = false;
            self.clear()
        } else if old_mode != EngineMode::Off {
            self.clear()
        } else {
            None
        }
    }

    /// Enable or disable playback (compat with ARP-only callers).
    pub fn set_enabled(&mut self, enabled: bool) -> Option<ScheduledEvent> {
        self.set_mode(if enabled {
            EngineMode::Arp
        } else {
            EngineMode::Off
        })
    }

    /// Current engine mode.
    #[must_use]
    pub const fn mode(&self) -> EngineMode {
        self.mode
    }

    /// Whether HOLD is currently active.
    #[must_use]
    pub const fn hold(&self) -> bool {
        self.hold
    }

    /// Disable HOLD by clearing the latched notes and releasing any voice.
    pub fn set_hold(&mut self, hold: bool) -> Option<ScheduledEvent> {
        if self.mode == EngineMode::Off || self.hold == hold {
            return None;
        }
        self.hold = hold;
        if hold { None } else { self.clear() }
    }

    /// Set a new pattern, restarting its sequence.
    pub fn set_pattern(&mut self, pattern: Pattern) {
        self.arp_engine.set_pattern(pattern);
    }

    /// Append the given number of octave-transposed first notes.
    pub fn set_additional_octaves(&mut self, octaves: u8) {
        self.arp_engine.set_additional_octaves(octaves);
    }

    /// Set phrase length for the MELODY engine.
    pub fn set_melody_bars(&mut self, bars: BarLength) {
        self.melody_engine.set_bars(bars);
    }

    /// Set melody preset for the MELODY engine.
    pub fn set_melody_variant(&mut self, variant: MelodyVariant) {
        self.melody_engine.set_variant(variant);
    }

    /// Set mutation intensity for the MUTATOR engine.
    pub fn set_mutator_intensity(&mut self, intensity: MutationIntensity) {
        self.mutator_engine.set_intensity(intensity);
    }

    /// Set base melody preset for the MUTATOR engine.
    pub fn set_mutator_variant(&mut self, variant: MelodyVariant) {
        self.mutator_engine.set_variant(variant);
    }

    /// Set phrase length for the MUTATOR engine.
    pub fn set_mutator_bars(&mut self, bars: BarLength) {
        self.mutator_engine.set_bars(bars);
    }

    /// Freeze or unfreeze the MUTATOR engine.
    pub fn set_mutator_frozen(&mut self, frozen: bool) {
        self.mutator_engine.set_frozen(frozen);
    }

    /// Set chord mood for the ACORDATOR engine.
    pub fn set_chord_mood(&mut self, mood: ChordMood) {
        self.acordator_engine.set_mood(mood);
    }

    /// Set voicing style for the ACORDATOR engine.
    pub fn set_voicing_style(&mut self, style: VoicingStyle) {
        self.acordator_engine.set_style(style);
    }

    /// Set atmosphere for the ORBITATOR engine.
    pub fn set_orbit_atmosphere(&mut self, atmosphere: OrbitAtmosphere) {
        self.orbitator_engine.set_atmosphere(atmosphere);
    }

    /// Set dynamics for the ORBITATOR engine.
    pub fn set_orbit_dynamics(&mut self, dynamics: OrbitDynamics) {
        self.orbitator_engine.set_dynamics(dynamics);
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
        // Reject presses while the engine is Off BEFORE latching the physical
        // bit. Otherwise a press during a SHIFT+B1 mode jump leaves an orphaned
        // bit that suppresses the next press of the same pad (audio silent,
        // animation still running). Verified by the 0.0.7.1 regression tests.
        if self.mode == EngineMode::Off {
            return None;
        }
        if self.physical & bit != 0 {
            return None;
        }
        self.physical |= bit;
        if self.mode == EngineMode::Arp {
            if self.hold && self.selected & bit != 0 {
                self.selected &= !bit;
                self.arp_engine.release(pad);
                return self.stop_if_empty();
            }
            self.selected |= bit;
            self.arp_engine.press(pad, velocity);
            None
        } else if self.mode == EngineMode::Melody {
            // In MELODY mode: monophonic voice-stealing / retrigger from this root pad.
            if self.hold && self.selected == bit {
                self.selected = 0;
                self.melody_engine.clear();
                return self.stop_if_empty();
            }
            let off = if self.selected != 0 {
                self.scheduler.stop()
            } else {
                None
            };
            self.selected = bit;
            self.melody_engine.press(pad, velocity);
            off
        } else if self.mode == EngineMode::Mutator {
            // In MUTATOR mode: single-pad latch with voice stealing and reset to motif.
            if self.hold && self.selected == bit {
                self.selected = 0;
                self.mutator_engine.clear();
                return self.stop_if_empty();
            }
            let off = if self.selected != 0 {
                self.scheduler.stop()
            } else {
                None
            };
            self.selected = bit;
            self.mutator_engine.press(pad, velocity);
            off
        } else if self.mode == EngineMode::Acordator {
            // In ACORDATOR mode: single-pad chord latch with voice stealing.
            if self.hold && self.selected == bit {
                self.selected = 0;
                self.acordator_engine.clear();
                return self.stop_if_empty();
            }
            let off = if self.selected != 0 {
                self.scheduler.stop()
            } else {
                None
            };
            self.selected = bit;
            self.acordator_engine.press(pad, velocity);
            off
        } else if self.mode == EngineMode::Orbitator {
            // In ORBITATOR mode: single-pad orbit center with voice stealing.
            if self.hold && self.selected == bit {
                self.selected = 0;
                self.orbitator_engine.clear();
                return self.stop_if_empty();
            }
            let off = if self.selected != 0 {
                self.scheduler.stop()
            } else {
                None
            };
            self.selected = bit;
            self.orbitator_engine.press(pad, velocity);
            off
        } else {
            None
        }
    }

    /// Handle a physical release; HOLD retains selected pads.
    pub fn release(&mut self, pad: u8) -> Option<ScheduledEvent> {
        let bit = bit(pad)?;
        self.physical &= !bit;
        if self.mode == EngineMode::Off || self.hold {
            return None;
        }
        if self.mode == EngineMode::Arp {
            self.selected &= !bit;
            self.arp_engine.release(pad);
            self.stop_if_empty()
        } else if self.mode == EngineMode::Melody {
            if self.selected == bit {
                self.selected = 0;
                self.melody_engine.release(pad);
                self.stop_if_empty()
            } else {
                None
            }
        } else if self.mode == EngineMode::Mutator {
            if self.selected == bit {
                self.selected = 0;
                self.mutator_engine.release(pad);
                self.stop_if_empty()
            } else {
                None
            }
        } else if self.mode == EngineMode::Acordator {
            if self.selected == bit {
                self.selected = 0;
                self.acordator_engine.release(pad);
                self.stop_if_empty()
            } else {
                None
            }
        } else if self.mode == EngineMode::Orbitator {
            if self.selected == bit {
                self.selected = 0;
                self.orbitator_engine.release(pad);
                self.stop_if_empty()
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Active source-pad bit mask, including latched pads.
    #[must_use]
    pub const fn selected(&self) -> u16 {
        self.selected
    }

    /// Emit due commands. On BPM change, release the old voice before a new one.
    /// Invalid or unavailable BPM stops playback rather than inventing a tempo.
    pub fn tick(&mut self, now_us: u64, bpm: Option<f32>) -> [Option<ScheduledEvent>; 2] {
        if self.mode == EngineMode::Off || self.selected == 0 {
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
        let events = match self.mode {
            EngineMode::Arp => self.scheduler.tick(now_us, &mut self.arp_engine),
            EngineMode::Melody => self.scheduler.tick(now_us, &mut self.melody_engine),
            EngineMode::Mutator => self.scheduler.tick(now_us, &mut self.mutator_engine),
            EngineMode::Acordator => self.scheduler.tick(now_us, &mut self.acordator_engine),
            EngineMode::Orbitator => self.scheduler.tick(now_us, &mut self.orbitator_engine),
            _ => [None, None],
        };
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
        // A mode jump (set_mode -> clear) must also drop physical latch state.
        // Without this, a pad held through the jump stays "down" in Playback
        // even after the audio side reset its own inputs, so the first press
        // after entering Mutator/Acordator is swallowed.
        self.physical = 0;
        self.arp_engine.clear();
        self.melody_engine.clear();
        self.mutator_engine.clear();
        self.acordator_engine.clear();
        self.orbitator_engine.clear();
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

    #[test]
    fn melody_mode_advances_and_plays_steps() {
        let mut p = Playback::default();
        p.set_mode(EngineMode::Melody);
        p.set_melody_bars(BarLength::Bars2);
        p.set_melody_variant(MelodyVariant::Mel1);
        p.press(4, 0.8);
        assert_eq!(p.selected(), 1 << 4);

        let first = p.tick(0, Some(120.0));
        assert!(matches!(first[0], Some(ScheduledEvent::NoteOn(Step { pad: 4, .. }))));

        // Releasing stops in non-hold mode
        assert_eq!(p.release(4), Some(ScheduledEvent::NoteOff { pad: 4 }));
        assert_eq!(p.selected(), 0);
    }

    #[test]
    fn melody_mode_hold_switches_active_pad() {
        let mut p = Playback::default();
        p.set_mode(EngineMode::Melody);
        p.set_hold(true);

        p.press(1, 0.8);
        p.release(1);
        assert_eq!(p.selected(), 1 << 1);

        let _ = p.tick(0, Some(120.0));

        // Hitting another pad switches to that pad and turns off previous
        let off = p.press(8, 0.9);
        assert_eq!(off, Some(ScheduledEvent::NoteOff { pad: 1 }));
        assert_eq!(p.selected(), 1 << 8);
        p.release(8);

        // Hitting the same pad again toggles it off
        let toggle_off = p.press(8, 0.9);
        assert_eq!(p.selected(), 0);
        assert!(toggle_off.is_none());
    }

    #[test]
    fn mutator_mode_advances_and_plays_steps() {
        let mut p = Playback::default();
        p.set_mode(EngineMode::Mutator);
        p.set_mutator_intensity(MutationIntensity::Subtle);
        p.set_mutator_variant(MelodyVariant::Mel1);
        p.press(2, 0.85);
        assert_eq!(p.selected(), 1 << 2);

        let first = p.tick(0, Some(120.0));
        assert!(matches!(first[0], Some(ScheduledEvent::NoteOn(Step { pad: 2, .. }))));

        assert_eq!(p.release(2), Some(ScheduledEvent::NoteOff { pad: 2 }));
        assert_eq!(p.selected(), 0);
    }

    #[test]
    fn mutator_mode_hold_switches_active_pad() {
        let mut p = Playback::default();
        p.set_mode(EngineMode::Mutator);
        p.set_hold(true);

        p.press(3, 0.8);
        p.release(3);
        assert_eq!(p.selected(), 1 << 3);

        let _ = p.tick(0, Some(120.0));

        let off = p.press(7, 0.9);
        assert_eq!(off, Some(ScheduledEvent::NoteOff { pad: 3 }));
        assert_eq!(p.selected(), 1 << 7);
        p.release(7);

        let toggle_off = p.press(7, 0.9);
        assert_eq!(p.selected(), 0);
        assert!(toggle_off.is_none());
    }

    #[test]
    fn acordator_mode_advances_and_plays_steps() {
        let mut p = Playback::default();
        p.set_mode(EngineMode::Acordator);
        p.set_chord_mood(ChordMood::Warm);
        p.set_voicing_style(VoicingStyle::Strum);
        p.press(1, 0.9);
        assert_eq!(p.selected(), 1 << 1);

        let first = p.tick(0, Some(120.0));
        assert!(matches!(first[0], Some(ScheduledEvent::NoteOn(Step { pad: 1, transpose: 0, .. }))));

        assert_eq!(p.release(1), Some(ScheduledEvent::NoteOff { pad: 1 }));
        assert_eq!(p.selected(), 0);
    }

    #[test]
    fn hold_persists_across_mode_changes_until_turned_off() {
        let mut p = Playback::default();
        p.set_mode(EngineMode::Arp);
        assert!(!p.hold());

        // Enable HOLD in Arp
        assert_eq!(p.set_hold(true), None);
        assert!(p.hold());

        // Cycle through all active modes: HOLD must persist
        for mode in [
            EngineMode::Melody,
            EngineMode::Mutator,
            EngineMode::Acordator,
            EngineMode::Orbitator,
        ] {
            p.set_mode(mode);
            assert!(p.hold(), "HOLD should persist in mode {:?}", mode);
        }

        // Switching to Off resets HOLD to false
        p.set_mode(EngineMode::Off);
        assert!(!p.hold());

        // Re-entering Arp starts with HOLD off
        p.set_mode(EngineMode::Arp);
        assert!(!p.hold());

        // While in Off, set_hold is rejected
        p.set_mode(EngineMode::Off);
        assert_eq!(p.set_hold(true), None);
        assert!(!p.hold());
    }

    #[test]
    fn orbitator_mode_advances_and_plays_steps() {
        let mut p = Playback::default();
        p.set_mode(EngineMode::Orbitator);
        p.set_orbit_atmosphere(OrbitAtmosphere::Space);
        p.set_orbit_dynamics(OrbitDynamics::Calm);
        p.press(4, 0.85);
        assert_eq!(p.selected(), 1 << 4);

        let first = p.tick(0, Some(120.0));
        assert!(matches!(first[0], Some(ScheduledEvent::NoteOn(Step { pad: 4, .. }))));

        assert_eq!(p.release(4), Some(ScheduledEvent::NoteOff { pad: 4 }));
        assert_eq!(p.selected(), 0);
    }

    /// Regression for "first pad after a SHIFT+B1 menu jump: animation moves
    /// but no sound". A press received while the engine is Off must not latch
    /// `physical`, because Off -> Mutator/Acordator does not call `clear()`.
    #[test]
    fn press_while_off_does_not_swallow_first_press_after_mode_jump() {
        for mode in [EngineMode::Mutator, EngineMode::Acordator] {
            let mut p = Playback::default();
            assert_eq!(p.mode, EngineMode::Off);
            p.set_hold(true);

            p.press(3, 0.8); // pad event during the menu jump, engine still Off
            p.set_mode(mode);

            p.press(3, 0.8); // first real press must produce a note
            assert_ne!(
                p.selected() & (1 << 3),
                0,
                "first press after mode jump was swallowed"
            );
            assert!(matches!(
                p.tick(0, Some(120.0))[0],
                Some(ScheduledEvent::NoteOn(_))
            ));
        }
    }

    /// Regression for the other half: a latched pad must be unlatched when
    /// the engine jumps through Off (set_mode -> clear()).
    #[test]
    fn mode_jump_through_off_unlatches_physical_pad() {
        let mut p = Playback::default();
        p.set_enabled(true);
        p.press(3, 0.8);
        p.set_enabled(false); // -> Off -> clear() must drop the latch
        p.set_mode(EngineMode::Mutator);

        p.press(3, 0.8);
        assert_ne!(p.selected() & (1 << 3), 0, "stale physical bit survived Off");
        assert!(matches!(
            p.tick(0, Some(120.0))[0],
            Some(ScheduledEvent::NoteOn(_))
        ));
    }
}
