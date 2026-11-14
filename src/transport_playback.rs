//! Transport-grid playback model. Not connected to firmware until a reliable
//! transport position and its thread/lifetime contract have been verified.
#![allow(dead_code)]

use crate::{ArpEngine, Pattern, ScheduledEvent, StepRate};

/// The adapter must provide monotonic, unwrapped transport pulses, including
/// across loop boundaries. A seek or restart must reset this model.
pub(crate) struct TransportPlayback {
    engine: ArpEngine,
    desired: [Option<f32>; 16],
    press_order: [u64; 16],
    serial: u64,
    rate: StepRate,
    pulses_per_quarter: u64,
    next_boundary: Option<u64>,
    last_pulse: Option<u64>,
    active: Option<(u8, u64)>,
}

impl TransportPlayback {
    pub(crate) fn new(rate: StepRate, pulses_per_quarter: u64) -> Option<Self> {
        let (num, den) = rate.quarter_fraction();
        let numerator = pulses_per_quarter.checked_mul(num)?;
        if numerator == 0 || numerator % den != 0 {
            return None;
        }
        Some(Self {
            engine: ArpEngine::new(),
            desired: [None; 16],
            press_order: [0; 16],
            serial: 0,
            rate,
            pulses_per_quarter,
            next_boundary: None,
            last_pulse: None,
            active: None,
        })
    }

    /// Replace one desired pad state; the next grid boundary applies it.
    pub(crate) fn set_pad(&mut self, pad: u8, velocity: Option<f32>) {
        if let Some(slot) = self.desired.get_mut(usize::from(pad)) {
            if slot.is_none() && velocity.is_some() {
                self.serial = self.serial.saturating_add(1);
                self.press_order[usize::from(pad)] = self.serial;
            }
            *slot = velocity;
        }
    }

    pub(crate) fn set_pattern(&mut self, pattern: Pattern) {
        self.engine.set_pattern(pattern);
    }

    pub(crate) fn set_additional_octaves(&mut self, octaves: u8) {
        self.engine.set_additional_octaves(octaves);
    }

    pub(crate) fn set_rate(&mut self, rate: StepRate) -> Option<ScheduledEvent> {
        if self.rate == rate {
            return None;
        }
        let off = self.stop();
        self.rate = rate;
        off
    }

    /// Process one transport observation. NoteOff precedes NoteOn. A backwards
    /// position is rejected; the adapter must reset after transport seeks.
    pub(crate) fn tick(&mut self, pulse: u64) -> Option<[Option<ScheduledEvent>; 2]> {
        if self.last_pulse.is_some_and(|last| pulse < last) {
            return None;
        }
        self.last_pulse = Some(pulse);
        let (num, den) = self.rate.quarter_fraction();
        let step = self.pulses_per_quarter * num / den;
        let boundary = *self
            .next_boundary
            .get_or_insert_with(|| pulse.div_ceil(step).saturating_mul(step));
        let mut events = [None, None];
        if self.active.is_some_and(|(_, off)| pulse >= off) {
            events[0] = self
                .active
                .take()
                .map(|(pad, _)| ScheduledEvent::NoteOff { pad });
        }
        if pulse < boundary {
            return Some(events);
        }
        if events[0].is_none() {
            events[0] = self
                .active
                .take()
                .map(|(pad, _)| ScheduledEvent::NoteOff { pad });
        }
        self.engine
            .update_at_boundary(&self.desired, &self.press_order);
        if let Some(note) = self.engine.next_step() {
            let off = pulse.saturating_add((step / 2).max(1));
            self.active = Some((note.pad, off));
            let slot = usize::from(events[0].is_some());
            events[slot] = Some(ScheduledEvent::NoteOn(note));
        }
        self.next_boundary = Some((pulse / step).saturating_add(1).saturating_mul(step));
        Some(events)
    }

    /// Release any active voice when transport stops or the clock is reset.
    pub(crate) fn stop(&mut self) -> Option<ScheduledEvent> {
        self.next_boundary = None;
        self.last_pulse = None;
        self.engine.clear();
        self.active
            .take()
            .map(|(pad, _)| ScheduledEvent::NoteOff { pad })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn note(events: [Option<ScheduledEvent>; 2]) -> Option<(u8, u8)> {
        events.into_iter().flatten().find_map(|event| match event {
            ScheduledEvent::NoteOn(step) => Some((step.pad, step.transpose)),
            _ => None,
        })
    }

    #[test]
    fn pad_changes_take_effect_at_grid_boundaries_without_resetting_order() {
        let mut player = TransportPlayback::new(StepRate::Sixteenth, 960).unwrap();
        player.set_pad(0, Some(0.8));
        player.set_pad(1, Some(0.7));
        assert_eq!(note(player.tick(100).unwrap()), None);
        assert_eq!(note(player.tick(240).unwrap()), Some((0, 0)));
        player.set_pad(2, Some(0.6));
        assert_eq!(note(player.tick(300).unwrap()), None);
        assert_eq!(
            player.tick(360).unwrap()[0],
            Some(ScheduledEvent::NoteOff { pad: 0 })
        );
        assert_eq!(note(player.tick(480).unwrap()), Some((1, 0)));
        player.set_pad(1, None);
        assert_eq!(note(player.tick(720).unwrap()), Some((0, 0)));
    }

    #[test]
    fn stop_and_seek_do_not_leave_an_active_voice() {
        let mut player = TransportPlayback::new(StepRate::Sixteenth, 960).unwrap();
        player.set_pad(3, Some(1.0));
        assert_eq!(note(player.tick(0).unwrap()), Some((3, 0)));
        assert!(player.tick(10).is_some());
        assert!(player.tick(5).is_none());
        assert_eq!(player.stop(), Some(ScheduledEvent::NoteOff { pad: 3 }));
        assert_eq!(note(player.tick(0).unwrap()), Some((3, 0)));
    }

    #[test]
    fn played_pattern_preserves_order_of_pads_queued_before_a_boundary() {
        let mut player = TransportPlayback::new(StepRate::Sixteenth, 960).unwrap();
        player.set_pattern(Pattern::Played);
        player.set_pad(9, Some(1.0));
        player.set_pad(1, Some(0.7));
        assert_eq!(note(player.tick(0).unwrap()), Some((9, 0)));
        assert_eq!(note(player.tick(240).unwrap()), Some((1, 0)));
    }

    #[test]
    fn late_observations_do_not_burst_and_invalid_resolution_is_rejected() {
        assert!(TransportPlayback::new(StepRate::EighthTriplet, 100).is_none());
        let mut player = TransportPlayback::new(StepRate::Sixteenth, 960).unwrap();
        player.set_pad(0, Some(1.0));
        assert_eq!(note(player.tick(0).unwrap()), Some((0, 0)));
        let late = player.tick(800).unwrap();
        assert_eq!(late[0], Some(ScheduledEvent::NoteOff { pad: 0 }));
        assert_eq!(note(late), Some((0, 0)));
        assert_eq!(note(player.tick(801).unwrap()), None);
    }
}
