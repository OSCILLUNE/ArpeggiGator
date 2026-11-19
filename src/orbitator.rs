//! Host-independent generative wandering sequencer for the ORBITATOR engine.
//!
//! Inspired by ORBIX: a single pressed pad sets the gravitational center (the planet).
//! Notes orbit around the root note following authentic musical scales, returning
//! periodically to the center. Completely allocation-free with reproducible seeds.

use crate::mutator::Rng;
use crate::{Step, StepEngine};

/// Musical atmosphere / mood for generative wandering.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum OrbitAtmosphere {
    #[default]
    Ambient,
    Space,
    Dark,
    Rhythm,
}

impl OrbitAtmosphere {
    pub const ALL: [Self; 4] = [Self::Ambient, Self::Space, Self::Dark, Self::Rhythm];

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Ambient => "AMBIENT",
            Self::Space => "SPACE",
            Self::Dark => "DARK",
            Self::Rhythm => "RHYTHM",
        }
    }

    #[must_use]
    pub const fn cycle(self) -> Self {
        match self {
            Self::Ambient => Self::Space,
            Self::Space => Self::Dark,
            Self::Dark => Self::Rhythm,
            Self::Rhythm => Self::Ambient,
        }
    }

    #[must_use]
    pub const fn scale(self) -> &'static [i8] {
        match self {
            // Pentatonic minor / ambient: C, Eb, F, G, Bb, C, Eb
            Self::Ambient => &[0, 3, 5, 7, 10, 12, 15],
            // Cosmic / Lydian sus: C, D, E, F#, G, A, B, C, D
            Self::Space => &[0, 2, 4, 6, 7, 9, 11, 12, 14],
            // Dark / Phrygian synthwave: C, Db, Eb, F, G, Ab, Bb
            Self::Dark => &[-12, -7, 0, 1, 3, 5, 7, 8, 10, 12],
            // Rhythmic techno groove: Root, Octave, 5th, 7th
            Self::Rhythm => &[0, 0, 12, 0, 7, 10, 0, 12],
        }
    }
}

/// Variation dynamics / drift radius.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum OrbitDynamics {
    #[default]
    Calm,
    Flow,
    Wild,
}

impl OrbitDynamics {
    pub const ALL: [Self; 3] = [Self::Calm, Self::Flow, Self::Wild];

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Calm => "CALM",
            Self::Flow => "FLOW",
            Self::Wild => "WILD",
        }
    }

    #[must_use]
    pub const fn cycle(self) -> Self {
        match self {
            Self::Calm => Self::Flow,
            Self::Flow => Self::Wild,
            Self::Wild => Self::Calm,
        }
    }

    #[must_use]
    pub const fn max_offset(self) -> i8 {
        match self {
            Self::Calm => 7,
            Self::Flow => 14,
            Self::Wild => 24,
        }
    }
}

/// Generative Orbitator engine.
#[derive(Debug)]
pub struct OrbitatorEngine {
    atmosphere: OrbitAtmosphere,
    dynamics: OrbitDynamics,
    step_count: usize,
    current_index: usize,
    root_pad: Option<u8>,
    root_velocity: f32,
    rng: Rng,
}

impl Default for OrbitatorEngine {
    fn default() -> Self {
        Self {
            atmosphere: OrbitAtmosphere::Ambient,
            dynamics: OrbitDynamics::Calm,
            step_count: 0,
            current_index: 0,
            root_pad: None,
            root_velocity: 0.8,
            rng: Rng::new(0x08b1_7a00),
        }
    }
}

impl OrbitatorEngine {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_seed(&mut self, seed: u32) {
        self.rng = Rng::new(seed);
    }

    pub fn set_atmosphere(&mut self, atmosphere: OrbitAtmosphere) {
        self.atmosphere = atmosphere;
        if self.current_index >= atmosphere.scale().len() {
            self.current_index = 0;
        }
    }

    pub fn set_dynamics(&mut self, dynamics: OrbitDynamics) {
        self.dynamics = dynamics;
    }

    pub fn cycle_atmosphere(&mut self) -> OrbitAtmosphere {
        self.atmosphere = self.atmosphere.cycle();
        if self.current_index >= self.atmosphere.scale().len() {
            self.current_index = 0;
        }
        self.atmosphere
    }

    pub fn cycle_dynamics(&mut self) -> OrbitDynamics {
        self.dynamics = self.dynamics.cycle();
        self.dynamics
    }

    #[must_use]
    pub fn atmosphere(&self) -> OrbitAtmosphere {
        self.atmosphere
    }

    #[must_use]
    pub fn dynamics(&self) -> OrbitDynamics {
        self.dynamics
    }

