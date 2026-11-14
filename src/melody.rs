//! Host-independent melodic phrase sequencer for the MELODY engine.
//!
//! Stores and generates 20 authentic trance and techno melodic patterns across
//! 4 phrase lengths: 2BAR (32 steps), 4BAR (64 steps), 8BAR (128 steps), and 16BAR (256 steps).

use crate::{Step, StepEngine};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum BarLength {
    #[default]
    Bars2,
    Bars4,
    Bars8,
    Bars16,
}

impl BarLength {
    pub const ALL: [Self; 4] = [Self::Bars2, Self::Bars4, Self::Bars8, Self::Bars16];

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Bars2 => "2BAR",
            Self::Bars4 => "4BAR",
            Self::Bars8 => "8BAR",
            Self::Bars16 => "16BAR",
        }
    }

    #[must_use]
    pub const fn step_count(self) -> usize {
        match self {
            Self::Bars2 => 32,
            Self::Bars4 => 64,
            Self::Bars8 => 128,
            Self::Bars16 => 256,
        }
    }

    #[must_use]
    pub const fn cycle(self) -> Self {
        match self {
            Self::Bars2 => Self::Bars4,
            Self::Bars4 => Self::Bars8,
            Self::Bars8 => Self::Bars16,
            Self::Bars16 => Self::Bars2,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum MelodyVariant {
    #[default]
    Mel1,
    Mel2,
    Mel3,
    Mel4,
    Mel5,
}

impl MelodyVariant {
    pub const ALL: [Self; 5] = [Self::Mel1, Self::Mel2, Self::Mel3, Self::Mel4, Self::Mel5];

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Mel1 => "MEL1",
            Self::Mel2 => "MEL2",
            Self::Mel3 => "MEL3",
            Self::Mel4 => "MEL4",
            Self::Mel5 => "MEL5",
        }
    }

    #[must_use]
    pub const fn cycle(self) -> Self {
        match self {
            Self::Mel1 => Self::Mel2,
            Self::Mel2 => Self::Mel3,
            Self::Mel3 => Self::Mel4,
            Self::Mel4 => Self::Mel5,
            Self::Mel5 => Self::Mel1,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MelodyStep {
    pub offset: i8,
    pub velocity: u8,
}

impl MelodyStep {
    pub const fn new(offset: i8, velocity: u8) -> Self {
        Self { offset, velocity }
    }
}

static MEL_2BAR_1: [MelodyStep; 32] = [
    MelodyStep::new(0, 110),     MelodyStep::new(0, 0),     MelodyStep::new(12, 120),     MelodyStep::new(0, 90),     MelodyStep::new(3, 105),     MelodyStep::new(0, 0),     MelodyStep::new(12, 120),     MelodyStep::new(10, 115),
    MelodyStep::new(0, 0),     MelodyStep::new(12, 120),     MelodyStep::new(3, 100),     MelodyStep::new(0, 0),     MelodyStep::new(7, 110),     MelodyStep::new(0, 0),     MelodyStep::new(10, 115),     MelodyStep::new(12, 127),
    MelodyStep::new(0, 110),     MelodyStep::new(0, 0),     MelodyStep::new(12, 120),     MelodyStep::new(0, 90),     MelodyStep::new(3, 105),     MelodyStep::new(0, 0),     MelodyStep::new(12, 120),     MelodyStep::new(10, 115),
    MelodyStep::new(0, 0),     MelodyStep::new(15, 127),     MelodyStep::new(14, 120),     MelodyStep::new(12, 115),     MelodyStep::new(10, 110),     MelodyStep::new(7, 105),     MelodyStep::new(3, 100),     MelodyStep::new(2, 95),
];

static MEL_2BAR_2: [MelodyStep; 32] = [
    MelodyStep::new(0, 120),     MelodyStep::new(0, 90),     MelodyStep::new(0, 0),     MelodyStep::new(1, 115),     MelodyStep::new(0, 100),     MelodyStep::new(0, 0),     MelodyStep::new(0, 90),     MelodyStep::new(1, 115),
    MelodyStep::new(0, 120),     MelodyStep::new(0, 0),     MelodyStep::new(12, 125),     MelodyStep::new(1, 115),     MelodyStep::new(0, 100),     MelodyStep::new(0, 0),     MelodyStep::new(1, 115),     MelodyStep::new(0, 0),
    MelodyStep::new(0, 120),     MelodyStep::new(0, 90),     MelodyStep::new(0, 0),     MelodyStep::new(1, 115),     MelodyStep::new(0, 100),     MelodyStep::new(0, 0),     MelodyStep::new(0, 90),     MelodyStep::new(1, 115),
    MelodyStep::new(0, 0),     MelodyStep::new(3, 120),     MelodyStep::new(1, 110),     MelodyStep::new(0, 0),     MelodyStep::new(12, 127),     MelodyStep::new(1, 115),     MelodyStep::new(0, 0),     MelodyStep::new(0, 0),
];

static MEL_2BAR_3: [MelodyStep; 32] = [
    MelodyStep::new(0, 127),     MelodyStep::new(12, 95),     MelodyStep::new(12, 100),     MelodyStep::new(12, 105),     MelodyStep::new(0, 120),     MelodyStep::new(12, 95),     MelodyStep::new(12, 100),     MelodyStep::new(12, 105),
    MelodyStep::new(0, 120),     MelodyStep::new(12, 95),     MelodyStep::new(12, 100),     MelodyStep::new(12, 105),     MelodyStep::new(0, 120),     MelodyStep::new(12, 95),     MelodyStep::new(10, 110),     MelodyStep::new(12, 125),
    MelodyStep::new(0, 127),     MelodyStep::new(12, 95),     MelodyStep::new(12, 100),     MelodyStep::new(12, 105),     MelodyStep::new(0, 120),     MelodyStep::new(12, 95),     MelodyStep::new(12, 100),     MelodyStep::new(12, 105),
    MelodyStep::new(0, 120),     MelodyStep::new(12, 95),     MelodyStep::new(12, 100),     MelodyStep::new(12, 105),     MelodyStep::new(3, 110),     MelodyStep::new(5, 115),     MelodyStep::new(7, 120),     MelodyStep::new(10, 127),
];

static MEL_2BAR_4: [MelodyStep; 32] = [
    MelodyStep::new(0, 120),     MelodyStep::new(0, 0),     MelodyStep::new(0, 0),     MelodyStep::new(12, 127),     MelodyStep::new(0, 0),     MelodyStep::new(0, 0),     MelodyStep::new(10, 115),     MelodyStep::new(0, 0),
    MelodyStep::new(0, 0),     MelodyStep::new(7, 110),     MelodyStep::new(0, 0),     MelodyStep::new(0, 0),     MelodyStep::new(10, 115),     MelodyStep::new(0, 0),     MelodyStep::new(12, 125),     MelodyStep::new(0, 0),
    MelodyStep::new(0, 120),     MelodyStep::new(0, 0),     MelodyStep::new(0, 0),     MelodyStep::new(15, 127),     MelodyStep::new(0, 0),     MelodyStep::new(0, 0),     MelodyStep::new(14, 120),     MelodyStep::new(0, 0),
    MelodyStep::new(0, 0),     MelodyStep::new(12, 115),     MelodyStep::new(0, 0),     MelodyStep::new(10, 110),     MelodyStep::new(7, 105),     MelodyStep::new(0, 0),     MelodyStep::new(3, 100),     MelodyStep::new(0, 0),
];

static MEL_2BAR_5: [MelodyStep; 32] = [
    MelodyStep::new(0, 110),     MelodyStep::new(7, 95),     MelodyStep::new(12, 120),     MelodyStep::new(7, 95),     MelodyStep::new(0, 105),     MelodyStep::new(7, 95),     MelodyStep::new(12, 120),     MelodyStep::new(7, 95),
    MelodyStep::new(-2, 110),     MelodyStep::new(5, 95),     MelodyStep::new(10, 120),     MelodyStep::new(5, 95),     MelodyStep::new(-2, 105),     MelodyStep::new(5, 95),     MelodyStep::new(10, 120),     MelodyStep::new(5, 95),
    MelodyStep::new(-4, 110),     MelodyStep::new(3, 95),     MelodyStep::new(8, 120),     MelodyStep::new(3, 95),     MelodyStep::new(-4, 105),     MelodyStep::new(3, 95),     MelodyStep::new(8, 120),     MelodyStep::new(3, 95),
    MelodyStep::new(-5, 110),     MelodyStep::new(2, 95),     MelodyStep::new(7, 120),     MelodyStep::new(2, 95),     MelodyStep::new(-5, 105),     MelodyStep::new(2, 95),     MelodyStep::new(7, 120),     MelodyStep::new(2, 95),
];

static MEL_4BAR_1: [MelodyStep; 64] = [
    MelodyStep::new(12, 120),     MelodyStep::new(0, 0),     MelodyStep::new(12, 100),     MelodyStep::new(15, 125),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),     MelodyStep::new(10, 105),     MelodyStep::new(12, 120),
    MelodyStep::new(0, 0),     MelodyStep::new(12, 105),     MelodyStep::new(15, 125),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),     MelodyStep::new(10, 105),     MelodyStep::new(7, 100),     MelodyStep::new(10, 115),
    MelodyStep::new(8, 120),     MelodyStep::new(0, 0),     MelodyStep::new(8, 100),     MelodyStep::new(12, 125),     MelodyStep::new(15, 120),     MelodyStep::new(12, 110),     MelodyStep::new(10, 105),     MelodyStep::new(8, 120),
    MelodyStep::new(0, 0),     MelodyStep::new(8, 105),     MelodyStep::new(12, 125),     MelodyStep::new(15, 120),     MelodyStep::new(12, 110),     MelodyStep::new(10, 105),     MelodyStep::new(8, 100),     MelodyStep::new(7, 95),
    MelodyStep::new(3, 115),     MelodyStep::new(0, 0),     MelodyStep::new(7, 105),     MelodyStep::new(10, 120),     MelodyStep::new(15, 127),     MelodyStep::new(14, 120),     MelodyStep::new(12, 110),     MelodyStep::new(10, 105),
    MelodyStep::new(0, 0),     MelodyStep::new(7, 100),     MelodyStep::new(10, 115),     MelodyStep::new(14, 120),     MelodyStep::new(15, 125),     MelodyStep::new(17, 127),     MelodyStep::new(15, 120),     MelodyStep::new(14, 110),
    MelodyStep::new(10, 120),     MelodyStep::new(0, 0),     MelodyStep::new(10, 105),     MelodyStep::new(14, 120),     MelodyStep::new(17, 127),     MelodyStep::new(15, 120),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),
    MelodyStep::new(10, 105),     MelodyStep::new(12, 115),     MelodyStep::new(14, 120),     MelodyStep::new(15, 125),     MelodyStep::new(17, 127),     MelodyStep::new(19, 127),     MelodyStep::new(17, 120),     MelodyStep::new(15, 110),
];

