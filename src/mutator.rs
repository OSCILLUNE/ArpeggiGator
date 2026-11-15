//! Host-independent evolving melody sequencer for the MUTATOR engine.
//!
//! Takes authentic trance and techno phrases from the MELODY engine as foundation,
//! gradually evolving individual notes, octaves, rests, and velocity over time.
//! Original melody arrays remain immutable.

use crate::melody::{BarLength, MelodyStep, MelodyVariant, get_step};
use crate::{Step, StepEngine};

/// Mutation intensity levels.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum MutationIntensity {
    #[default]
    Subtle,
    Wild,
    Chaos,
}

impl MutationIntensity {
    pub const ALL: [Self; 3] = [Self::Subtle, Self::Wild, Self::Chaos];

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Subtle => "SUBTLE",
            Self::Wild => "WILD",
            Self::Chaos => "CHAOS",
        }
    }

    #[must_use]
    pub const fn cycle(self) -> Self {
        match self {
            Self::Subtle => Self::Wild,
            Self::Wild => Self::Chaos,
            Self::Chaos => Self::Subtle,
        }
    }
}

/// Simple fast allocation-free 32-bit xorshift generator.
#[derive(Clone, Copy, Debug)]
pub struct Rng {
    state: u32,
}

impl Rng {
    #[must_use]
    pub const fn new(seed: u32) -> Self {
        Self {
            state: if seed == 0 { 0x8a5b_c9e3 } else { seed },
        }
    }

    pub fn next_u32(&mut self) -> u32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        x
    }

    pub fn next_bounded(&mut self, bound: u32) -> u32 {
        if bound <= 1 {
            0
        } else {
            self.next_u32() % bound
        }
    }
}

impl Default for Rng {
    fn default() -> Self {
        Self::new(0x2026_1115)
    }
}

/// Consonant minor scale intervals relative to root for musical mutation.
const HARMONIC_INTERVALS: [i8; 14] = [-12, -7, -5, 0, 2, 3, 5, 7, 8, 10, 12, 14, 15, 19];

/// Evolving melody engine.
#[derive(Debug)]
pub struct MutatorEngine {
    bars: BarLength,
    variant: MelodyVariant,
    intensity: MutationIntensity,
    steps: [MelodyStep; 256],
    current_step: usize,
    loop_count: usize,
    root_pad: Option<u8>,
    root_velocity: f32,
    rng: Rng,
    frozen: bool,
}

impl Default for MutatorEngine {
    fn default() -> Self {
        let mut engine = Self {
            bars: BarLength::Bars2,
            variant: MelodyVariant::Mel1,
            intensity: MutationIntensity::Subtle,
            steps: [MelodyStep::new(0, 0); 256],
            current_step: 0,
            loop_count: 0,
            root_pad: None,
            root_velocity: 0.8,
            rng: Rng::default(),
            frozen: false,
        };
        engine.reload_base();
        engine
    }
}

impl MutatorEngine {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set deterministic seed (for unit tests and reproducible patterns).
    pub fn set_seed(&mut self, seed: u32) {
        self.rng = Rng::new(seed);
    }

    pub fn set_bars(&mut self, bars: BarLength) {
        if self.bars != bars {
            self.bars = bars;
            self.current_step %= bars.step_count();
            self.reload_base();
        }
    }

    pub fn set_variant(&mut self, variant: MelodyVariant) {
        if self.variant != variant {
            self.variant = variant;
            self.reload_base();
        }
    }

    pub fn set_intensity(&mut self, intensity: MutationIntensity) {
        self.intensity = intensity;
    }

    pub fn cycle_intensity(&mut self) -> MutationIntensity {
        self.intensity = self.intensity.cycle();
        self.intensity
    }

    pub fn cycle_variant(&mut self) -> MelodyVariant {
        self.set_variant(self.variant.cycle());
        self.variant
    }

    pub fn set_frozen(&mut self, frozen: bool) {
        self.frozen = frozen;
    }

    pub fn toggle_frozen(&mut self) -> bool {
        self.frozen = !self.frozen;
        self.frozen
    }

    #[must_use]
    pub fn is_frozen(&self) -> bool {
        self.frozen
    }

    #[must_use]
    pub fn bars(&self) -> BarLength {
        self.bars
    }

    #[must_use]
    pub fn variant(&self) -> MelodyVariant {
        self.variant
    }

    #[must_use]
    pub fn intensity(&self) -> MutationIntensity {
        self.intensity
    }

    #[must_use]
    pub fn current_step(&self) -> usize {
        self.current_step
    }

    #[must_use]
    pub fn loop_count(&self) -> usize {
        self.loop_count
    }

