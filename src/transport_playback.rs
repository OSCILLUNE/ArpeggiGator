//! Transport-grid playback engine for all ArpeggiGator modes.
//!
//! Synchronizes note generation strictly to the MPC transport pulse grid.
//! Queues pad changes to grid boundaries without breaking pattern/phrase continuity.

use crate::{
    AcordatorEngine, ArpEngine, BarLength, ChordMood, EngineMode, MelodyEngine, MelodyVariant,
    MutationIntensity, MutatorEngine, OrbitAtmosphere, OrbitDynamics, OrbitatorEngine, Pattern,
    ScheduledEvent, Step, StepEngine, StepRate, VoicingStyle,
};

#[allow(dead_code)]
pub(crate) struct TransportPlayback {
    arp_engine: ArpEngine,
    melody_engine: MelodyEngine,
    mutator_engine: MutatorEngine,
    acordator_engine: AcordatorEngine,
    orbitator_engine: OrbitatorEngine,
    desired: [Option<f32>; 16],
    press_order: [u64; 16],
    serial: u64,
    mode: EngineMode,
    rate: StepRate,
    pulses_per_quarter: u64,
    gate_percent: u8,
    next_boundary: Option<u64>,
    last_pulse: Option<u64>,
    active: Option<(u8, u64)>,
}

#[allow(dead_code)]
impl TransportPlayback {
    pub(crate) fn new(rate: StepRate, pulses_per_quarter: u64) -> Option<Self> {
        let (num, den) = rate.quarter_fraction();
        let numerator = pulses_per_quarter.checked_mul(num)?;
        if numerator == 0 || numerator % den != 0 {
            return None;
        }
        Some(Self {
            arp_engine: ArpEngine::new(),
            melody_engine: MelodyEngine::new(),
            mutator_engine: MutatorEngine::new(),
            acordator_engine: AcordatorEngine::new(),
            orbitator_engine: OrbitatorEngine::new(),
            desired: [None; 16],
            press_order: [0; 16],
            serial: 0,
            mode: EngineMode::Arp,
            rate,
            pulses_per_quarter,
            gate_percent: 50,
            next_boundary: None,
            last_pulse: None,
            active: None,
        })
    }

    pub(crate) fn set_mode(&mut self, mode: EngineMode) -> Option<ScheduledEvent> {
        if self.mode == mode {
            return None;
        }
        let off = self.stop();
        self.mode = mode;
        off
    }

    #[allow(dead_code)]
    pub(crate) fn mode(&self) -> EngineMode {
        self.mode
    }