static MEL_4BAR_2: [MelodyStep; 64] = [
    MelodyStep::new(12, 125),     MelodyStep::new(12, 100),     MelodyStep::new(0, 0),     MelodyStep::new(15, 127),     MelodyStep::new(0, 0),     MelodyStep::new(14, 115),     MelodyStep::new(12, 120),     MelodyStep::new(0, 0),
    MelodyStep::new(12, 110),     MelodyStep::new(12, 100),     MelodyStep::new(0, 0),     MelodyStep::new(15, 125),     MelodyStep::new(0, 0),     MelodyStep::new(17, 127),     MelodyStep::new(19, 127),     MelodyStep::new(0, 0),
    MelodyStep::new(17, 125),     MelodyStep::new(15, 115),     MelodyStep::new(14, 110),     MelodyStep::new(12, 105),     MelodyStep::new(10, 120),     MelodyStep::new(0, 0),     MelodyStep::new(12, 110),     MelodyStep::new(14, 115),
    MelodyStep::new(15, 120),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),     MelodyStep::new(10, 105),     MelodyStep::new(7, 115),     MelodyStep::new(0, 0),     MelodyStep::new(10, 110),     MelodyStep::new(12, 125),
    MelodyStep::new(12, 125),     MelodyStep::new(12, 100),     MelodyStep::new(0, 0),     MelodyStep::new(15, 127),     MelodyStep::new(0, 0),     MelodyStep::new(14, 115),     MelodyStep::new(12, 120),     MelodyStep::new(0, 0),
    MelodyStep::new(12, 110),     MelodyStep::new(12, 100),     MelodyStep::new(0, 0),     MelodyStep::new(19, 127),     MelodyStep::new(0, 0),     MelodyStep::new(17, 125),     MelodyStep::new(15, 120),     MelodyStep::new(0, 0),
    MelodyStep::new(14, 120),     MelodyStep::new(15, 115),     MelodyStep::new(17, 125),     MelodyStep::new(15, 120),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),     MelodyStep::new(10, 105),     MelodyStep::new(8, 100),
    MelodyStep::new(7, 110),     MelodyStep::new(8, 115),     MelodyStep::new(10, 120),     MelodyStep::new(12, 125),     MelodyStep::new(14, 125),     MelodyStep::new(15, 127),     MelodyStep::new(14, 115),     MelodyStep::new(10, 105),
];

static MEL_4BAR_3: [MelodyStep; 64] = [
    MelodyStep::new(0, 115),     MelodyStep::new(12, 127),     MelodyStep::new(0, 0),     MelodyStep::new(12, 110),     MelodyStep::new(0, 0),     MelodyStep::new(0, 0),     MelodyStep::new(12, 125),     MelodyStep::new(0, 0),
    MelodyStep::new(15, 127),     MelodyStep::new(14, 120),     MelodyStep::new(12, 115),     MelodyStep::new(0, 0),     MelodyStep::new(10, 110),     MelodyStep::new(0, 0),     MelodyStep::new(12, 125),     MelodyStep::new(0, 0),
    MelodyStep::new(0, 115),     MelodyStep::new(12, 127),     MelodyStep::new(0, 0),     MelodyStep::new(12, 110),     MelodyStep::new(0, 0),     MelodyStep::new(0, 0),     MelodyStep::new(12, 125),     MelodyStep::new(0, 0),
    MelodyStep::new(17, 127),     MelodyStep::new(15, 120),     MelodyStep::new(14, 115),     MelodyStep::new(0, 0),     MelodyStep::new(12, 110),     MelodyStep::new(0, 0),     MelodyStep::new(10, 105),     MelodyStep::new(0, 0),
    MelodyStep::new(0, 115),     MelodyStep::new(12, 127),     MelodyStep::new(0, 0),     MelodyStep::new(12, 110),     MelodyStep::new(0, 0),     MelodyStep::new(0, 0),     MelodyStep::new(12, 125),     MelodyStep::new(0, 0),
    MelodyStep::new(15, 127),     MelodyStep::new(14, 120),     MelodyStep::new(12, 115),     MelodyStep::new(0, 0),     MelodyStep::new(10, 110),     MelodyStep::new(0, 0),     MelodyStep::new(12, 125),     MelodyStep::new(0, 0),
    MelodyStep::new(19, 127),     MelodyStep::new(17, 120),     MelodyStep::new(15, 115),     MelodyStep::new(14, 110),     MelodyStep::new(17, 125),     MelodyStep::new(15, 120),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),
    MelodyStep::new(15, 120),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),     MelodyStep::new(10, 105),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),     MelodyStep::new(10, 105),     MelodyStep::new(7, 100),
];

static MEL_4BAR_4: [MelodyStep; 64] = [
    MelodyStep::new(12, 115),     MelodyStep::new(15, 105),     MelodyStep::new(19, 125),     MelodyStep::new(15, 105),     MelodyStep::new(12, 110),     MelodyStep::new(15, 105),     MelodyStep::new(19, 125),     MelodyStep::new(15, 105),
    MelodyStep::new(14, 115),     MelodyStep::new(17, 105),     MelodyStep::new(21, 127),     MelodyStep::new(17, 105),     MelodyStep::new(14, 110),     MelodyStep::new(17, 105),     MelodyStep::new(21, 127),     MelodyStep::new(17, 105),
    MelodyStep::new(15, 115),     MelodyStep::new(19, 105),     MelodyStep::new(22, 127),     MelodyStep::new(19, 105),     MelodyStep::new(15, 110),     MelodyStep::new(19, 105),     MelodyStep::new(22, 127),     MelodyStep::new(19, 105),
    MelodyStep::new(14, 115),     MelodyStep::new(17, 105),     MelodyStep::new(21, 125),     MelodyStep::new(17, 105),     MelodyStep::new(12, 110),     MelodyStep::new(15, 105),     MelodyStep::new(19, 120),     MelodyStep::new(15, 105),
    MelodyStep::new(8, 115),     MelodyStep::new(12, 105),     MelodyStep::new(15, 125),     MelodyStep::new(12, 105),     MelodyStep::new(8, 110),     MelodyStep::new(12, 105),     MelodyStep::new(15, 125),     MelodyStep::new(12, 105),
    MelodyStep::new(10, 115),     MelodyStep::new(14, 105),     MelodyStep::new(17, 127),     MelodyStep::new(14, 105),     MelodyStep::new(10, 110),     MelodyStep::new(14, 105),     MelodyStep::new(17, 127),     MelodyStep::new(14, 105),
    MelodyStep::new(12, 120),     MelodyStep::new(15, 110),     MelodyStep::new(19, 127),     MelodyStep::new(15, 110),     MelodyStep::new(10, 115),     MelodyStep::new(14, 105),     MelodyStep::new(17, 125),     MelodyStep::new(14, 105),
    MelodyStep::new(8, 110),     MelodyStep::new(12, 100),     MelodyStep::new(15, 120),     MelodyStep::new(12, 100),     MelodyStep::new(7, 105),     MelodyStep::new(10, 95),     MelodyStep::new(14, 115),     MelodyStep::new(10, 95),
];

static MEL_4BAR_5: [MelodyStep; 64] = [
    MelodyStep::new(0, 120),     MelodyStep::new(0, 0),     MelodyStep::new(12, 127),     MelodyStep::new(0, 0),     MelodyStep::new(0, 0),     MelodyStep::new(10, 115),     MelodyStep::new(0, 0),     MelodyStep::new(12, 125),
    MelodyStep::new(0, 0),     MelodyStep::new(0, 0),     MelodyStep::new(15, 127),     MelodyStep::new(14, 120),     MelodyStep::new(12, 115),     MelodyStep::new(0, 0),     MelodyStep::new(10, 110),     MelodyStep::new(0, 0),
    MelodyStep::new(0, 120),     MelodyStep::new(0, 0),     MelodyStep::new(12, 127),     MelodyStep::new(0, 0),     MelodyStep::new(0, 0),     MelodyStep::new(10, 115),     MelodyStep::new(0, 0),     MelodyStep::new(12, 125),
    MelodyStep::new(0, 0),     MelodyStep::new(0, 0),     MelodyStep::new(17, 127),     MelodyStep::new(15, 120),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),     MelodyStep::new(10, 105),     MelodyStep::new(7, 100),
    MelodyStep::new(0, 120),     MelodyStep::new(0, 0),     MelodyStep::new(12, 127),     MelodyStep::new(0, 0),     MelodyStep::new(0, 0),     MelodyStep::new(10, 115),     MelodyStep::new(0, 0),     MelodyStep::new(12, 125),
    MelodyStep::new(0, 0),     MelodyStep::new(0, 0),     MelodyStep::new(15, 127),     MelodyStep::new(14, 120),     MelodyStep::new(12, 115),     MelodyStep::new(0, 0),     MelodyStep::new(10, 110),     MelodyStep::new(0, 0),
    MelodyStep::new(10, 110),     MelodyStep::new(12, 115),     MelodyStep::new(14, 120),     MelodyStep::new(15, 125),     MelodyStep::new(12, 115),     MelodyStep::new(14, 120),     MelodyStep::new(15, 125),     MelodyStep::new(17, 127),
    MelodyStep::new(14, 120),     MelodyStep::new(15, 125),     MelodyStep::new(17, 127),     MelodyStep::new(19, 127),     MelodyStep::new(17, 125),     MelodyStep::new(15, 120),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),
];

static MEL_8BAR_1: [MelodyStep; 128] = [
    MelodyStep::new(12, 120),     MelodyStep::new(0, 0),     MelodyStep::new(12, 100),     MelodyStep::new(15, 125),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),     MelodyStep::new(10, 105),     MelodyStep::new(12, 120),
    MelodyStep::new(0, 0),     MelodyStep::new(12, 105),     MelodyStep::new(15, 125),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),     MelodyStep::new(10, 105),     MelodyStep::new(7, 100),     MelodyStep::new(10, 115),
    MelodyStep::new(8, 120),     MelodyStep::new(0, 0),     MelodyStep::new(8, 100),     MelodyStep::new(12, 125),     MelodyStep::new(15, 120),     MelodyStep::new(12, 110),     MelodyStep::new(10, 105),     MelodyStep::new(8, 120),
    MelodyStep::new(0, 0),     MelodyStep::new(8, 105),     MelodyStep::new(12, 125),     MelodyStep::new(15, 120),     MelodyStep::new(12, 110),     MelodyStep::new(10, 105),     MelodyStep::new(8, 100),     MelodyStep::new(7, 95),
    MelodyStep::new(3, 115),     MelodyStep::new(0, 0),     MelodyStep::new(7, 105),     MelodyStep::new(10, 120),     MelodyStep::new(15, 127),     MelodyStep::new(14, 120),     MelodyStep::new(12, 110),     MelodyStep::new(10, 105),
    MelodyStep::new(0, 0),     MelodyStep::new(7, 100),     MelodyStep::new(10, 115),     MelodyStep::new(14, 120),     MelodyStep::new(15, 125),     MelodyStep::new(17, 127),     MelodyStep::new(15, 120),     MelodyStep::new(14, 110),
    MelodyStep::new(10, 120),     MelodyStep::new(0, 0),     MelodyStep::new(10, 105),     MelodyStep::new(14, 120),     MelodyStep::new(17, 127),     MelodyStep::new(15, 120),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),
    MelodyStep::new(10, 105),     MelodyStep::new(12, 115),     MelodyStep::new(14, 120),     MelodyStep::new(15, 125),     MelodyStep::new(17, 127),     MelodyStep::new(19, 127),     MelodyStep::new(17, 120),     MelodyStep::new(15, 110),
    MelodyStep::new(15, 127),     MelodyStep::new(0, 0),     MelodyStep::new(15, 110),     MelodyStep::new(19, 127),     MelodyStep::new(17, 120),     MelodyStep::new(15, 115),     MelodyStep::new(14, 110),     MelodyStep::new(15, 120),
    MelodyStep::new(0, 0),     MelodyStep::new(15, 110),     MelodyStep::new(19, 127),     MelodyStep::new(17, 120),     MelodyStep::new(15, 115),     MelodyStep::new(14, 110),     MelodyStep::new(12, 105),     MelodyStep::new(14, 115),
    MelodyStep::new(12, 125),     MelodyStep::new(0, 0),     MelodyStep::new(12, 110),     MelodyStep::new(15, 127),     MelodyStep::new(19, 125),     MelodyStep::new(15, 115),     MelodyStep::new(14, 110),     MelodyStep::new(12, 120),
    MelodyStep::new(0, 0),     MelodyStep::new(12, 110),     MelodyStep::new(15, 127),     MelodyStep::new(19, 125),     MelodyStep::new(15, 115),     MelodyStep::new(14, 110),     MelodyStep::new(12, 105),     MelodyStep::new(10, 100),
    MelodyStep::new(8, 120),     MelodyStep::new(0, 0),     MelodyStep::new(12, 110),     MelodyStep::new(15, 125),     MelodyStep::new(20, 127),     MelodyStep::new(19, 120),     MelodyStep::new(17, 115),     MelodyStep::new(15, 110),
    MelodyStep::new(0, 0),     MelodyStep::new(12, 110),     MelodyStep::new(15, 125),     MelodyStep::new(19, 127),     MelodyStep::new(20, 127),     MelodyStep::new(22, 127),     MelodyStep::new(20, 120),     MelodyStep::new(19, 115),
    MelodyStep::new(17, 127),     MelodyStep::new(0, 0),     MelodyStep::new(17, 115),     MelodyStep::new(19, 120),     MelodyStep::new(22, 127),     MelodyStep::new(20, 120),     MelodyStep::new(19, 115),     MelodyStep::new(17, 110),
    MelodyStep::new(15, 115),     MelodyStep::new(17, 120),     MelodyStep::new(19, 125),     MelodyStep::new(20, 127),     MelodyStep::new(22, 127),     MelodyStep::new(24, 127),     MelodyStep::new(22, 120),     MelodyStep::new(19, 115),
];