    #[must_use]
    pub fn root_pad(&self) -> Option<u8> {
        self.root_pad
    }

    pub fn press(&mut self, pad: u8, velocity: f32) {
        self.root_pad = Some(pad);
        self.root_velocity = velocity.clamp(0.01, 1.0);
        self.step_count = 0;
        self.current_index = 0;
    }

    /// Update root pad preserving the current orbit wandering state.
    pub fn set_root(&mut self, pad: u8, velocity: f32) {
        self.root_pad = Some(pad);
        self.root_velocity = velocity.clamp(0.01, 1.0);
    }

    pub fn release(&mut self, pad: u8) {
        if self.root_pad == Some(pad) {
            self.root_pad = None;
        }
    }

    pub fn clear(&mut self) {
        self.root_pad = None;
        self.step_count = 0;
        self.current_index = 0;
    }
}

impl StepEngine for OrbitatorEngine {
    fn held_count(&self) -> usize {
        if self.root_pad.is_some() { 1 } else { 0 }
    }

    fn next_step(&mut self) -> Option<Step> {
        let pad = self.root_pad?;
        let scale = self.atmosphere.scale();
        let max_off = self.dynamics.max_offset();

        // Periodic gravitational pull back to center root note every 4 or 8 steps
        let transpose = if self.step_count % 8 == 0 {
            0
        } else if self.step_count % 4 == 0 {
            // Gravitational anchor: pick root or 5th / octave
            if self.rng.next_bounded(2) == 0 { 0 } else { 7.min(max_off) }
        } else {
            // Orbital wander: step relative to current index in scale
            let step_choice = self.rng.next_bounded(3);
            let curr = self.current_index % scale.len();
            let next_idx = match step_choice {
                0 => curr.saturating_sub(1),
                1 => curr,
                _ => (curr + 1) % scale.len(),
            };
            self.current_index = next_idx;
            let note = scale[next_idx];
            // Occasional octave drift in Wild dynamics
            let octave_drift = if self.dynamics == OrbitDynamics::Wild && self.rng.next_bounded(4) == 0 {
                if self.rng.next_bounded(2) == 0 { 12 } else { -12 }
            } else {
                0
            };
            (note + octave_drift).clamp(-max_off, max_off)
        };

        self.step_count = self.step_count.wrapping_add(1);

        // Velocity modulation following orbital proximity
        let distance_factor = 1.0 - (transpose.abs() as f32 / 30.0);
        let vel = (self.root_velocity * distance_factor.clamp(0.65, 1.0)).clamp(0.05, 1.0);

        Some(Step {
            pad,
            velocity: vel,
            transpose: transpose.clamp(-24, 24),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orbitator_generates_notes_around_root_and_pulls_to_center() {
        let mut engine = OrbitatorEngine::new();
        engine.set_seed(0x54321);
        engine.set_atmosphere(OrbitAtmosphere::Ambient);
        engine.press(0, 0.8);

        // Step 0 must be 0 (pulled to center)
        let s0 = engine.next_step().unwrap();
        assert_eq!(s0.transpose, 0);

        // Step 8 must also be pulled to center (0)
        for _ in 1..8 {
            engine.next_step();
        }
        let s8 = engine.next_step().unwrap();
        assert_eq!(s8.transpose, 0);
    }

    #[test]
    fn orbitator_respects_dynamics_bounds() {
        for &dynamics in &OrbitDynamics::ALL {
            let mut engine = OrbitatorEngine::new();
            engine.set_dynamics(dynamics);
            engine.press(2, 0.9);

            let max_off = dynamics.max_offset();
            for _ in 0..100 {
                let s = engine.next_step().unwrap();
                assert!(s.transpose.abs() <= max_off, "Transpose {} exceeded max {}", s.transpose, max_off);
                assert!((-24..=24).contains(&s.transpose));
            }
        }
    }

    #[test]
    fn atmosphere_cycling_with_varying_scale_lengths_is_safe() {
        let mut engine = OrbitatorEngine::new();
        engine.press(3, 0.85);

        // Advance into Dark atmosphere (10 notes)
        engine.set_atmosphere(OrbitAtmosphere::Dark);
        for _ in 0..100 {
            engine.next_step();
        }

        // Switch through all atmospheres with different scale lengths
        for atmosphere in OrbitAtmosphere::ALL {
            engine.set_atmosphere(atmosphere);
            for _ in 0..50 {
                assert!(engine.next_step().is_some());
            }
            engine.cycle_atmosphere();
            for _ in 0..50 {
                assert!(engine.next_step().is_some());
            }
        }
    }
}