    #[must_use]
    pub fn root_pad(&self) -> Option<u8> {
        self.root_pad
    }

    /// Reset buffer from base immutable melody.
    pub fn reload_base(&mut self) {
        let count = self.bars.step_count();
        for i in 0..count {
            self.steps[i] = get_step(self.bars, self.variant, i);
        }
        self.loop_count = 0;
    }

    pub fn press(&mut self, pad: u8, velocity: f32) {
        self.root_pad = Some(pad);
        self.root_velocity = velocity.clamp(0.01, 1.0);
        self.current_step = 0;
        self.reload_base();
    }

    /// Update root pad preserving the current mutation buffer and phrase step.
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
        self.current_step = 0;
        self.loop_count = 0;
    }

    /// Mutate several notes in the active buffer based on intensity.
    pub fn mutate_cycle(&mut self) {
        if self.frozen {
            return;
        }
        self.loop_count = self.loop_count.saturating_add(1);
        let count = self.bars.step_count();

        let num_mutations = match self.intensity {
            MutationIntensity::Subtle => {
                // Mutate 1 note every cycle (or every other cycle if count is small)
                1
            }
            MutationIntensity::Wild => {
                // 3 to 5 notes
                (count / 16).clamp(2, 5)
            }
            MutationIntensity::Chaos => {
                // 6 to 10 notes
                (count / 8).clamp(4, 10)
            }
        };

        for _ in 0..num_mutations {
            let step_idx = self.rng.next_bounded(count as u32) as usize;
            let current = &mut self.steps[step_idx];

            match self.intensity {
                MutationIntensity::Subtle => {
                    if current.velocity > 0 {
                        // Subtle pitch tweak to nearest harmonic scale degree or octave leap
                        let mode = self.rng.next_bounded(3);
                        match mode {
                            0 => {
                                // Octave leap if within range
                                let new_offset = if current.offset <= 12 {
                                    current.offset + 12
                                } else {
                                    current.offset - 12
                                };
                                current.offset = new_offset.clamp(-24, 24);
                            }
                            1 => {
                                // Pick neighbouring harmonic interval
                                let idx = self.rng.next_bounded(HARMONIC_INTERVALS.len() as u32)
                                    as usize;
                                current.offset = HARMONIC_INTERVALS[idx];
                            }
                            _ => {
                                // Subtle velocity accent (±10)
                                let delta = if self.rng.next_bounded(2) == 0 { 8 } else { -8 };
                                current.velocity = (i16::from(current.velocity) + delta)
                                    .clamp(60, 127) as u8;
                            }
                        }
                    }
                }
                MutationIntensity::Wild => {
                    let action = self.rng.next_bounded(5);
                    match action {
                        0 => {
                            // Convert rest to a scale tone, or note to a rest
                            if current.velocity == 0 {
                                current.velocity = 100;
                                current.offset = HARMONIC_INTERVALS
                                    [self.rng.next_bounded(HARMONIC_INTERVALS.len() as u32) as usize];
                            } else {
                                current.velocity = 0;
                            }
                        }
                        1 | 2 => {
                            // Pick musical scale interval
                            let idx = self.rng.next_bounded(HARMONIC_INTERVALS.len() as u32) as usize;
                            current.offset = HARMONIC_INTERVALS[idx];
                        }
                        _ => {
                            // Accentuate dynamics
                            let new_vel = if self.rng.next_bounded(2) == 0 { 125 } else { 75 };
                            if current.velocity > 0 {
                                current.velocity = new_vel;
                            }
                        }
                    }
                }
                MutationIntensity::Chaos => {
                    let action = self.rng.next_bounded(4);
                    match action {
                        0 => {
                            // Toggle note/rest
                            if current.velocity == 0 {
                                current.velocity = 115;
                                current.offset = HARMONIC_INTERVALS
                                    [self.rng.next_bounded(HARMONIC_INTERVALS.len() as u32) as usize];
                            } else {
                                current.velocity = 0;
                            }
                        }
                        1 => {
                            // Inverted or octave transposed
                            let octave = if self.rng.next_bounded(2) == 0 { 12 } else { -12 };
                            current.offset = (current.offset + octave).clamp(-24, 24);
                        }
                        _ => {
                            let idx = self.rng.next_bounded(HARMONIC_INTERVALS.len() as u32) as usize;
                            current.offset = HARMONIC_INTERVALS[idx];
                            if current.velocity > 0 {
                                current.velocity = (self.rng.next_bounded(40) + 85) as u8;
                            }
                        }
                    }
                }
            }
        }
    }
}

