//! Host-independent arpeggiator state and pattern engine.
//!
//! This module contains no firmware calls, pointer handling, threads or audio
//! output. It can be tested on the host before connecting it to MPC callbacks.

/// The order in which held pads are emitted.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Pattern {
    /// Lowest pad to highest pad.
    #[default]
    Up,
    /// Highest pad to lowest pad.
    Down,
    /// Up, then down, without repeating either endpoint.
    UpDown,
    /// Down, then up, without repeating either endpoint.
    DownUp,
    /// Outside in: lowest, highest, second-lowest, second-highest.
    Converge,
    /// Centre outwards.
    Diverge,
    /// Pseudo-random order, avoiding an immediate repeat where possible.
    Random,
    /// Order in which the pads were pressed.
    Played,
}

impl Pattern {
    /// All patterns in the order used by SHIFT+B2.
    pub const ALL: [Self; 8] = [
        Self::Up,
        Self::Down,
        Self::UpDown,
        Self::DownUp,
        Self::Converge,
        Self::Diverge,
        Self::Random,
        Self::Played,
    ];

    /// A short label suitable for the MPC display.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Up => "UP",
            Self::Down => "DOWN",
            Self::UpDown => "UP/DOWN",
            Self::DownUp => "DOWN/UP",
            Self::Converge => "CONVERGE",
            Self::Diverge => "DIVERGE",
            Self::Random => "RANDOM",
            Self::Played => "PLAYED",
        }
    }

    /// Return the next pattern in the SHIFT+B2 cycle.
    #[must_use]
    pub const fn next(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::UpDown,
            Self::UpDown => Self::DownUp,
            Self::DownUp => Self::Converge,
            Self::Converge => Self::Diverge,
            Self::Diverge => Self::Random,
            Self::Random => Self::Played,
            Self::Played => Self::Up,
        }
    }
}

/// One selected pad and its original strike velocity.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Step {
    /// Zero-based physical pad index (0–15).
    pub pad: u8,
    /// Original strike velocity, normalised to 0.0–1.0.
    pub velocity: f32,
}

#[derive(Clone, Copy, Debug, Default)]
struct HeldPad {
    active: bool,
    pad: u8,
    velocity: f32,
    press_order: u32,
}

/// The arpeggiator's held-pad set and pure pattern sequencer.
///
/// The engine deliberately does not schedule time or send MIDI. The MPC
/// integration layer will supply a verified clock and trigger mechanism.
#[derive(Debug)]
pub struct ArpEngine {
    held: [HeldPad; 16],
    pattern: Pattern,
    cursor: u32,
    press_serial: u32,
    random_state: u32,
    previous_random: Option<usize>,
}

impl Default for ArpEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl ArpEngine {
    /// Create an empty engine with UP pattern.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            held: [HeldPad {
                active: false,
                pad: 0,
                velocity: 0.0,
                press_order: 0,
            }; 16],
            pattern: Pattern::Up,
            cursor: 0,
            press_serial: 0,
            random_state: 0x9e37_79b9,
            previous_random: None,
        }
    }

    /// Current pattern.
    #[must_use]
    pub const fn pattern(&self) -> Pattern {
        self.pattern
    }

    /// Set the current pattern and restart its cycle.
    pub fn set_pattern(&mut self, pattern: Pattern) {
        self.pattern = pattern;
        self.reset_cycle();
    }

    /// Cycle to the next pattern and return it.
    pub fn cycle_pattern(&mut self) -> Pattern {
        self.set_pattern(self.pattern.next());
        self.pattern
    }

    /// Add a held pad. Repeated press updates velocity without changing
    /// the pad's position in PLAYED order.
    pub fn press(&mut self, pad: u8, velocity: f32) {
        let index = usize::from(pad);
        if index >= self.held.len() {
            return;
        }

        let normalised = if velocity.is_finite() {
            velocity.clamp(0.0, 1.0)
        } else {
            0.0
        };
        if self.held[index].active {
            self.held[index].velocity = normalised;
            return;
        }

        self.press_serial = self.press_serial.wrapping_add(1);
        self.held[index] = HeldPad {
            active: true,
            pad,
            velocity: normalised,
            press_order: self.press_serial,
        };
        self.reset_cycle();
    }

    /// Remove a released pad.
    pub fn release(&mut self, pad: u8) {
        let index = usize::from(pad);
        if index >= self.held.len() || !self.held[index].active {
            return;
        }
        self.held[index] = HeldPad::default();
        self.reset_cycle();
    }

    /// Remove all held pads and reset pattern state.
    pub fn clear(&mut self) {
        self.held = [HeldPad::default(); 16];
        self.reset_cycle();
    }

    /// Number of pads currently held.
    #[must_use]
    pub fn held_count(&self) -> usize {
        self.held.iter().filter(|pad| pad.active).count()
    }

    /// Return the next pad in the current pattern, or None when no pad is held.
    pub fn next_step(&mut self) -> Option<Step> {
        let mut pads = [HeldPad::default(); 16];
        let mut count = 0;
        for held in self.held.iter().copied().filter(|held| held.active) {
            pads[count] = held;
            count += 1;
        }
        if count == 0 {
            self.reset_cycle();
            return None;
        }

        let active = &mut pads[..count];
        if self.pattern == Pattern::Played {
            active.sort_by_key(|pad| pad.press_order);
        } else {
            active.sort_by_key(|pad| pad.pad);
        }

        let step_index = match self.pattern {
            Pattern::Up | Pattern::Played => self.cursor as usize % count,
            Pattern::Down => count - 1 - (self.cursor as usize % count),
            Pattern::UpDown => bounce_up(self.cursor as usize, count),
            Pattern::DownUp => bounce_down(self.cursor as usize, count),
            Pattern::Converge => converge(self.cursor as usize, count),
            Pattern::Diverge => diverge(self.cursor as usize, count),
            Pattern::Random => self.random_index(count),
        };
        self.cursor = self.cursor.wrapping_add(1);
        let selected = active[step_index];
        Some(Step {
            pad: selected.pad,
            velocity: selected.velocity,
        })
    }

    fn reset_cycle(&mut self) {
        self.cursor = 0;
        self.previous_random = None;
    }

    fn random_index(&mut self, count: usize) -> usize {
        let mut value = self.random_state;
        value ^= value << 13;
        value ^= value >> 17;
        value ^= value << 5;
        self.random_state = value;
        let mut index = value as usize % count;
        if count > 1 && self.previous_random == Some(index) {
            index = (index + 1 + ((value >> 8) as usize % (count - 1))) % count;
        }
        self.previous_random = Some(index);
        index
    }
}

