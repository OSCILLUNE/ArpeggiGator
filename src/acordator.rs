//! Host-independent chord generator for the ACORDATOR engine.
//!
//! Generates rich musical chords from a single pressed pad, formatted as
//! temporally-spread chord voicings, rhythmic pulses, and sweeping waves.
//! Guaranteed single-voice tracking per step for 100% audio engine stability.

use crate::{Step, StepEngine};

/// Emotional chord mood.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ChordMood {
    #[default]
    Warm,
    Sad,
    Space,
    Cinema,
}

impl ChordMood {
    pub const ALL: [Self; 4] = [Self::Warm, Self::Sad, Self::Space, Self::Cinema];

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Warm => "WARM",
            Self::Sad => "SAD",
            Self::Space => "SPACE",
            Self::Cinema => "CINEMA",
        }
    }

    #[must_use]
    pub const fn cycle(self) -> Self {
        match self {
            Self::Warm => Self::Sad,
            Self::Sad => Self::Space,
            Self::Space => Self::Cinema,
            Self::Cinema => Self::Warm,
        }
    }

    /// Semitone offsets relative to root for the chord voicing.
    #[must_use]
    pub const fn intervals(self) -> &'static [i8] {
        match self {
            // Warm: Major 9th / Add9 (Root, 3rd, 5th, 7th, 9th)
            Self::Warm => &[0, 4, 7, 11, 14, 11, 7, 4],
            // Sad: Minor 9th / Minor 11th (Root, min3rd, 5th, min7th, 9th, 11th)
            Self::Sad => &[0, 3, 7, 10, 14, 17, 14, 10],
            // Space: Lydian Sus2 / Sus4 / Open 5ths ethereal
            Self::Space => &[0, 7, 12, 14, 18, 14, 12, 7],
            // Cinema: Epic cinematic progression intervals
            Self::Cinema => &[0, 3, 7, 12, 15, 14, 10, 7],
        }
    }
}

/// Voicing and playback style across time.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum VoicingStyle {
    #[default]
    Strum,
    Pulse,
    Wave,
}

impl VoicingStyle {
    pub const ALL: [Self; 3] = [Self::Strum, Self::Pulse, Self::Wave];

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Strum => "STRUM",
            Self::Pulse => "PULSE",
            Self::Wave => "WAVE",
        }
    }

    #[must_use]
    pub const fn cycle(self) -> Self {
        match self {
            Self::Strum => Self::Pulse,
            Self::Pulse => Self::Wave,
            Self::Wave => Self::Strum,
        }
    }
}

/// Chord engine generating chord events from a single root pad.
#[derive(Debug, Default)]
pub struct AcordatorEngine {
    mood: ChordMood,
    style: VoicingStyle,
    step_index: usize,
    root_pad: Option<u8>,
    root_velocity: f32,
}

impl AcordatorEngine {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_mood(&mut self, mood: ChordMood) {
        self.mood = mood;
    }

    pub fn set_style(&mut self, style: VoicingStyle) {
        self.style = style;
    }

    pub fn cycle_mood(&mut self) -> ChordMood {
        self.mood = self.mood.cycle();
        self.mood
    }

    pub fn cycle_style(&mut self) -> VoicingStyle {
        self.style = self.style.cycle();
        self.style
    }

    #[must_use]
    pub fn mood(&self) -> ChordMood {
        self.mood
    }

    #[must_use]
    pub fn style(&self) -> VoicingStyle {
        self.style
    }

    #[must_use]
    pub fn root_pad(&self) -> Option<u8> {
        self.root_pad
    }

    pub fn press(&mut self, pad: u8, velocity: f32) {
        self.root_pad = Some(pad);
        self.root_velocity = velocity.clamp(0.01, 1.0);
        self.step_index = 0;
    }

    pub fn release(&mut self, pad: u8) {
        if self.root_pad == Some(pad) {
            self.root_pad = None;
        }
    }

    pub fn clear(&mut self) {
        self.root_pad = None;
        self.step_index = 0;
    }
}

impl StepEngine for AcordatorEngine {
    fn held_count(&self) -> usize {
        if self.root_pad.is_some() { 1 } else { 0 }
    }

    fn next_step(&mut self) -> Option<Step> {
        let pad = self.root_pad?;
        let intervals = self.mood.intervals();
        let len = intervals.len();

        let (offset, vel_factor) = match self.style {
            VoicingStyle::Strum => {
                // Sequential rolling through intervals
                let idx = self.step_index % len;
                let off = intervals[idx];
                let vel = if idx == 0 { 1.0 } else { 0.88 };
                (off, vel)
            }
            VoicingStyle::Pulse => {
                // Alternating bass root and upper chord cluster
                if self.step_index % 2 == 0 {
                    (0, 1.0)
                } else {
                    let cluster_idx = 1 + (self.step_index / 2) % (len.saturating_sub(1).max(1));
                    (intervals[cluster_idx % len], 0.85)
                }
            }
            VoicingStyle::Wave => {
                // Ascending then descending smooth wave with dynamics
                let double_len = len * 2 - 2;
                let phase = self.step_index % double_len.max(1);
                let idx = if phase < len {
                    phase
                } else {
                    double_len - phase
                };
                let dynamic = 0.8 + 0.2 * ((idx as f32) / (len as f32).max(1.0));
                (intervals[idx % len], dynamic)
            }
        };

        self.step_index = self.step_index.wrapping_add(1);

        Some(Step {
            pad,
            velocity: (self.root_velocity * vel_factor).clamp(0.05, 1.0),
            transpose: offset.clamp(-24, 24),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acordator_advances_through_chord_intervals() {
        let mut engine = AcordatorEngine::new();
        engine.set_mood(ChordMood::Warm);
        engine.set_style(VoicingStyle::Strum);
        engine.press(3, 0.9);

        let intervals = ChordMood::Warm.intervals();
        for &expected in intervals {
            let s = engine.next_step().unwrap();
            assert_eq!(s.pad, 3);
            assert_eq!(s.transpose, expected);
        }
    }

    #[test]
    fn acordator_pulse_style_alternates_root_and_tones() {
        let mut engine = AcordatorEngine::new();
        engine.set_mood(ChordMood::Sad);
        engine.set_style(VoicingStyle::Pulse);
        engine.press(1, 1.0);

        // Step 0: root
        let s0 = engine.next_step().unwrap();
        assert_eq!(s0.transpose, 0);

        // Step 1: upper chord tone
        let s1 = engine.next_step().unwrap();
        assert_eq!(s1.transpose, 3); // min3rd

        // Step 2: root
        let s2 = engine.next_step().unwrap();
        assert_eq!(s2.transpose, 0);
    }

    #[test]
    fn all_mood_intervals_are_within_hardware_bounds() {
        for &mood in &ChordMood::ALL {
            for &interval in mood.intervals() {
                assert!((-24..=24).contains(&interval));
            }
        }
    }
}