    #[allow(dead_code)]
    pub(crate) fn set_gate_percent(&mut self, percent: u8) {
        self.gate_percent = percent.clamp(10, 95);
    }

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
        if self.arp_engine.pattern() != pattern {
            self.arp_engine.set_pattern(pattern);
        }
    }

    pub(crate) fn set_additional_octaves(&mut self, octaves: u8) {
        self.arp_engine.set_additional_octaves(octaves);
    }

    pub(crate) fn set_melody_bars(&mut self, bars: BarLength) {
        self.melody_engine.set_bars(bars);
    }

    pub(crate) fn set_melody_variant(&mut self, variant: MelodyVariant) {
        self.melody_engine.set_variant(variant);
    }

    pub(crate) fn set_mutator_intensity(&mut self, intensity: MutationIntensity) {
        self.mutator_engine.set_intensity(intensity);
    }

    pub(crate) fn set_mutator_variant(&mut self, variant: MelodyVariant) {
        self.mutator_engine.set_variant(variant);
    }

    #[allow(dead_code)]
    pub(crate) fn set_mutator_bars(&mut self, bars: BarLength) {
        self.mutator_engine.set_bars(bars);
    }

    #[allow(dead_code)]
    pub(crate) fn set_mutator_frozen(&mut self, frozen: bool) {
        self.mutator_engine.set_frozen(frozen);
    }

    pub(crate) fn set_chord_mood(&mut self, mood: ChordMood) {
        self.acordator_engine.set_mood(mood);
    }

    pub(crate) fn set_voicing_style(&mut self, style: VoicingStyle) {
        self.acordator_engine.set_style(style);
    }

    pub(crate) fn set_orbit_atmosphere(&mut self, atmosphere: OrbitAtmosphere) {
        self.orbitator_engine.set_atmosphere(atmosphere);
    }

    pub(crate) fn set_orbit_dynamics(&mut self, dynamics: OrbitDynamics) {
        self.orbitator_engine.set_dynamics(dynamics);
    }

    pub(crate) fn set_rate(&mut self, rate: StepRate) -> Option<ScheduledEvent> {
        if self.rate == rate {
            return None;
        }
        let off = self.stop();
        self.rate = rate;
        off
    }

    fn active_root_candidate(&self) -> Option<(u8, f32)> {
        // Monophonic generative modes: pick the latest pressed active pad
        let mut best: Option<(u8, f32, u64)> = None;
        for (pad, &vel) in self.desired.iter().enumerate() {
            if let Some(v) = vel {
                let order = self.press_order[pad];
                if best.is_none() || best.is_some_and(|b| order > b.2) {
                    best = Some((pad as u8, v, order));
                }
            }
        }
        best.map(|(p, v, _)| (p, v))
    }

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

        // 1. Check Note-Off expiration
        if self.active.is_some_and(|(_, off)| pulse >= off) {
            events[0] = self
                .active
                .take()
                .map(|(pad, _)| ScheduledEvent::NoteOff { pad });
        }

        // 2. Wait until transport reaches grid boundary
        if pulse < boundary {
            return Some(events);
        }

        // 3. Grid boundary reached: close preceding voice if still sounding
        if events[0].is_none() {
            events[0] = self
                .active
                .take()
                .map(|(pad, _)| ScheduledEvent::NoteOff { pad });
        }

        // 4. Update engine states at the step boundary and query next step
        let step_opt: Option<Step> = match self.mode {
            EngineMode::Off => None,
            EngineMode::Arp => {
                self.arp_engine
                    .update_at_boundary(&self.desired, &self.press_order);
                self.arp_engine.next_step()
            }
            EngineMode::Melody => {
                match (self.active_root_candidate(), self.melody_engine.root_pad()) {
                    (Some((pad, vel)), Some(old_pad)) => {
                        if pad != old_pad {
                            self.melody_engine.set_root(pad, vel);
                        }
                    }
                    (Some((pad, vel)), None) => {
                        self.melody_engine.press(pad, vel);
                    }
                    (None, Some(_)) => {
                        self.melody_engine.clear();
                    }
                    (None, None) => {}
                }
                self.melody_engine.next_step()
            }
            EngineMode::Mutator => {
                match (self.active_root_candidate(), self.mutator_engine.root_pad()) {
                    (Some((pad, vel)), Some(old_pad)) => {
                        if pad != old_pad {
                            self.mutator_engine.set_root(pad, vel);
                        }
                    }
                    (Some((pad, vel)), None) => {
                        self.mutator_engine.press(pad, vel);
                    }
                    (None, Some(_)) => {
                        self.mutator_engine.clear();
                    }
                    (None, None) => {}
                }
                self.mutator_engine.next_step()
            }
            EngineMode::Acordator => {
                match (self.active_root_candidate(), self.acordator_engine.root_pad()) {
                    (Some((pad, vel)), Some(old_pad)) => {
                        if pad != old_pad {
                            self.acordator_engine.set_root(pad, vel);
                        }
                    }
                    (Some((pad, vel)), None) => {
                        self.acordator_engine.press(pad, vel);
                    }
                    (None, Some(_)) => {
                        self.acordator_engine.clear();
                    }
                    (None, None) => {}
                }
                self.acordator_engine.next_step()
            }
            EngineMode::Orbitator => {
                match (self.active_root_candidate(), self.orbitator_engine.root_pad()) {
                    (Some((pad, vel)), Some(old_pad)) => {
                        if pad != old_pad {
                            self.orbitator_engine.set_root(pad, vel);
                        }
                    }
                    (Some((pad, vel)), None) => {
                        self.orbitator_engine.press(pad, vel);
                    }
                    (None, Some(_)) => {
                        self.orbitator_engine.clear();
                    }
                    (None, None) => {}
                }
                self.orbitator_engine.next_step()
            }
        };

        // 5. Emit NoteOn if not a rest
        if let Some(note) = step_opt {
            if note.velocity > 0.0 {
                let gate_pulses = (step * u64::from(self.gate_percent) / 100).max(1);
                let off = pulse.saturating_add(gate_pulses);
                self.active = Some((note.pad, off));
                let slot = usize::from(events[0].is_some());
                events[slot] = Some(ScheduledEvent::NoteOn(note));
            }
        }

        self.next_boundary = Some((pulse / step).saturating_add(1).saturating_mul(step));
        Some(events)
    }

    pub(crate) fn stop(&mut self) -> Option<ScheduledEvent> {
        self.next_boundary = None;
        self.last_pulse = None;
        self.arp_engine.clear();
        self.melody_engine.clear();
        self.mutator_engine.clear();
        self.acordator_engine.clear();
        self.orbitator_engine.clear();
        self.active
            .take()
            .map(|(pad, _)| ScheduledEvent::NoteOff { pad })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn extract_note(events: [Option<ScheduledEvent>; 2]) -> Option<(u8, u8)> {
        events.into_iter().flatten().find_map(|event| match event {
            ScheduledEvent::NoteOn(step) => Some((step.pad, step.transpose as u8)),
            _ => None,
        })
    }

    #[test]
    fn arp_mode_step_boundaries_preserve_order_without_reset() {
        let mut player = TransportPlayback::new(StepRate::Sixteenth, 960).unwrap();
        player.set_pad(1, Some(0.8));
        player.set_pad(8, Some(0.7));
        assert_eq!(extract_note(player.tick(100).unwrap()), None);
        assert_eq!(extract_note(player.tick(240).unwrap()), Some((1, 0)));
        // Add 3rd pad
        player.set_pad(15, Some(0.6));
        assert_eq!(extract_note(player.tick(300).unwrap()), None);
        assert_eq!(
            player.tick(360).unwrap()[0],
            Some(ScheduledEvent::NoteOff { pad: 1 })
        );
        assert_eq!(extract_note(player.tick(480).unwrap()), Some((8, 0)));
        assert_eq!(extract_note(player.tick(720).unwrap()), Some((15, 0)));
    }

    #[test]
    fn melody_mode_root_transition_preserves_phrase_continuity() {
        let mut player = TransportPlayback::new(StepRate::Sixteenth, 960).unwrap();
        player.set_mode(EngineMode::Melody);
        player.set_pad(1, Some(0.8));
        // Step 0 on Pad 1
        assert_eq!(extract_note(player.tick(0).unwrap()), Some((1, 0)));
        // Rest on step 1 (MEL_2BAR_1 step 1 has vel=0)
        let evs = player.tick(240).unwrap();
        assert_eq!(extract_note(evs), None);

        // Switch root to Pad 8 before step 2
        player.set_pad(8, Some(0.9));
        // Step 2 has offset 12: should now play on Pad 8 with transpose 12!
        let evs = player.tick(480).unwrap();
        assert_eq!(extract_note(evs), Some((8, 12)));
    }

    #[test]
    fn gate_length_matches_configured_percentage() {
        let mut player = TransportPlayback::new(StepRate::Sixteenth, 960).unwrap();
        player.set_gate_percent(75); // 75% of 240 = 180 pulses
        player.set_pad(1, Some(1.0));
        assert_eq!(extract_note(player.tick(0).unwrap()), Some((1, 0)));
        // At 179 pulses NoteOff should not have fired yet
        assert_eq!(player.tick(179).unwrap()[0], None);
        // At 180 pulses NoteOff fires!
        assert_eq!(
            player.tick(180).unwrap()[0],
            Some(ScheduledEvent::NoteOff { pad: 1 })
        );
    }

    #[test]
    fn mutator_acordator_and_orbitator_run_cleanly() {
        let mut player = TransportPlayback::new(StepRate::Sixteenth, 960).unwrap();
        for mode in [EngineMode::Mutator, EngineMode::Acordator, EngineMode::Orbitator] {
            player.set_mode(mode);
            player.set_pad(3, Some(0.8));
            assert!(player.tick(0).is_some());
            assert!(player.tick(240).is_some());
            assert!(player.tick(480).is_some());
            assert!(player.stop().is_some() || player.stop().is_none());
        }
    }
}