fn bounce_up(position: usize, count: usize) -> usize {
    if count <= 1 {
        return 0;
    }
    let period = 2 * count - 2;
    let step = position % period;
    if step < count { step } else { period - step }
}

fn bounce_down(position: usize, count: usize) -> usize {
    if count <= 1 {
        return 0;
    }
    let period = 2 * count - 2;
    let step = position % period;
    if step < count {
        count - 1 - step
    } else {
        step - (count - 1)
    }
}

fn converge(position: usize, count: usize) -> usize {
    let slot = position % count;
    if slot.is_multiple_of(2) {
        slot / 2
    } else {
        count - 1 - slot / 2
    }
}

fn diverge(position: usize, count: usize) -> usize {
    let slot = position % count;
    if !count.is_multiple_of(2) {
        let centre = count / 2;
        if slot == 0 {
            centre
        } else {
            let distance = slot.div_ceil(2);
            if !slot.is_multiple_of(2) {
                centre - distance
            } else {
                centre + distance
            }
        }
    } else {
        let left = count / 2 - 1;
        let distance = slot / 2;
        if slot.is_multiple_of(2) {
            left - distance
        } else {
            left + 1 + distance
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ArpEngine, Pattern};

    fn engine_with_pads(pattern: Pattern, pads: &[u8]) -> ArpEngine {
        let mut engine = ArpEngine::new();
        engine.set_pattern(pattern);
        for (index, pad) in pads.iter().copied().enumerate() {
            engine.press(pad, 0.5 + index as f32 / 20.0);
        }
        engine
    }

    fn sequence(engine: &mut ArpEngine, count: usize) -> Vec<u8> {
        (0..count)
            .filter_map(|_| engine.next_step().map(|step| step.pad))
            .collect()
    }

    #[test]
    fn up_and_down_use_ascending_pad_numbers() {
        let mut up = engine_with_pads(Pattern::Up, &[9, 2, 5]);
        assert_eq!(sequence(&mut up, 3), [2, 5, 9]);
        let mut down = engine_with_pads(Pattern::Down, &[9, 2, 5]);
        assert_eq!(sequence(&mut down, 3), [9, 5, 2]);
    }

    #[test]
    fn bouncing_patterns_do_not_repeat_endpoints() {
        let mut up_down = engine_with_pads(Pattern::UpDown, &[0, 1, 2]);
        assert_eq!(sequence(&mut up_down, 8), [0, 1, 2, 1, 0, 1, 2, 1]);
        let mut down_up = engine_with_pads(Pattern::DownUp, &[0, 1, 2]);
        assert_eq!(sequence(&mut down_up, 8), [2, 1, 0, 1, 2, 1, 0, 1]);
    }

    #[test]
    fn converge_and_diverge_have_distinct_orders() {
        let mut converge = engine_with_pads(Pattern::Converge, &[0, 1, 2, 3, 4]);
        assert_eq!(sequence(&mut converge, 5), [0, 4, 1, 3, 2]);
        let mut diverge = engine_with_pads(Pattern::Diverge, &[0, 1, 2, 3, 4]);
        assert_eq!(sequence(&mut diverge, 5), [2, 1, 3, 0, 4]);
    }

    #[test]
    fn played_pattern_preserves_press_order() {
        let mut engine = engine_with_pads(Pattern::Played, &[7, 1, 12]);
        assert_eq!(sequence(&mut engine, 3), [7, 1, 12]);
    }

    #[test]
    fn release_removes_a_pad_and_restarts_the_cycle() {
        let mut engine = engine_with_pads(Pattern::Up, &[1, 3, 5]);
        assert_eq!(engine.next_step().map(|step| step.pad), Some(1));
        engine.release(1);
        assert_eq!(sequence(&mut engine, 4), [3, 5, 3, 5]);
        assert_eq!(engine.held_count(), 2);
    }

    #[test]
    fn invalid_pad_indices_are_ignored() {
        let mut engine = ArpEngine::new();
        engine.press(16, 1.0);
        engine.release(16);
        assert_eq!(engine.held_count(), 0);
        assert_eq!(engine.next_step(), None);
    }

    #[test]
    fn random_pattern_avoids_immediate_repeats() {
        let mut engine = engine_with_pads(Pattern::Random, &[0, 1, 2, 3]);
        let random = sequence(&mut engine, 64);
        assert!(random.windows(2).all(|pair| pair[0] != pair[1]));
    }

    #[test]
    fn velocities_are_clamped_and_non_finite_values_are_safe() {
        let mut engine = ArpEngine::new();
        engine.press(3, 5.0);
        assert_eq!(engine.next_step().map(|step| step.velocity), Some(1.0));
        engine.clear();
        engine.press(3, f32::NAN);
        assert_eq!(engine.next_step().map(|step| step.velocity), Some(0.0));
    }
}