impl StepEngine for MutatorEngine {
    fn held_count(&self) -> usize {
        if self.root_pad.is_some() { 1 } else { 0 }
    }

    fn next_step(&mut self) -> Option<Step> {
        let pad = self.root_pad?;
        let count = self.bars.step_count();
        let step_data = self.steps[self.current_step % count];

        self.current_step += 1;
        if self.current_step >= count {
            self.current_step = 0;
            self.mutate_cycle();
        }

        if step_data.velocity == 0 {
            Some(Step {
                pad,
                velocity: 0.0,
                transpose: 0,
            })
        } else {
            let vel = self.root_velocity * (f32::from(step_data.velocity) / 127.0);
            Some(Step {
                pad,
                velocity: vel.clamp(0.05, 1.0),
                transpose: step_data.offset.clamp(-24, 24),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mutator_starts_identical_to_base_melody() {
        let mut engine = MutatorEngine::new();
        engine.set_bars(BarLength::Bars2);
        engine.set_variant(MelodyVariant::Mel1);
        engine.press(1, 1.0);

        // First cycle (32 steps) should match the base melody exactly
        for i in 0..32 {
            let expected = get_step(BarLength::Bars2, MelodyVariant::Mel1, i);
            let s = engine.next_step().unwrap();
            assert_eq!(s.pad, 1);
            if expected.velocity == 0 {
                assert_eq!(s.velocity, 0.0);
            } else {
                let exp_vel = (f32::from(expected.velocity) / 127.0).clamp(0.05, 1.0);
                assert!((s.velocity - exp_vel).abs() < 1e-4);
                assert_eq!(s.transpose, expected.offset);
            }
        }
    }

    #[test]
    fn mutator_evolves_after_first_cycle() {
        let mut engine = MutatorEngine::new();
        engine.set_seed(0x1337);
        engine.set_intensity(MutationIntensity::Wild);
        engine.press(3, 0.9);

        // Play 1st cycle
        for _ in 0..32 {
            engine.next_step();
        }
        assert_eq!(engine.loop_count(), 1);

        // Play 2nd cycle and see if it mutated
        let mut differences = 0;
        for i in 0..32 {
            let base = get_step(BarLength::Bars2, MelodyVariant::Mel1, i);
            let s = engine.next_step().unwrap();
            if s.transpose != base.offset {
                differences += 1;
            }
        }
        assert!(differences > 0, "Wild intensity should produce mutations after loop 1");
    }

    #[test]
    fn mutator_freeze_prevents_further_mutation() {
        let mut engine = MutatorEngine::new();
        engine.set_seed(0x42);
        engine.set_intensity(MutationIntensity::Chaos);
        engine.press(0, 1.0);

        // Play 1st cycle
        for _ in 0..32 {
            engine.next_step();
        }
        // Freeze
        engine.set_frozen(true);
        let frozen_steps: Vec<Step> = (0..32).map(|_| engine.next_step().unwrap()).collect();

        // Play another cycle while frozen
        let next_steps: Vec<Step> = (0..32).map(|_| engine.next_step().unwrap()).collect();
        assert_eq!(frozen_steps, next_steps, "Frozen engine must not mutate further");
    }

    #[test]
    fn repressing_pad_resets_mutations_to_base() {
        let mut engine = MutatorEngine::new();
        engine.set_seed(0x999);
        engine.set_intensity(MutationIntensity::Chaos);
        engine.press(0, 1.0);

        // Advance 3 cycles to mutate heavily
        for _ in 0..96 {
            engine.next_step();
        }
        assert_eq!(engine.loop_count(), 3);

        // Repress pad
        engine.press(0, 1.0);
        assert_eq!(engine.loop_count(), 0);

        // Steps should again match clean base
        for i in 0..32 {
            let expected = get_step(BarLength::Bars2, MelodyVariant::Mel1, i);
            let s = engine.next_step().unwrap();
            if expected.velocity == 0 {
                assert_eq!(s.velocity, 0.0);
            } else {
                assert_eq!(s.transpose, expected.offset);
            }
        }
    }

    #[test]
    fn pitch_transposition_is_always_within_hardware_bounds() {
        let mut engine = MutatorEngine::new();
        engine.set_intensity(MutationIntensity::Chaos);
        engine.press(2, 0.8);

        for _ in 0..500 {
            let s = engine.next_step().unwrap();
            assert!((-24..=24).contains(&s.transpose), "Transpose out of bounds: {}", s.transpose);
            assert!((0.0..=1.0).contains(&s.velocity), "Velocity out of bounds: {}", s.velocity);
        }
    }
}