static MEL_8BAR_2: [MelodyStep; 128] = [
    MelodyStep::new(12, 125),     MelodyStep::new(12, 100),     MelodyStep::new(0, 0),     MelodyStep::new(15, 127),     MelodyStep::new(0, 0),     MelodyStep::new(14, 115),     MelodyStep::new(12, 120),     MelodyStep::new(0, 0),
    MelodyStep::new(12, 110),     MelodyStep::new(12, 100),     MelodyStep::new(0, 0),     MelodyStep::new(15, 125),     MelodyStep::new(0, 0),     MelodyStep::new(17, 127),     MelodyStep::new(19, 127),     MelodyStep::new(0, 0),
    MelodyStep::new(17, 125),     MelodyStep::new(15, 115),     MelodyStep::new(14, 110),     MelodyStep::new(12, 105),     MelodyStep::new(10, 120),     MelodyStep::new(0, 0),     MelodyStep::new(12, 110),     MelodyStep::new(14, 115),
    MelodyStep::new(15, 120),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),     MelodyStep::new(10, 105),     MelodyStep::new(7, 115),     MelodyStep::new(0, 0),     MelodyStep::new(10, 110),     MelodyStep::new(12, 125),
    MelodyStep::new(12, 125),     MelodyStep::new(12, 100),     MelodyStep::new(0, 0),     MelodyStep::new(15, 127),     MelodyStep::new(0, 0),     MelodyStep::new(14, 115),     MelodyStep::new(12, 120),     MelodyStep::new(0, 0),
    MelodyStep::new(12, 110),     MelodyStep::new(12, 100),     MelodyStep::new(0, 0),     MelodyStep::new(19, 127),     MelodyStep::new(0, 0),     MelodyStep::new(17, 125),     MelodyStep::new(15, 120),     MelodyStep::new(0, 0),
    MelodyStep::new(14, 120),     MelodyStep::new(15, 115),     MelodyStep::new(17, 125),     MelodyStep::new(15, 120),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),     MelodyStep::new(10, 105),     MelodyStep::new(8, 100),
    MelodyStep::new(7, 110),     MelodyStep::new(8, 115),     MelodyStep::new(10, 120),     MelodyStep::new(12, 125),     MelodyStep::new(14, 125),     MelodyStep::new(15, 127),     MelodyStep::new(14, 115),     MelodyStep::new(10, 105),
    MelodyStep::new(12, 127),     MelodyStep::new(15, 115),     MelodyStep::new(17, 120),     MelodyStep::new(19, 127),     MelodyStep::new(0, 0),     MelodyStep::new(17, 120),     MelodyStep::new(15, 115),     MelodyStep::new(0, 0),
    MelodyStep::new(12, 120),     MelodyStep::new(15, 115),     MelodyStep::new(17, 120),     MelodyStep::new(22, 127),     MelodyStep::new(0, 0),     MelodyStep::new(20, 120),     MelodyStep::new(19, 115),     MelodyStep::new(0, 0),
    MelodyStep::new(17, 127),     MelodyStep::new(19, 120),     MelodyStep::new(20, 125),     MelodyStep::new(19, 120),     MelodyStep::new(17, 115),     MelodyStep::new(15, 110),     MelodyStep::new(14, 105),     MelodyStep::new(12, 100),
    MelodyStep::new(10, 110),     MelodyStep::new(12, 115),     MelodyStep::new(14, 120),     MelodyStep::new(15, 125),     MelodyStep::new(17, 127),     MelodyStep::new(19, 125),     MelodyStep::new(17, 120),     MelodyStep::new(15, 115),
    MelodyStep::new(12, 127),     MelodyStep::new(15, 115),     MelodyStep::new(17, 120),     MelodyStep::new(19, 127),     MelodyStep::new(0, 0),     MelodyStep::new(17, 120),     MelodyStep::new(15, 115),     MelodyStep::new(0, 0),
    MelodyStep::new(12, 120),     MelodyStep::new(15, 115),     MelodyStep::new(17, 120),     MelodyStep::new(24, 127),     MelodyStep::new(0, 0),     MelodyStep::new(22, 120),     MelodyStep::new(19, 115),     MelodyStep::new(0, 0),
    MelodyStep::new(20, 127),     MelodyStep::new(19, 120),     MelodyStep::new(17, 115),     MelodyStep::new(15, 110),     MelodyStep::new(19, 125),     MelodyStep::new(17, 120),     MelodyStep::new(15, 115),     MelodyStep::new(14, 110),
    MelodyStep::new(17, 120),     MelodyStep::new(15, 115),     MelodyStep::new(14, 110),     MelodyStep::new(12, 105),     MelodyStep::new(15, 115),     MelodyStep::new(14, 110),     MelodyStep::new(12, 105),     MelodyStep::new(10, 100),
];

static MEL_8BAR_3: [MelodyStep; 128] = [
    MelodyStep::new(0, 0),     MelodyStep::new(12, 110),     MelodyStep::new(15, 120),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),     MelodyStep::new(0, 0),     MelodyStep::new(10, 105),     MelodyStep::new(0, 0),
    MelodyStep::new(12, 120),     MelodyStep::new(0, 0),     MelodyStep::new(15, 125),     MelodyStep::new(0, 0),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),     MelodyStep::new(10, 105),     MelodyStep::new(0, 0),
    MelodyStep::new(0, 0),     MelodyStep::new(7, 105),     MelodyStep::new(10, 115),     MelodyStep::new(12, 120),     MelodyStep::new(10, 110),     MelodyStep::new(0, 0),     MelodyStep::new(7, 100),     MelodyStep::new(0, 0),
    MelodyStep::new(8, 115),     MelodyStep::new(0, 0),     MelodyStep::new(10, 120),     MelodyStep::new(0, 0),     MelodyStep::new(8, 110),     MelodyStep::new(7, 100),     MelodyStep::new(5, 95),     MelodyStep::new(0, 0),
    MelodyStep::new(0, 0),     MelodyStep::new(12, 110),     MelodyStep::new(15, 120),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),     MelodyStep::new(0, 0),     MelodyStep::new(10, 105),     MelodyStep::new(0, 0),
    MelodyStep::new(12, 120),     MelodyStep::new(0, 0),     MelodyStep::new(17, 127),     MelodyStep::new(0, 0),     MelodyStep::new(15, 120),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),     MelodyStep::new(0, 0),
    MelodyStep::new(14, 120),     MelodyStep::new(0, 0),     MelodyStep::new(15, 125),     MelodyStep::new(0, 0),     MelodyStep::new(17, 127),     MelodyStep::new(0, 0),     MelodyStep::new(19, 127),     MelodyStep::new(0, 0),
    MelodyStep::new(17, 120),     MelodyStep::new(15, 115),     MelodyStep::new(14, 110),     MelodyStep::new(12, 105),     MelodyStep::new(10, 100),     MelodyStep::new(0, 0),     MelodyStep::new(12, 110),     MelodyStep::new(0, 0),
    MelodyStep::new(0, 0),     MelodyStep::new(19, 127),     MelodyStep::new(17, 120),     MelodyStep::new(15, 115),     MelodyStep::new(19, 127),     MelodyStep::new(0, 0),     MelodyStep::new(17, 120),     MelodyStep::new(0, 0),
    MelodyStep::new(15, 120),     MelodyStep::new(0, 0),     MelodyStep::new(14, 115),     MelodyStep::new(0, 0),     MelodyStep::new(12, 110),     MelodyStep::new(10, 105),     MelodyStep::new(12, 115),     MelodyStep::new(0, 0),
    MelodyStep::new(0, 0),     MelodyStep::new(15, 120),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),     MelodyStep::new(15, 120),     MelodyStep::new(0, 0),     MelodyStep::new(14, 115),     MelodyStep::new(0, 0),
    MelodyStep::new(12, 110),     MelodyStep::new(0, 0),     MelodyStep::new(10, 105),     MelodyStep::new(0, 0),     MelodyStep::new(8, 100),     MelodyStep::new(7, 95),     MelodyStep::new(8, 100),     MelodyStep::new(0, 0),
    MelodyStep::new(0, 0),     MelodyStep::new(17, 125),     MelodyStep::new(15, 120),     MelodyStep::new(14, 115),     MelodyStep::new(17, 125),     MelodyStep::new(0, 0),     MelodyStep::new(15, 120),     MelodyStep::new(0, 0),
    MelodyStep::new(14, 115),     MelodyStep::new(0, 0),     MelodyStep::new(12, 110),     MelodyStep::new(0, 0),     MelodyStep::new(10, 105),     MelodyStep::new(8, 100),     MelodyStep::new(10, 105),     MelodyStep::new(0, 0),
    MelodyStep::new(12, 125),     MelodyStep::new(14, 120),     MelodyStep::new(15, 125),     MelodyStep::new(17, 127),     MelodyStep::new(19, 127),     MelodyStep::new(20, 127),     MelodyStep::new(22, 127),     MelodyStep::new(24, 127),
    MelodyStep::new(22, 120),     MelodyStep::new(20, 115),     MelodyStep::new(19, 110),     MelodyStep::new(17, 105),     MelodyStep::new(15, 110),     MelodyStep::new(14, 105),     MelodyStep::new(12, 110),     MelodyStep::new(10, 100),
];

static MEL_8BAR_4: [MelodyStep; 128] = [
    MelodyStep::new(12, 115),     MelodyStep::new(15, 105),     MelodyStep::new(19, 125),     MelodyStep::new(15, 105),     MelodyStep::new(12, 110),     MelodyStep::new(15, 105),     MelodyStep::new(19, 125),     MelodyStep::new(15, 105),
    MelodyStep::new(14, 115),     MelodyStep::new(17, 105),     MelodyStep::new(21, 127),     MelodyStep::new(17, 105),     MelodyStep::new(14, 110),     MelodyStep::new(17, 105),     MelodyStep::new(21, 127),     MelodyStep::new(17, 105),
    MelodyStep::new(15, 115),     MelodyStep::new(19, 105),     MelodyStep::new(22, 127),     MelodyStep::new(19, 105),     MelodyStep::new(15, 110),     MelodyStep::new(19, 105),     MelodyStep::new(22, 127),     MelodyStep::new(19, 105),
    MelodyStep::new(14, 115),     MelodyStep::new(17, 105),     MelodyStep::new(21, 125),     MelodyStep::new(17, 105),     MelodyStep::new(12, 110),     MelodyStep::new(15, 105),     MelodyStep::new(19, 120),     MelodyStep::new(15, 105),
    MelodyStep::new(8, 115),     MelodyStep::new(12, 105),     MelodyStep::new(15, 125),     MelodyStep::new(12, 105),     MelodyStep::new(8, 110),     MelodyStep::new(12, 105),     MelodyStep::new(15, 125),     MelodyStep::new(12, 105),
    MelodyStep::new(10, 115),     MelodyStep::new(14, 105),     MelodyStep::new(17, 127),     MelodyStep::new(14, 105),     MelodyStep::new(10, 110),     MelodyStep::new(14, 105),     MelodyStep::new(17, 127),     MelodyStep::new(14, 105),
    MelodyStep::new(12, 120),     MelodyStep::new(15, 110),     MelodyStep::new(19, 127),     MelodyStep::new(15, 110),     MelodyStep::new(10, 115),     MelodyStep::new(14, 105),     MelodyStep::new(17, 125),     MelodyStep::new(14, 105),
    MelodyStep::new(8, 110),     MelodyStep::new(12, 100),     MelodyStep::new(15, 120),     MelodyStep::new(12, 100),     MelodyStep::new(7, 105),     MelodyStep::new(10, 95),     MelodyStep::new(14, 115),     MelodyStep::new(10, 95),
    MelodyStep::new(15, 120),     MelodyStep::new(19, 110),     MelodyStep::new(22, 127),     MelodyStep::new(19, 110),     MelodyStep::new(15, 115),     MelodyStep::new(19, 110),     MelodyStep::new(22, 127),     MelodyStep::new(19, 110),
    MelodyStep::new(17, 120),     MelodyStep::new(21, 110),     MelodyStep::new(24, 127),     MelodyStep::new(21, 110),     MelodyStep::new(17, 115),     MelodyStep::new(21, 110),     MelodyStep::new(24, 127),     MelodyStep::new(21, 110),
    MelodyStep::new(19, 125),     MelodyStep::new(22, 115),     MelodyStep::new(14, 127),     MelodyStep::new(22, 115),     MelodyStep::new(19, 120),     MelodyStep::new(22, 115),     MelodyStep::new(14, 127),     MelodyStep::new(22, 115),
    MelodyStep::new(17, 120),     MelodyStep::new(21, 110),     MelodyStep::new(24, 125),     MelodyStep::new(21, 110),     MelodyStep::new(15, 115),     MelodyStep::new(19, 105),     MelodyStep::new(22, 120),     MelodyStep::new(19, 105),
    MelodyStep::new(12, 115),     MelodyStep::new(15, 105),     MelodyStep::new(19, 125),     MelodyStep::new(15, 105),     MelodyStep::new(12, 110),     MelodyStep::new(15, 105),     MelodyStep::new(19, 125),     MelodyStep::new(15, 105),
    MelodyStep::new(14, 115),     MelodyStep::new(17, 105),     MelodyStep::new(21, 127),     MelodyStep::new(17, 105),     MelodyStep::new(14, 110),     MelodyStep::new(17, 105),     MelodyStep::new(21, 127),     MelodyStep::new(17, 105),
    MelodyStep::new(15, 120),     MelodyStep::new(19, 110),     MelodyStep::new(22, 127),     MelodyStep::new(19, 110),     MelodyStep::new(14, 115),     MelodyStep::new(17, 105),     MelodyStep::new(21, 125),     MelodyStep::new(17, 105),
    MelodyStep::new(12, 110),     MelodyStep::new(15, 100),     MelodyStep::new(19, 120),     MelodyStep::new(15, 100),     MelodyStep::new(10, 105),     MelodyStep::new(14, 95),     MelodyStep::new(17, 115),     MelodyStep::new(14, 95),
];

static MEL_8BAR_5: [MelodyStep; 128] = [
    MelodyStep::new(0, 120),     MelodyStep::new(0, 0),     MelodyStep::new(12, 127),     MelodyStep::new(0, 0),     MelodyStep::new(0, 0),     MelodyStep::new(10, 115),     MelodyStep::new(0, 0),     MelodyStep::new(12, 125),
    MelodyStep::new(0, 0),     MelodyStep::new(0, 0),     MelodyStep::new(15, 127),     MelodyStep::new(14, 120),     MelodyStep::new(12, 115),     MelodyStep::new(0, 0),     MelodyStep::new(10, 110),     MelodyStep::new(0, 0),
    MelodyStep::new(0, 120),     MelodyStep::new(0, 0),     MelodyStep::new(12, 127),     MelodyStep::new(0, 0),     MelodyStep::new(0, 0),     MelodyStep::new(10, 115),     MelodyStep::new(0, 0),     MelodyStep::new(12, 125),
    MelodyStep::new(0, 0),     MelodyStep::new(0, 0),     MelodyStep::new(17, 127),     MelodyStep::new(15, 120),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),     MelodyStep::new(10, 105),     MelodyStep::new(7, 100),
    MelodyStep::new(0, 120),     MelodyStep::new(0, 0),     MelodyStep::new(12, 127),     MelodyStep::new(0, 0),     MelodyStep::new(0, 0),     MelodyStep::new(10, 115),     MelodyStep::new(0, 0),     MelodyStep::new(12, 125),
    MelodyStep::new(0, 0),     MelodyStep::new(0, 0),     MelodyStep::new(15, 127),     MelodyStep::new(14, 120),     MelodyStep::new(12, 115),     MelodyStep::new(0, 0),     MelodyStep::new(10, 110),     MelodyStep::new(0, 0),
    MelodyStep::new(10, 110),     MelodyStep::new(12, 115),     MelodyStep::new(14, 120),     MelodyStep::new(15, 125),     MelodyStep::new(12, 115),     MelodyStep::new(14, 120),     MelodyStep::new(15, 125),     MelodyStep::new(17, 127),
    MelodyStep::new(14, 120),     MelodyStep::new(15, 125),     MelodyStep::new(17, 127),     MelodyStep::new(19, 127),     MelodyStep::new(17, 125),     MelodyStep::new(15, 120),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),
    MelodyStep::new(0, 125),     MelodyStep::new(0, 0),     MelodyStep::new(12, 127),     MelodyStep::new(0, 110),     MelodyStep::new(0, 0),     MelodyStep::new(10, 115),     MelodyStep::new(0, 110),     MelodyStep::new(12, 127),
    MelodyStep::new(0, 110),     MelodyStep::new(0, 0),     MelodyStep::new(15, 127),     MelodyStep::new(14, 120),     MelodyStep::new(12, 115),     MelodyStep::new(0, 110),     MelodyStep::new(10, 115),     MelodyStep::new(0, 0),
    MelodyStep::new(0, 125),     MelodyStep::new(0, 0),     MelodyStep::new(12, 127),     MelodyStep::new(0, 110),     MelodyStep::new(0, 0),     MelodyStep::new(10, 115),     MelodyStep::new(0, 110),     MelodyStep::new(12, 127),
    MelodyStep::new(0, 110),     MelodyStep::new(0, 0),     MelodyStep::new(17, 127),     MelodyStep::new(15, 120),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),     MelodyStep::new(10, 105),     MelodyStep::new(7, 100),
    MelodyStep::new(0, 125),     MelodyStep::new(0, 110),     MelodyStep::new(12, 127),     MelodyStep::new(0, 110),     MelodyStep::new(0, 110),     MelodyStep::new(10, 115),     MelodyStep::new(0, 110),     MelodyStep::new(12, 127),
    MelodyStep::new(0, 110),     MelodyStep::new(0, 110),     MelodyStep::new(15, 127),     MelodyStep::new(14, 120),     MelodyStep::new(12, 115),     MelodyStep::new(0, 110),     MelodyStep::new(10, 115),     MelodyStep::new(0, 110),
    MelodyStep::new(12, 125),     MelodyStep::new(14, 125),     MelodyStep::new(15, 127),     MelodyStep::new(17, 127),     MelodyStep::new(15, 125),     MelodyStep::new(17, 127),     MelodyStep::new(19, 127),     MelodyStep::new(21, 127),
    MelodyStep::new(19, 127),     MelodyStep::new(21, 127),     MelodyStep::new(22, 127),     MelodyStep::new(24, 127),     MelodyStep::new(22, 125),     MelodyStep::new(20, 120),     MelodyStep::new(19, 115),     MelodyStep::new(17, 110),
];

static MEL_16BAR_1: [MelodyStep; 256] = [
    MelodyStep::new(12, 120),     MelodyStep::new(0, 0),     MelodyStep::new(12, 100),     MelodyStep::new(15, 125),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),     MelodyStep::new(10, 105),     MelodyStep::new(12, 120),
    MelodyStep::new(0, 0),     MelodyStep::new(12, 105),     MelodyStep::new(15, 125),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),     MelodyStep::new(10, 105),     MelodyStep::new(7, 100),     MelodyStep::new(10, 115),
    MelodyStep::new(8, 120),     MelodyStep::new(0, 0),     MelodyStep::new(8, 100),     MelodyStep::new(12, 125),     MelodyStep::new(15, 120),     MelodyStep::new(12, 110),     MelodyStep::new(10, 105),     MelodyStep::new(8, 120),
    MelodyStep::new(0, 0),     MelodyStep::new(8, 105),     MelodyStep::new(12, 125),     MelodyStep::new(15, 120),     MelodyStep::new(12, 110),     MelodyStep::new(10, 105),     MelodyStep::new(8, 100),     MelodyStep::new(7, 95),
    MelodyStep::new(3, 115),     MelodyStep::new(0, 0),     MelodyStep::new(7, 105),     MelodyStep::new(10, 120),     MelodyStep::new(15, 127),     MelodyStep::new(14, 120),     MelodyStep::new(12, 110),     MelodyStep::new(10, 105),
    MelodyStep::new(0, 0),     MelodyStep::new(7, 100),     MelodyStep::new(10, 115),     MelodyStep::new(14, 120),     MelodyStep::new(15, 125),     MelodyStep::new(17, 127),     MelodyStep::new(15, 120),     MelodyStep::new(14, 110),
    MelodyStep::new(10, 120),     MelodyStep::new(0, 0),     MelodyStep::new(10, 105),     MelodyStep::new(14, 120),     MelodyStep::new(17, 127),     MelodyStep::new(15, 120),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),
    MelodyStep::new(10, 105),     MelodyStep::new(12, 115),     MelodyStep::new(14, 120),     MelodyStep::new(15, 125),     MelodyStep::new(17, 127),     MelodyStep::new(19, 127),     MelodyStep::new(17, 120),     MelodyStep::new(15, 110),
    MelodyStep::new(15, 127),     MelodyStep::new(0, 0),     MelodyStep::new(15, 110),     MelodyStep::new(19, 127),     MelodyStep::new(17, 120),     MelodyStep::new(15, 115),     MelodyStep::new(14, 110),     MelodyStep::new(15, 120),
    MelodyStep::new(0, 0),     MelodyStep::new(15, 110),     MelodyStep::new(19, 127),     MelodyStep::new(17, 120),     MelodyStep::new(15, 115),     MelodyStep::new(14, 110),     MelodyStep::new(12, 105),     MelodyStep::new(14, 115),
    MelodyStep::new(12, 125),     MelodyStep::new(0, 0),     MelodyStep::new(12, 110),     MelodyStep::new(15, 127),     MelodyStep::new(19, 125),     MelodyStep::new(15, 115),     MelodyStep::new(14, 110),     MelodyStep::new(12, 120),
    MelodyStep::new(0, 0),     MelodyStep::new(12, 110),     MelodyStep::new(15, 127),     MelodyStep::new(19, 125),     MelodyStep::new(15, 115),     MelodyStep::new(14, 110),     MelodyStep::new(12, 105),     MelodyStep::new(10, 100),
    MelodyStep::new(8, 120),     MelodyStep::new(0, 0),     MelodyStep::new(12, 110),     MelodyStep::new(15, 125),     MelodyStep::new(20, 127),     MelodyStep::new(19, 120),     MelodyStep::new(17, 115),     MelodyStep::new(15, 110),
    MelodyStep::new(0, 0),     MelodyStep::new(12, 110),     MelodyStep::new(15, 125),     MelodyStep::new(19, 127),     MelodyStep::new(20, 127),     MelodyStep::new(22, 127),     MelodyStep::new(20, 120),     MelodyStep::new(19, 115),
    MelodyStep::new(17, 127),     MelodyStep::new(0, 0),     MelodyStep::new(17, 115),     MelodyStep::new(19, 120),     MelodyStep::new(22, 127),     MelodyStep::new(20, 120),     MelodyStep::new(19, 115),     MelodyStep::new(17, 110),
    MelodyStep::new(15, 115),     MelodyStep::new(17, 120),     MelodyStep::new(19, 125),     MelodyStep::new(20, 127),     MelodyStep::new(22, 127),     MelodyStep::new(24, 127),     MelodyStep::new(22, 120),     MelodyStep::new(19, 115),
    MelodyStep::new(12, 127),     MelodyStep::new(0, 0),     MelodyStep::new(12, 115),     MelodyStep::new(15, 127),     MelodyStep::new(14, 120),     MelodyStep::new(12, 115),     MelodyStep::new(10, 110),     MelodyStep::new(12, 125),
    MelodyStep::new(0, 0),     MelodyStep::new(12, 115),     MelodyStep::new(15, 127),     MelodyStep::new(14, 120),     MelodyStep::new(12, 115),     MelodyStep::new(10, 110),     MelodyStep::new(7, 105),     MelodyStep::new(10, 120),
    MelodyStep::new(8, 125),     MelodyStep::new(0, 0),     MelodyStep::new(8, 115),     MelodyStep::new(12, 127),     MelodyStep::new(15, 125),     MelodyStep::new(12, 115),     MelodyStep::new(10, 110),     MelodyStep::new(8, 120),
    MelodyStep::new(0, 0),     MelodyStep::new(8, 115),     MelodyStep::new(12, 127),     MelodyStep::new(15, 125),     MelodyStep::new(12, 115),     MelodyStep::new(10, 110),     MelodyStep::new(8, 105),     MelodyStep::new(7, 100),
    MelodyStep::new(3, 120),     MelodyStep::new(0, 0),     MelodyStep::new(7, 115),     MelodyStep::new(10, 125),     MelodyStep::new(15, 127),     MelodyStep::new(14, 120),     MelodyStep::new(12, 115),     MelodyStep::new(10, 110),
    MelodyStep::new(0, 0),     MelodyStep::new(7, 110),     MelodyStep::new(10, 120),     MelodyStep::new(14, 125),     MelodyStep::new(15, 127),     MelodyStep::new(17, 127),     MelodyStep::new(15, 120),     MelodyStep::new(14, 115),
    MelodyStep::new(10, 125),     MelodyStep::new(0, 0),     MelodyStep::new(10, 115),     MelodyStep::new(14, 125),     MelodyStep::new(17, 127),     MelodyStep::new(15, 120),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),
    MelodyStep::new(10, 110),     MelodyStep::new(12, 115),     MelodyStep::new(14, 120),     MelodyStep::new(15, 125),     MelodyStep::new(17, 127),     MelodyStep::new(19, 127),     MelodyStep::new(17, 120),     MelodyStep::new(15, 115),
    MelodyStep::new(19, 127),     MelodyStep::new(0, 0),     MelodyStep::new(19, 120),     MelodyStep::new(22, 127),     MelodyStep::new(20, 125),     MelodyStep::new(19, 120),     MelodyStep::new(17, 115),     MelodyStep::new(19, 127),
    MelodyStep::new(0, 0),     MelodyStep::new(19, 120),     MelodyStep::new(22, 127),     MelodyStep::new(20, 125),     MelodyStep::new(19, 120),     MelodyStep::new(17, 115),     MelodyStep::new(15, 110),     MelodyStep::new(17, 120),
    MelodyStep::new(15, 127),     MelodyStep::new(0, 0),     MelodyStep::new(15, 120),     MelodyStep::new(19, 127),     MelodyStep::new(22, 127),     MelodyStep::new(19, 120),     MelodyStep::new(17, 115),     MelodyStep::new(15, 120),
    MelodyStep::new(0, 0),     MelodyStep::new(15, 120),     MelodyStep::new(19, 127),     MelodyStep::new(22, 127),     MelodyStep::new(19, 120),     MelodyStep::new(17, 115),     MelodyStep::new(15, 110),     MelodyStep::new(14, 105),
    MelodyStep::new(12, 125),     MelodyStep::new(0, 0),     MelodyStep::new(15, 120),     MelodyStep::new(19, 127),     MelodyStep::new(24, 127),     MelodyStep::new(22, 125),     MelodyStep::new(20, 120),     MelodyStep::new(19, 115),
    MelodyStep::new(0, 0),     MelodyStep::new(15, 120),     MelodyStep::new(19, 125),     MelodyStep::new(22, 127),     MelodyStep::new(24, 127),     MelodyStep::new(14, 127),     MelodyStep::new(24, 125),     MelodyStep::new(22, 120),
    MelodyStep::new(20, 127),     MelodyStep::new(0, 0),     MelodyStep::new(20, 120),     MelodyStep::new(22, 125),     MelodyStep::new(14, 127),     MelodyStep::new(24, 125),     MelodyStep::new(22, 120),     MelodyStep::new(20, 115),
    MelodyStep::new(19, 120),     MelodyStep::new(20, 125),     MelodyStep::new(22, 127),     MelodyStep::new(24, 127),     MelodyStep::new(14, 127),     MelodyStep::new(16, 127),     MelodyStep::new(14, 125),     MelodyStep::new(24, 120),
];

static MEL_16BAR_2: [MelodyStep; 256] = [
    MelodyStep::new(12, 125),     MelodyStep::new(12, 100),     MelodyStep::new(0, 0),     MelodyStep::new(15, 127),     MelodyStep::new(0, 0),     MelodyStep::new(14, 115),     MelodyStep::new(12, 120),     MelodyStep::new(0, 0),
    MelodyStep::new(12, 110),     MelodyStep::new(12, 100),     MelodyStep::new(0, 0),     MelodyStep::new(15, 125),     MelodyStep::new(0, 0),     MelodyStep::new(17, 127),     MelodyStep::new(19, 127),     MelodyStep::new(0, 0),
    MelodyStep::new(17, 125),     MelodyStep::new(15, 115),     MelodyStep::new(14, 110),     MelodyStep::new(12, 105),     MelodyStep::new(10, 120),     MelodyStep::new(0, 0),     MelodyStep::new(12, 110),     MelodyStep::new(14, 115),
    MelodyStep::new(15, 120),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),     MelodyStep::new(10, 105),     MelodyStep::new(7, 115),     MelodyStep::new(0, 0),     MelodyStep::new(10, 110),     MelodyStep::new(12, 125),
    MelodyStep::new(12, 125),     MelodyStep::new(12, 100),     MelodyStep::new(0, 0),     MelodyStep::new(15, 127),     MelodyStep::new(0, 0),     MelodyStep::new(14, 115),     MelodyStep::new(12, 120),     MelodyStep::new(0, 0),
    MelodyStep::new(12, 110),     MelodyStep::new(12, 100),     MelodyStep::new(0, 0),     MelodyStep::new(19, 127),     MelodyStep::new(0, 0),     MelodyStep::new(17, 125),     MelodyStep::new(15, 120),     MelodyStep::new(0, 0),
    MelodyStep::new(14, 120),     MelodyStep::new(15, 115),     MelodyStep::new(17, 125),     MelodyStep::new(15, 120),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),     MelodyStep::new(10, 105),     MelodyStep::new(8, 100),
    MelodyStep::new(7, 110),     MelodyStep::new(8, 115),     MelodyStep::new(10, 120),     MelodyStep::new(12, 125),     MelodyStep::new(14, 125),     MelodyStep::new(15, 127),     MelodyStep::new(14, 115),     MelodyStep::new(10, 105),
    MelodyStep::new(12, 127),     MelodyStep::new(15, 115),     MelodyStep::new(17, 120),     MelodyStep::new(19, 127),     MelodyStep::new(0, 0),     MelodyStep::new(17, 120),     MelodyStep::new(15, 115),     MelodyStep::new(0, 0),
    MelodyStep::new(12, 120),     MelodyStep::new(15, 115),     MelodyStep::new(17, 120),     MelodyStep::new(22, 127),     MelodyStep::new(0, 0),     MelodyStep::new(20, 120),     MelodyStep::new(19, 115),     MelodyStep::new(0, 0),
    MelodyStep::new(17, 127),     MelodyStep::new(19, 120),     MelodyStep::new(20, 125),     MelodyStep::new(19, 120),     MelodyStep::new(17, 115),     MelodyStep::new(15, 110),     MelodyStep::new(14, 105),     MelodyStep::new(12, 100),
    MelodyStep::new(10, 110),     MelodyStep::new(12, 115),     MelodyStep::new(14, 120),     MelodyStep::new(15, 125),     MelodyStep::new(17, 127),     MelodyStep::new(19, 125),     MelodyStep::new(17, 120),     MelodyStep::new(15, 115),
    MelodyStep::new(12, 127),     MelodyStep::new(15, 115),     MelodyStep::new(17, 120),     MelodyStep::new(19, 127),     MelodyStep::new(0, 0),     MelodyStep::new(17, 120),     MelodyStep::new(15, 115),     MelodyStep::new(0, 0),
    MelodyStep::new(12, 120),     MelodyStep::new(15, 115),     MelodyStep::new(17, 120),     MelodyStep::new(24, 127),     MelodyStep::new(0, 0),     MelodyStep::new(22, 120),     MelodyStep::new(19, 115),     MelodyStep::new(0, 0),
    MelodyStep::new(20, 127),     MelodyStep::new(19, 120),     MelodyStep::new(17, 115),     MelodyStep::new(15, 110),     MelodyStep::new(19, 125),     MelodyStep::new(17, 120),     MelodyStep::new(15, 115),     MelodyStep::new(14, 110),
    MelodyStep::new(17, 120),     MelodyStep::new(15, 115),     MelodyStep::new(14, 110),     MelodyStep::new(12, 105),     MelodyStep::new(15, 115),     MelodyStep::new(14, 110),     MelodyStep::new(12, 105),     MelodyStep::new(10, 100),
    MelodyStep::new(0, 125),     MelodyStep::new(12, 127),     MelodyStep::new(0, 0),     MelodyStep::new(12, 115),     MelodyStep::new(0, 0),     MelodyStep::new(0, 0),     MelodyStep::new(12, 127),     MelodyStep::new(0, 0),
    MelodyStep::new(15, 127),     MelodyStep::new(14, 120),     MelodyStep::new(12, 115),     MelodyStep::new(0, 0),     MelodyStep::new(10, 110),     MelodyStep::new(0, 0),     MelodyStep::new(12, 127),     MelodyStep::new(0, 0),
    MelodyStep::new(0, 125),     MelodyStep::new(12, 127),     MelodyStep::new(0, 0),     MelodyStep::new(12, 115),     MelodyStep::new(0, 0),     MelodyStep::new(0, 0),     MelodyStep::new(12, 127),     MelodyStep::new(0, 0),
    MelodyStep::new(17, 127),     MelodyStep::new(15, 120),     MelodyStep::new(14, 115),     MelodyStep::new(0, 0),     MelodyStep::new(12, 110),     MelodyStep::new(0, 0),     MelodyStep::new(10, 105),     MelodyStep::new(0, 0),
    MelodyStep::new(0, 125),     MelodyStep::new(12, 127),     MelodyStep::new(0, 0),     MelodyStep::new(12, 115),     MelodyStep::new(0, 0),     MelodyStep::new(0, 0),     MelodyStep::new(12, 127),     MelodyStep::new(0, 0),
    MelodyStep::new(15, 127),     MelodyStep::new(14, 120),     MelodyStep::new(12, 115),     MelodyStep::new(0, 0),     MelodyStep::new(10, 110),     MelodyStep::new(0, 0),     MelodyStep::new(12, 127),     MelodyStep::new(0, 0),
    MelodyStep::new(19, 127),     MelodyStep::new(17, 120),     MelodyStep::new(15, 115),     MelodyStep::new(14, 110),     MelodyStep::new(17, 125),     MelodyStep::new(15, 120),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),
    MelodyStep::new(15, 120),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),     MelodyStep::new(10, 105),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),     MelodyStep::new(10, 105),     MelodyStep::new(7, 100),
    MelodyStep::new(12, 127),     MelodyStep::new(0, 0),     MelodyStep::new(15, 120),     MelodyStep::new(17, 125),     MelodyStep::new(19, 127),     MelodyStep::new(0, 0),     MelodyStep::new(17, 120),     MelodyStep::new(15, 115),
    MelodyStep::new(0, 0),     MelodyStep::new(12, 120),     MelodyStep::new(15, 125),     MelodyStep::new(17, 125),     MelodyStep::new(22, 127),     MelodyStep::new(0, 0),     MelodyStep::new(20, 120),     MelodyStep::new(19, 115),
    MelodyStep::new(17, 127),     MelodyStep::new(19, 125),     MelodyStep::new(20, 127),     MelodyStep::new(19, 125),     MelodyStep::new(17, 120),     MelodyStep::new(15, 115),     MelodyStep::new(14, 110),     MelodyStep::new(12, 105),
    MelodyStep::new(10, 110),     MelodyStep::new(12, 115),     MelodyStep::new(14, 120),     MelodyStep::new(15, 125),     MelodyStep::new(17, 127),     MelodyStep::new(19, 127),     MelodyStep::new(17, 120),     MelodyStep::new(15, 115),
    MelodyStep::new(12, 127),     MelodyStep::new(0, 0),     MelodyStep::new(15, 120),     MelodyStep::new(17, 125),     MelodyStep::new(19, 127),     MelodyStep::new(0, 0),     MelodyStep::new(17, 120),     MelodyStep::new(15, 115),
    MelodyStep::new(0, 0),     MelodyStep::new(12, 120),     MelodyStep::new(15, 125),     MelodyStep::new(17, 125),     MelodyStep::new(24, 127),     MelodyStep::new(0, 0),     MelodyStep::new(22, 125),     MelodyStep::new(19, 120),
    MelodyStep::new(20, 127),     MelodyStep::new(19, 125),     MelodyStep::new(17, 120),     MelodyStep::new(15, 115),     MelodyStep::new(19, 125),     MelodyStep::new(17, 120),     MelodyStep::new(15, 115),     MelodyStep::new(14, 110),
    MelodyStep::new(17, 120),     MelodyStep::new(15, 115),     MelodyStep::new(14, 110),     MelodyStep::new(12, 105),     MelodyStep::new(15, 115),     MelodyStep::new(14, 110),     MelodyStep::new(12, 105),     MelodyStep::new(0, 127),
];

static MEL_16BAR_3: [MelodyStep; 256] = [
    MelodyStep::new(0, 0),     MelodyStep::new(12, 110),     MelodyStep::new(15, 120),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),     MelodyStep::new(0, 0),     MelodyStep::new(10, 105),     MelodyStep::new(0, 0),
    MelodyStep::new(12, 120),     MelodyStep::new(0, 0),     MelodyStep::new(15, 125),     MelodyStep::new(0, 0),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),     MelodyStep::new(10, 105),     MelodyStep::new(0, 0),
    MelodyStep::new(0, 0),     MelodyStep::new(7, 105),     MelodyStep::new(10, 115),     MelodyStep::new(12, 120),     MelodyStep::new(10, 110),     MelodyStep::new(0, 0),     MelodyStep::new(7, 100),     MelodyStep::new(0, 0),
    MelodyStep::new(8, 115),     MelodyStep::new(0, 0),     MelodyStep::new(10, 120),     MelodyStep::new(0, 0),     MelodyStep::new(8, 110),     MelodyStep::new(7, 100),     MelodyStep::new(5, 95),     MelodyStep::new(0, 0),
    MelodyStep::new(0, 0),     MelodyStep::new(12, 110),     MelodyStep::new(15, 120),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),     MelodyStep::new(0, 0),     MelodyStep::new(10, 105),     MelodyStep::new(0, 0),
    MelodyStep::new(12, 120),     MelodyStep::new(0, 0),     MelodyStep::new(17, 127),     MelodyStep::new(0, 0),     MelodyStep::new(15, 120),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),     MelodyStep::new(0, 0),
    MelodyStep::new(14, 120),     MelodyStep::new(0, 0),     MelodyStep::new(15, 125),     MelodyStep::new(0, 0),     MelodyStep::new(17, 127),     MelodyStep::new(0, 0),     MelodyStep::new(19, 127),     MelodyStep::new(0, 0),
    MelodyStep::new(17, 120),     MelodyStep::new(15, 115),     MelodyStep::new(14, 110),     MelodyStep::new(12, 105),     MelodyStep::new(10, 100),     MelodyStep::new(0, 0),     MelodyStep::new(12, 110),     MelodyStep::new(0, 0),
    MelodyStep::new(0, 0),     MelodyStep::new(19, 127),     MelodyStep::new(17, 120),     MelodyStep::new(15, 115),     MelodyStep::new(19, 127),     MelodyStep::new(0, 0),     MelodyStep::new(17, 120),     MelodyStep::new(0, 0),
    MelodyStep::new(15, 120),     MelodyStep::new(0, 0),     MelodyStep::new(14, 115),     MelodyStep::new(0, 0),     MelodyStep::new(12, 110),     MelodyStep::new(10, 105),     MelodyStep::new(12, 115),     MelodyStep::new(0, 0),
    MelodyStep::new(0, 0),     MelodyStep::new(15, 120),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),     MelodyStep::new(15, 120),     MelodyStep::new(0, 0),     MelodyStep::new(14, 115),     MelodyStep::new(0, 0),
    MelodyStep::new(12, 110),     MelodyStep::new(0, 0),     MelodyStep::new(10, 105),     MelodyStep::new(0, 0),     MelodyStep::new(8, 100),     MelodyStep::new(7, 95),     MelodyStep::new(8, 100),     MelodyStep::new(0, 0),
    MelodyStep::new(0, 0),     MelodyStep::new(17, 125),     MelodyStep::new(15, 120),     MelodyStep::new(14, 115),     MelodyStep::new(17, 125),     MelodyStep::new(0, 0),     MelodyStep::new(15, 120),     MelodyStep::new(0, 0),
    MelodyStep::new(14, 115),     MelodyStep::new(0, 0),     MelodyStep::new(12, 110),     MelodyStep::new(0, 0),     MelodyStep::new(10, 105),     MelodyStep::new(8, 100),     MelodyStep::new(10, 105),     MelodyStep::new(0, 0),
    MelodyStep::new(12, 125),     MelodyStep::new(14, 120),     MelodyStep::new(15, 125),     MelodyStep::new(17, 127),     MelodyStep::new(19, 127),     MelodyStep::new(20, 127),     MelodyStep::new(22, 127),     MelodyStep::new(24, 127),
    MelodyStep::new(22, 120),     MelodyStep::new(20, 115),     MelodyStep::new(19, 110),     MelodyStep::new(17, 105),     MelodyStep::new(15, 110),     MelodyStep::new(14, 105),     MelodyStep::new(12, 110),     MelodyStep::new(10, 100),
    MelodyStep::new(0, 0),     MelodyStep::new(12, 110),     MelodyStep::new(15, 120),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),     MelodyStep::new(0, 0),     MelodyStep::new(10, 105),     MelodyStep::new(0, 0),
    MelodyStep::new(12, 120),     MelodyStep::new(0, 0),     MelodyStep::new(15, 125),     MelodyStep::new(0, 0),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),     MelodyStep::new(10, 105),     MelodyStep::new(0, 0),
    MelodyStep::new(0, 0),     MelodyStep::new(7, 105),     MelodyStep::new(10, 115),     MelodyStep::new(12, 120),     MelodyStep::new(10, 110),     MelodyStep::new(0, 0),     MelodyStep::new(7, 100),     MelodyStep::new(0, 0),
    MelodyStep::new(8, 115),     MelodyStep::new(0, 0),     MelodyStep::new(10, 120),     MelodyStep::new(0, 0),     MelodyStep::new(8, 110),     MelodyStep::new(7, 100),     MelodyStep::new(5, 95),     MelodyStep::new(0, 0),
    MelodyStep::new(0, 0),     MelodyStep::new(12, 110),     MelodyStep::new(15, 120),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),     MelodyStep::new(0, 0),     MelodyStep::new(10, 105),     MelodyStep::new(0, 0),
    MelodyStep::new(12, 120),     MelodyStep::new(0, 0),     MelodyStep::new(17, 127),     MelodyStep::new(0, 0),     MelodyStep::new(15, 120),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),     MelodyStep::new(0, 0),
    MelodyStep::new(14, 120),     MelodyStep::new(0, 0),     MelodyStep::new(15, 125),     MelodyStep::new(0, 0),     MelodyStep::new(17, 127),     MelodyStep::new(0, 0),     MelodyStep::new(19, 127),     MelodyStep::new(0, 0),
    MelodyStep::new(17, 120),     MelodyStep::new(15, 115),     MelodyStep::new(14, 110),     MelodyStep::new(12, 105),     MelodyStep::new(10, 100),     MelodyStep::new(0, 0),     MelodyStep::new(12, 110),     MelodyStep::new(0, 0),
    MelodyStep::new(0, 0),     MelodyStep::new(24, 127),     MelodyStep::new(22, 125),     MelodyStep::new(20, 120),     MelodyStep::new(24, 127),     MelodyStep::new(0, 0),     MelodyStep::new(22, 125),     MelodyStep::new(0, 0),
    MelodyStep::new(20, 120),     MelodyStep::new(0, 0),     MelodyStep::new(19, 115),     MelodyStep::new(0, 0),     MelodyStep::new(17, 110),     MelodyStep::new(15, 105),     MelodyStep::new(17, 115),     MelodyStep::new(0, 0),
    MelodyStep::new(0, 0),     MelodyStep::new(20, 125),     MelodyStep::new(19, 120),     MelodyStep::new(17, 115),     MelodyStep::new(20, 125),     MelodyStep::new(0, 0),     MelodyStep::new(19, 120),     MelodyStep::new(0, 0),
    MelodyStep::new(17, 115),     MelodyStep::new(0, 0),     MelodyStep::new(15, 110),     MelodyStep::new(0, 0),     MelodyStep::new(14, 105),     MelodyStep::new(12, 100),     MelodyStep::new(14, 105),     MelodyStep::new(0, 0),
    MelodyStep::new(0, 0),     MelodyStep::new(22, 127),     MelodyStep::new(20, 125),     MelodyStep::new(19, 120),     MelodyStep::new(22, 127),     MelodyStep::new(0, 0),     MelodyStep::new(20, 125),     MelodyStep::new(0, 0),
    MelodyStep::new(19, 120),     MelodyStep::new(0, 0),     MelodyStep::new(17, 115),     MelodyStep::new(0, 0),     MelodyStep::new(15, 110),     MelodyStep::new(14, 105),     MelodyStep::new(15, 110),     MelodyStep::new(0, 0),
    MelodyStep::new(24, 127),     MelodyStep::new(22, 125),     MelodyStep::new(20, 120),     MelodyStep::new(19, 115),     MelodyStep::new(22, 125),     MelodyStep::new(20, 120),     MelodyStep::new(19, 115),     MelodyStep::new(17, 110),
    MelodyStep::new(20, 120),     MelodyStep::new(19, 115),     MelodyStep::new(17, 110),     MelodyStep::new(15, 105),     MelodyStep::new(19, 115),     MelodyStep::new(17, 110),     MelodyStep::new(15, 105),     MelodyStep::new(12, 125),
];

static MEL_16BAR_4: [MelodyStep; 256] = [
    MelodyStep::new(12, 115),     MelodyStep::new(15, 105),     MelodyStep::new(19, 125),     MelodyStep::new(15, 105),     MelodyStep::new(12, 110),     MelodyStep::new(15, 105),     MelodyStep::new(19, 125),     MelodyStep::new(15, 105),
    MelodyStep::new(14, 115),     MelodyStep::new(17, 105),     MelodyStep::new(21, 127),     MelodyStep::new(17, 105),     MelodyStep::new(14, 110),     MelodyStep::new(17, 105),     MelodyStep::new(21, 127),     MelodyStep::new(17, 105),
    MelodyStep::new(15, 115),     MelodyStep::new(19, 105),     MelodyStep::new(22, 127),     MelodyStep::new(19, 105),     MelodyStep::new(15, 110),     MelodyStep::new(19, 105),     MelodyStep::new(22, 127),     MelodyStep::new(19, 105),
    MelodyStep::new(14, 115),     MelodyStep::new(17, 105),     MelodyStep::new(21, 125),     MelodyStep::new(17, 105),     MelodyStep::new(12, 110),     MelodyStep::new(15, 105),     MelodyStep::new(19, 120),     MelodyStep::new(15, 105),
    MelodyStep::new(8, 115),     MelodyStep::new(12, 105),     MelodyStep::new(15, 125),     MelodyStep::new(12, 105),     MelodyStep::new(8, 110),     MelodyStep::new(12, 105),     MelodyStep::new(15, 125),     MelodyStep::new(12, 105),
    MelodyStep::new(10, 115),     MelodyStep::new(14, 105),     MelodyStep::new(17, 127),     MelodyStep::new(14, 105),     MelodyStep::new(10, 110),     MelodyStep::new(14, 105),     MelodyStep::new(17, 127),     MelodyStep::new(14, 105),
    MelodyStep::new(12, 120),     MelodyStep::new(15, 110),     MelodyStep::new(19, 127),     MelodyStep::new(15, 110),     MelodyStep::new(10, 115),     MelodyStep::new(14, 105),     MelodyStep::new(17, 125),     MelodyStep::new(14, 105),
    MelodyStep::new(8, 110),     MelodyStep::new(12, 100),     MelodyStep::new(15, 120),     MelodyStep::new(12, 100),     MelodyStep::new(7, 105),     MelodyStep::new(10, 95),     MelodyStep::new(14, 115),     MelodyStep::new(10, 95),
    MelodyStep::new(15, 120),     MelodyStep::new(19, 110),     MelodyStep::new(22, 127),     MelodyStep::new(19, 110),     MelodyStep::new(15, 115),     MelodyStep::new(19, 110),     MelodyStep::new(22, 127),     MelodyStep::new(19, 110),
    MelodyStep::new(17, 120),     MelodyStep::new(21, 110),     MelodyStep::new(24, 127),     MelodyStep::new(21, 110),     MelodyStep::new(17, 115),     MelodyStep::new(21, 110),     MelodyStep::new(24, 127),     MelodyStep::new(21, 110),
    MelodyStep::new(19, 125),     MelodyStep::new(22, 115),     MelodyStep::new(14, 127),     MelodyStep::new(22, 115),     MelodyStep::new(19, 120),     MelodyStep::new(22, 115),     MelodyStep::new(14, 127),     MelodyStep::new(22, 115),
    MelodyStep::new(17, 120),     MelodyStep::new(21, 110),     MelodyStep::new(24, 125),     MelodyStep::new(21, 110),     MelodyStep::new(15, 115),     MelodyStep::new(19, 105),     MelodyStep::new(22, 120),     MelodyStep::new(19, 105),
    MelodyStep::new(12, 115),     MelodyStep::new(15, 105),     MelodyStep::new(19, 125),     MelodyStep::new(15, 105),     MelodyStep::new(12, 110),     MelodyStep::new(15, 105),     MelodyStep::new(19, 125),     MelodyStep::new(15, 105),
    MelodyStep::new(14, 115),     MelodyStep::new(17, 105),     MelodyStep::new(21, 127),     MelodyStep::new(17, 105),     MelodyStep::new(14, 110),     MelodyStep::new(17, 105),     MelodyStep::new(21, 127),     MelodyStep::new(17, 105),
    MelodyStep::new(15, 120),     MelodyStep::new(19, 110),     MelodyStep::new(22, 127),     MelodyStep::new(19, 110),     MelodyStep::new(14, 115),     MelodyStep::new(17, 105),     MelodyStep::new(21, 125),     MelodyStep::new(17, 105),
    MelodyStep::new(12, 110),     MelodyStep::new(15, 100),     MelodyStep::new(19, 120),     MelodyStep::new(15, 100),     MelodyStep::new(10, 105),     MelodyStep::new(14, 95),     MelodyStep::new(17, 115),     MelodyStep::new(14, 95),
    MelodyStep::new(12, 115),     MelodyStep::new(15, 105),     MelodyStep::new(19, 125),     MelodyStep::new(15, 105),     MelodyStep::new(12, 110),     MelodyStep::new(15, 105),     MelodyStep::new(19, 125),     MelodyStep::new(15, 105),
    MelodyStep::new(14, 115),     MelodyStep::new(17, 105),     MelodyStep::new(21, 127),     MelodyStep::new(17, 105),     MelodyStep::new(14, 110),     MelodyStep::new(17, 105),     MelodyStep::new(21, 127),     MelodyStep::new(17, 105),
    MelodyStep::new(15, 115),     MelodyStep::new(19, 105),     MelodyStep::new(22, 127),     MelodyStep::new(19, 105),     MelodyStep::new(15, 110),     MelodyStep::new(19, 105),     MelodyStep::new(22, 127),     MelodyStep::new(19, 105),
    MelodyStep::new(14, 115),     MelodyStep::new(17, 105),     MelodyStep::new(21, 125),     MelodyStep::new(17, 105),     MelodyStep::new(12, 110),     MelodyStep::new(15, 105),     MelodyStep::new(19, 120),     MelodyStep::new(15, 105),
    MelodyStep::new(8, 115),     MelodyStep::new(12, 105),     MelodyStep::new(15, 125),     MelodyStep::new(12, 105),     MelodyStep::new(8, 110),     MelodyStep::new(12, 105),     MelodyStep::new(15, 125),     MelodyStep::new(12, 105),
    MelodyStep::new(10, 115),     MelodyStep::new(14, 105),     MelodyStep::new(17, 127),     MelodyStep::new(14, 105),     MelodyStep::new(10, 110),     MelodyStep::new(14, 105),     MelodyStep::new(17, 127),     MelodyStep::new(14, 105),
    MelodyStep::new(12, 120),     MelodyStep::new(15, 110),     MelodyStep::new(19, 127),     MelodyStep::new(15, 110),     MelodyStep::new(10, 115),     MelodyStep::new(14, 105),     MelodyStep::new(17, 125),     MelodyStep::new(14, 105),
    MelodyStep::new(8, 110),     MelodyStep::new(12, 100),     MelodyStep::new(15, 120),     MelodyStep::new(12, 100),     MelodyStep::new(7, 105),     MelodyStep::new(10, 95),     MelodyStep::new(14, 115),     MelodyStep::new(10, 95),
    MelodyStep::new(19, 125),     MelodyStep::new(22, 115),     MelodyStep::new(14, 127),     MelodyStep::new(22, 115),     MelodyStep::new(19, 120),     MelodyStep::new(22, 115),     MelodyStep::new(14, 127),     MelodyStep::new(22, 115),
    MelodyStep::new(21, 125),     MelodyStep::new(24, 115),     MelodyStep::new(16, 127),     MelodyStep::new(24, 115),     MelodyStep::new(21, 120),     MelodyStep::new(24, 115),     MelodyStep::new(16, 127),     MelodyStep::new(24, 115),
    MelodyStep::new(22, 127),     MelodyStep::new(14, 120),     MelodyStep::new(17, 127),     MelodyStep::new(14, 120),     MelodyStep::new(22, 125),     MelodyStep::new(14, 120),     MelodyStep::new(17, 127),     MelodyStep::new(14, 120),
    MelodyStep::new(21, 125),     MelodyStep::new(24, 115),     MelodyStep::new(16, 127),     MelodyStep::new(24, 115),     MelodyStep::new(19, 120),     MelodyStep::new(22, 115),     MelodyStep::new(14, 125),     MelodyStep::new(22, 115),
    MelodyStep::new(15, 120),     MelodyStep::new(19, 110),     MelodyStep::new(22, 125),     MelodyStep::new(19, 110),     MelodyStep::new(15, 115),     MelodyStep::new(19, 110),     MelodyStep::new(22, 125),     MelodyStep::new(19, 110),
    MelodyStep::new(17, 120),     MelodyStep::new(21, 110),     MelodyStep::new(24, 127),     MelodyStep::new(21, 110),     MelodyStep::new(17, 115),     MelodyStep::new(21, 110),     MelodyStep::new(24, 127),     MelodyStep::new(21, 110),
    MelodyStep::new(19, 125),     MelodyStep::new(22, 115),     MelodyStep::new(14, 127),     MelodyStep::new(22, 115),     MelodyStep::new(17, 120),     MelodyStep::new(21, 110),     MelodyStep::new(24, 125),     MelodyStep::new(21, 110),
    MelodyStep::new(15, 115),     MelodyStep::new(19, 105),     MelodyStep::new(22, 120),     MelodyStep::new(19, 105),     MelodyStep::new(12, 120),     MelodyStep::new(15, 110),     MelodyStep::new(19, 125),     MelodyStep::new(12, 127),
];

static MEL_16BAR_5: [MelodyStep; 256] = [
    MelodyStep::new(0, 120),     MelodyStep::new(0, 0),     MelodyStep::new(12, 127),     MelodyStep::new(0, 0),     MelodyStep::new(0, 0),     MelodyStep::new(10, 115),     MelodyStep::new(0, 0),     MelodyStep::new(12, 125),
    MelodyStep::new(0, 0),     MelodyStep::new(0, 0),     MelodyStep::new(15, 127),     MelodyStep::new(14, 120),     MelodyStep::new(12, 115),     MelodyStep::new(0, 0),     MelodyStep::new(10, 110),     MelodyStep::new(0, 0),
    MelodyStep::new(0, 120),     MelodyStep::new(0, 0),     MelodyStep::new(12, 127),     MelodyStep::new(0, 0),     MelodyStep::new(0, 0),     MelodyStep::new(10, 115),     MelodyStep::new(0, 0),     MelodyStep::new(12, 125),
    MelodyStep::new(0, 0),     MelodyStep::new(0, 0),     MelodyStep::new(17, 127),     MelodyStep::new(15, 120),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),     MelodyStep::new(10, 105),     MelodyStep::new(7, 100),
    MelodyStep::new(0, 120),     MelodyStep::new(0, 0),     MelodyStep::new(12, 127),     MelodyStep::new(0, 0),     MelodyStep::new(0, 0),     MelodyStep::new(10, 115),     MelodyStep::new(0, 0),     MelodyStep::new(12, 125),
    MelodyStep::new(0, 0),     MelodyStep::new(0, 0),     MelodyStep::new(15, 127),     MelodyStep::new(14, 120),     MelodyStep::new(12, 115),     MelodyStep::new(0, 0),     MelodyStep::new(10, 110),     MelodyStep::new(0, 0),
    MelodyStep::new(10, 110),     MelodyStep::new(12, 115),     MelodyStep::new(14, 120),     MelodyStep::new(15, 125),     MelodyStep::new(12, 115),     MelodyStep::new(14, 120),     MelodyStep::new(15, 125),     MelodyStep::new(17, 127),
    MelodyStep::new(14, 120),     MelodyStep::new(15, 125),     MelodyStep::new(17, 127),     MelodyStep::new(19, 127),     MelodyStep::new(17, 125),     MelodyStep::new(15, 120),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),
    MelodyStep::new(0, 125),     MelodyStep::new(0, 0),     MelodyStep::new(12, 127),     MelodyStep::new(0, 110),     MelodyStep::new(0, 0),     MelodyStep::new(10, 115),     MelodyStep::new(0, 110),     MelodyStep::new(12, 127),
    MelodyStep::new(0, 110),     MelodyStep::new(0, 0),     MelodyStep::new(15, 127),     MelodyStep::new(14, 120),     MelodyStep::new(12, 115),     MelodyStep::new(0, 110),     MelodyStep::new(10, 115),     MelodyStep::new(0, 0),
    MelodyStep::new(0, 125),     MelodyStep::new(0, 0),     MelodyStep::new(12, 127),     MelodyStep::new(0, 110),     MelodyStep::new(0, 0),     MelodyStep::new(10, 115),     MelodyStep::new(0, 110),     MelodyStep::new(12, 127),
    MelodyStep::new(0, 110),     MelodyStep::new(0, 0),     MelodyStep::new(17, 127),     MelodyStep::new(15, 120),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),     MelodyStep::new(10, 105),     MelodyStep::new(7, 100),
    MelodyStep::new(0, 125),     MelodyStep::new(0, 110),     MelodyStep::new(12, 127),     MelodyStep::new(0, 110),     MelodyStep::new(0, 110),     MelodyStep::new(10, 115),     MelodyStep::new(0, 110),     MelodyStep::new(12, 127),
    MelodyStep::new(0, 110),     MelodyStep::new(0, 110),     MelodyStep::new(15, 127),     MelodyStep::new(14, 120),     MelodyStep::new(12, 115),     MelodyStep::new(0, 110),     MelodyStep::new(10, 115),     MelodyStep::new(0, 110),
    MelodyStep::new(12, 125),     MelodyStep::new(14, 125),     MelodyStep::new(15, 127),     MelodyStep::new(17, 127),     MelodyStep::new(15, 125),     MelodyStep::new(17, 127),     MelodyStep::new(19, 127),     MelodyStep::new(21, 127),
    MelodyStep::new(19, 127),     MelodyStep::new(21, 127),     MelodyStep::new(22, 127),     MelodyStep::new(24, 127),     MelodyStep::new(22, 125),     MelodyStep::new(20, 120),     MelodyStep::new(19, 115),     MelodyStep::new(17, 110),
    MelodyStep::new(0, 125),     MelodyStep::new(0, 0),     MelodyStep::new(12, 127),     MelodyStep::new(0, 110),     MelodyStep::new(0, 0),     MelodyStep::new(10, 115),     MelodyStep::new(0, 110),     MelodyStep::new(12, 127),
    MelodyStep::new(0, 110),     MelodyStep::new(0, 0),     MelodyStep::new(15, 127),     MelodyStep::new(14, 120),     MelodyStep::new(12, 115),     MelodyStep::new(0, 110),     MelodyStep::new(10, 115),     MelodyStep::new(0, 0),
    MelodyStep::new(0, 125),     MelodyStep::new(0, 0),     MelodyStep::new(12, 127),     MelodyStep::new(0, 110),     MelodyStep::new(0, 0),     MelodyStep::new(10, 115),     MelodyStep::new(0, 110),     MelodyStep::new(12, 127),
    MelodyStep::new(0, 110),     MelodyStep::new(0, 0),     MelodyStep::new(17, 127),     MelodyStep::new(15, 120),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),     MelodyStep::new(10, 105),     MelodyStep::new(7, 100),
    MelodyStep::new(0, 125),     MelodyStep::new(0, 0),     MelodyStep::new(12, 127),     MelodyStep::new(0, 110),     MelodyStep::new(0, 0),     MelodyStep::new(10, 115),     MelodyStep::new(0, 110),     MelodyStep::new(12, 127),
    MelodyStep::new(0, 110),     MelodyStep::new(0, 0),     MelodyStep::new(15, 127),     MelodyStep::new(14, 120),     MelodyStep::new(12, 115),     MelodyStep::new(0, 110),     MelodyStep::new(10, 115),     MelodyStep::new(0, 0),
    MelodyStep::new(10, 110),     MelodyStep::new(12, 115),     MelodyStep::new(14, 120),     MelodyStep::new(15, 125),     MelodyStep::new(12, 115),     MelodyStep::new(14, 120),     MelodyStep::new(15, 125),     MelodyStep::new(17, 127),
    MelodyStep::new(14, 120),     MelodyStep::new(15, 125),     MelodyStep::new(17, 127),     MelodyStep::new(19, 127),     MelodyStep::new(17, 125),     MelodyStep::new(15, 120),     MelodyStep::new(14, 115),     MelodyStep::new(12, 110),
    MelodyStep::new(0, 127),     MelodyStep::new(0, 115),     MelodyStep::new(12, 127),     MelodyStep::new(0, 115),     MelodyStep::new(3, 120),     MelodyStep::new(0, 115),     MelodyStep::new(12, 127),     MelodyStep::new(0, 115),
    MelodyStep::new(7, 125),     MelodyStep::new(0, 115),     MelodyStep::new(12, 127),     MelodyStep::new(0, 115),     MelodyStep::new(10, 127),     MelodyStep::new(0, 115),     MelodyStep::new(12, 127),     MelodyStep::new(0, 115),
    MelodyStep::new(12, 127),     MelodyStep::new(0, 115),     MelodyStep::new(15, 127),     MelodyStep::new(0, 115),     MelodyStep::new(17, 127),     MelodyStep::new(0, 115),     MelodyStep::new(19, 127),     MelodyStep::new(0, 115),
    MelodyStep::new(17, 125),     MelodyStep::new(0, 115),     MelodyStep::new(15, 120),     MelodyStep::new(0, 115),     MelodyStep::new(14, 115),     MelodyStep::new(0, 115),     MelodyStep::new(12, 110),     MelodyStep::new(0, 115),
    MelodyStep::new(10, 125),     MelodyStep::new(0, 115),     MelodyStep::new(12, 127),     MelodyStep::new(0, 115),     MelodyStep::new(14, 125),     MelodyStep::new(0, 115),     MelodyStep::new(15, 127),     MelodyStep::new(0, 115),
    MelodyStep::new(17, 127),     MelodyStep::new(0, 115),     MelodyStep::new(19, 127),     MelodyStep::new(0, 115),     MelodyStep::new(22, 127),     MelodyStep::new(0, 115),     MelodyStep::new(24, 127),     MelodyStep::new(0, 115),
    MelodyStep::new(24, 127),     MelodyStep::new(22, 125),     MelodyStep::new(20, 120),     MelodyStep::new(19, 115),     MelodyStep::new(17, 110),     MelodyStep::new(15, 105),     MelodyStep::new(14, 100),     MelodyStep::new(12, 95),
    MelodyStep::new(10, 90),     MelodyStep::new(8, 85),     MelodyStep::new(7, 80),     MelodyStep::new(5, 75),     MelodyStep::new(3, 70),     MelodyStep::new(2, 65),     MelodyStep::new(1, 60),     MelodyStep::new(0, 127),
];

pub fn get_step(bars: BarLength, variant: MelodyVariant, step: usize) -> MelodyStep {
    match (bars, variant) {
        (BarLength::Bars2, MelodyVariant::Mel1) => MEL_2BAR_1[step % 32],
        (BarLength::Bars2, MelodyVariant::Mel2) => MEL_2BAR_2[step % 32],
        (BarLength::Bars2, MelodyVariant::Mel3) => MEL_2BAR_3[step % 32],
        (BarLength::Bars2, MelodyVariant::Mel4) => MEL_2BAR_4[step % 32],
        (BarLength::Bars2, MelodyVariant::Mel5) => MEL_2BAR_5[step % 32],

        (BarLength::Bars4, MelodyVariant::Mel1) => MEL_4BAR_1[step % 64],
        (BarLength::Bars4, MelodyVariant::Mel2) => MEL_4BAR_2[step % 64],
        (BarLength::Bars4, MelodyVariant::Mel3) => MEL_4BAR_3[step % 64],
        (BarLength::Bars4, MelodyVariant::Mel4) => MEL_4BAR_4[step % 64],
        (BarLength::Bars4, MelodyVariant::Mel5) => MEL_4BAR_5[step % 64],

        (BarLength::Bars8, MelodyVariant::Mel1) => MEL_8BAR_1[step % 128],
        (BarLength::Bars8, MelodyVariant::Mel2) => MEL_8BAR_2[step % 128],
        (BarLength::Bars8, MelodyVariant::Mel3) => MEL_8BAR_3[step % 128],
        (BarLength::Bars8, MelodyVariant::Mel4) => MEL_8BAR_4[step % 128],
        (BarLength::Bars8, MelodyVariant::Mel5) => MEL_8BAR_5[step % 128],

        (BarLength::Bars16, MelodyVariant::Mel1) => MEL_16BAR_1[step % 256],
        (BarLength::Bars16, MelodyVariant::Mel2) => MEL_16BAR_2[step % 256],
        (BarLength::Bars16, MelodyVariant::Mel3) => MEL_16BAR_3[step % 256],
        (BarLength::Bars16, MelodyVariant::Mel4) => MEL_16BAR_4[step % 256],
        (BarLength::Bars16, MelodyVariant::Mel5) => MEL_16BAR_5[step % 256],
    }
}

/// The melodic phrase sequencer.
#[derive(Debug, Default)]
pub struct MelodyEngine {
    bars: BarLength,
    variant: MelodyVariant,
    current_step: usize,
    root_pad: Option<u8>,
    root_velocity: f32,
}

impl MelodyEngine {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_bars(&mut self, bars: BarLength) {
        if self.bars != bars {
            self.bars = bars;
            self.current_step %= bars.step_count();
        }
    }

    pub fn set_variant(&mut self, variant: MelodyVariant) {
        self.variant = variant;
    }

    pub fn cycle_bars(&mut self) -> BarLength {
        self.set_bars(self.bars.cycle());
        self.bars
    }

    pub fn cycle_variant(&mut self) -> MelodyVariant {
        self.variant = self.variant.cycle();
        self.variant
    }

    pub fn press(&mut self, pad: u8, velocity: f32) {
        self.root_pad = Some(pad);
        self.root_velocity = velocity.clamp(0.01, 1.0);
        self.current_step = 0;
    }

    pub fn release(&mut self, pad: u8) {
        if self.root_pad == Some(pad) {
            self.root_pad = None;
        }
    }

    pub fn clear(&mut self) {
        self.root_pad = None;
        self.current_step = 0;
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
    pub fn current_step(&self) -> usize {
        self.current_step
    }

    #[must_use]
    pub fn root_pad(&self) -> Option<u8> {
        self.root_pad
    }
}

impl StepEngine for MelodyEngine {
    fn held_count(&self) -> usize {
        if self.root_pad.is_some() {
            1
        } else {
            0
        }
    }

    fn next_step(&mut self) -> Option<Step> {
        let pad = self.root_pad?;
        let step_data = get_step(self.bars, self.variant, self.current_step);
        self.current_step = (self.current_step + 1) % self.bars.step_count();

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
                transpose: step_data.offset,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_melody_has_the_exact_expected_length() {
        for &bars in &BarLength::ALL {
            for &variant in &MelodyVariant::ALL {
                for step in 0..bars.step_count() {
                    let s = get_step(bars, variant, step);
                    assert!(s.velocity <= 127);
                    assert!((-24..=24).contains(&s.offset), "Step offset out of bounds: {}", s.offset);
                }
            }
        }
    }

    #[test]
    fn engine_advances_and_loops_around_bars() {
        let mut engine = MelodyEngine::new();
        engine.set_bars(BarLength::Bars2);
        engine.set_variant(MelodyVariant::Mel1);
        engine.press(2, 0.8);
        assert_eq!(engine.held_count(), 1);

        for i in 0..64 {
            let step = engine.next_step().unwrap();
            assert_eq!(step.pad, 2);
            assert_eq!(engine.current_step(), (i + 1) % 32);
        }
    }

    #[test]
    fn releasing_clears_playback() {
        let mut engine = MelodyEngine::new();
        engine.press(5, 0.9);
        assert_eq!(engine.held_count(), 1);
        engine.release(5);
        assert_eq!(engine.held_count(), 0);
        assert!(engine.next_step().is_none());
    }
}
