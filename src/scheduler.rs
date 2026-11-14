//! Platform-independent, host-testable arpeggiator timing.
//!
//! This module decides when a pattern step and its matching note-off are due.
//! It does not call MPC, retain application pointers, read project tempo or
//! claim transport-phase synchronisation. Firmware integration must feed it a
//! monotonic timestamp and dispatch its events on a verified app thread.

use crate::{ArpEngine, Step};

/// Musical subdivision of one quarter note.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StepRate {
    /// One step per quarter note.
    Quarter,
    /// Quarter-note triplet (three steps per two quarter notes).
    QuarterTriplet,
    /// Two steps per quarter note.
    Eighth,
    /// Three steps per quarter note.
    EighthTriplet,
    /// Four steps per quarter note.
    #[default]
    Sixteenth,
    /// Six steps per quarter note.
    SixteenthTriplet,
    /// Eight steps per quarter note.
    ThirtySecond,
    /// Twelve steps per quarter note.
    ThirtySecondTriplet,
    /// Sixteen steps per quarter note.
    SixtyFourth,
}

impl StepRate {
    /// Available rates from the slowest to the fastest.
    pub const ALL: [Self; 9] = [
        Self::Quarter,
        Self::QuarterTriplet,
        Self::Eighth,
        Self::EighthTriplet,
        Self::Sixteenth,
        Self::SixteenthTriplet,
        Self::ThirtySecond,
        Self::ThirtySecondTriplet,
        Self::SixtyFourth,
    ];

    /// Step duration as a rational fraction of one quarter note.
    pub(crate) const fn quarter_fraction(self) -> (u64, u64) {
        match self {
            Self::Quarter => (1, 1),
            Self::QuarterTriplet => (2, 3),
            Self::Eighth => (1, 2),
            Self::EighthTriplet => (1, 3),
            Self::Sixteenth => (1, 4),
            Self::SixteenthTriplet => (1, 6),
            Self::ThirtySecond => (1, 8),
            Self::ThirtySecondTriplet => (1, 12),
            Self::SixtyFourth => (1, 16),
        }
    }

    /// Move by encoder detents, stopping at either end of the rate list.
    #[must_use]
    pub fn shifted(self, delta: i32) -> Self {
        let index = Self::ALL.iter().position(|&rate| rate == self).unwrap_or(0);
        let next = (index as i64 + i64::from(delta)).clamp(0, (Self::ALL.len() - 1) as i64);
        Self::ALL[next as usize]
    }

    /// Short label for the device UI.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Quarter => "1/4",
            Self::QuarterTriplet => "1/4T",
            Self::Eighth => "1/8",
            Self::EighthTriplet => "1/8T",
            Self::Sixteenth => "1/16",
            Self::SixteenthTriplet => "1/16T",
            Self::ThirtySecond => "1/32",
            Self::ThirtySecondTriplet => "1/32T",
            Self::SixtyFourth => "1/64",
        }
    }
}

/// An event the firmware adapter must dispatch on the app's verified UI thread.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ScheduledEvent {
    /// Trigger one selected pad with the velocity captured by the pad engine.
    NoteOn(Step),
    /// Release the exact pad whose generated note-on was dispatched.
    NoteOff { pad: u8 },
}

#[derive(Clone, Copy, Debug)]
struct ActiveNote {
    pad: u8,
    off_at_us: u64,
}

/// A clock-driven arpeggiator scheduler, deliberately independent of firmware.
///
/// Time is supplied as monotonic microseconds. BPM is stored to 0.001 BPM and
/// grid deadlines are calculated from an integer rational, so rounding does
/// not accumulate from one step to the next. If the caller is late by multiple
/// steps, overdue events are skipped rather than emitted as a burst.
#[derive(Clone, Copy, Debug)]
pub struct ArpScheduler {
    rate: StepRate,
    gate_percent: u8,
    anchor_us: u64,
    bpm_milli: u64,
    next_step_index: u64,
    active_note: Option<ActiveNote>,
    running: bool,
}

impl Default for ArpScheduler {
    fn default() -> Self {
        Self::new(StepRate::default(), 50)
    }
}

impl ArpScheduler {
    /// Create a stopped scheduler. Gate is clamped to 1–100 percent.
    #[must_use]
    pub const fn new(rate: StepRate, gate_percent: u8) -> Self {
        Self {
            rate,
            gate_percent: if gate_percent == 0 {
                1
            } else if gate_percent > 100 {
                100
            } else {
                gate_percent
            },
            anchor_us: 0,
            bpm_milli: 0,
            next_step_index: 0,
            active_note: None,
            running: false,
        }
    }

    /// Current subdivision.
    #[must_use]
    pub const fn rate(&self) -> StepRate {
        self.rate
    }

    /// Current gate length in percent of one step.
    #[must_use]
    pub const fn gate_percent(&self) -> u8 {
        self.gate_percent
    }

    /// Whether the scheduler has been started and not stopped.
    #[must_use]
    pub const fn is_running(&self) -> bool {
        self.running
    }

    /// Start at now_us, using a finite project BPM in the range 20–400.
    ///
    /// Returns false for invalid tempo or when already running. Call stop
    /// first and dispatch its optional NoteOff before restarting.
    pub fn start(&mut self, now_us: u64, bpm: f32) -> bool {
        let Some(bpm_milli) = bpm_to_milli(bpm) else {
            return false;
        };
        if self.running {
            return false;
        }

        self.anchor_us = now_us;
        self.bpm_milli = bpm_milli;
        self.next_step_index = 0;
        self.active_note = None;
        self.running = true;
        true
    }

    /// Stop immediately and return the active note's matching NoteOff, if any.
    pub fn stop(&mut self) -> Option<ScheduledEvent> {
        self.running = false;
        self.active_note
            .take()
            .map(|active| ScheduledEvent::NoteOff { pad: active.pad })
    }

    /// Change the rate while stopped.
    ///
    /// Returns false rather than changing a running clock mid-step.
    pub fn set_rate(&mut self, rate: StepRate) -> bool {
        if self.running {
            return false;
        }
        self.rate = rate;
        true
    }

    /// Set gate length, clamped to 1–100 percent.
    pub fn set_gate_percent(&mut self, gate_percent: u8) {
        self.gate_percent = gate_percent.clamp(1, 100);
    }

    /// Poll the scheduler and return zero, one or two ordered events.
    ///
    /// The first slot is always processed before the second. A tick may return
    /// NoteOff followed by NoteOn when a new step is due. It never returns more
    /// than one NoteOn, even after a long scheduling delay.
    pub fn tick(&mut self, now_us: u64, engine: &mut ArpEngine) -> [Option<ScheduledEvent>; 2] {
        let mut events = [None, None];
        let mut event_count = 0;

        match self.active_note {
            Some(active) if active.off_at_us <= now_us => {
                self.active_note = None;
                events[event_count] = Some(ScheduledEvent::NoteOff { pad: active.pad });
                event_count += 1;
            }
            _ => {}
        }

        if !self.running {
            return events;
        }

        if engine.held_count() == 0 {
            if let Some(active) = self.active_note.take() {
                events[event_count] = Some(ScheduledEvent::NoteOff { pad: active.pad });
            }
            self.running = false;
            return events;
        }

        let due_index = self.next_step_index;
        if now_us < self.deadline_us(due_index) {
            return events;
        }

        // A delayed tick must never overlap two generated voices. If an
        // off-deadline has not yet fired, close the old voice before starting
        // the new step.
        if let Some(active) = self.active_note.take() {
            events[event_count] = Some(ScheduledEvent::NoteOff { pad: active.pad });
            event_count += 1;
        }

        let Some(step) = engine.next_step() else {
            self.running = false;
            return events;
        };
        events[event_count] = Some(ScheduledEvent::NoteOn(step));

        let step_duration = self
            .deadline_us(due_index.saturating_add(1))
            .saturating_sub(self.deadline_us(due_index));
        let gate_duration =
            (u128::from(step_duration) * u128::from(self.gate_percent) / 100).max(1);
        let gate_duration = u64::try_from(gate_duration).unwrap_or(u64::MAX);
        self.active_note = Some(ActiveNote {
            pad: step.pad,
            off_at_us: now_us.saturating_add(gate_duration),
        });

        // Keep the ideal grid phase but skip any deadlines already in the past.
        // Only the step actually emitted advances the pattern engine.
        let elapsed = now_us.saturating_sub(self.anchor_us);
        let latest_due = u64::try_from(
            (u128::from(elapsed) * u128::from(self.denominator())) / self.step_numerator(),
        )
        .unwrap_or(u64::MAX);
        self.next_step_index = latest_due
            .saturating_add(1)
            .max(due_index.saturating_add(1));

        events
    }

    fn denominator(self) -> u64 {
        self.bpm_milli * self.rate.quarter_fraction().1
    }

    fn step_numerator(self) -> u128 {
        STEP_NUMERATOR * u128::from(self.rate.quarter_fraction().0)
    }

    fn deadline_us(self, index: u64) -> u64 {
        let offset = (u128::from(index) * self.step_numerator()) / u128::from(self.denominator());
        let offset = u64::try_from(offset).unwrap_or(u64::MAX);
        self.anchor_us.saturating_add(offset)
    }
}

/// 60 seconds × 1,000,000 microseconds × 1,000 milli-BPM.
const STEP_NUMERATOR: u128 = 60_000_000_000;

fn bpm_to_milli(bpm: f32) -> Option<u64> {
    if !bpm.is_finite() || !(20.0..=400.0).contains(&bpm) {
        return None;
    }
    let milli = (f64::from(bpm) * 1000.0).round();
    (20_000.0..=400_000.0)
        .contains(&milli)
        .then_some(milli as u64)
}

#[cfg(test)]
mod tests {
    use super::{ArpScheduler, ScheduledEvent, StepRate};
    use crate::{ArpEngine, Pattern, Step};

    fn engine() -> ArpEngine {
        let mut engine = ArpEngine::new();
        engine.set_pattern(Pattern::Up);
        engine.press(2, 0.7);
        engine.press(5, 0.9);
        engine
    }

    fn note_on(events: [Option<ScheduledEvent>; 2]) -> Option<Step> {
        events.into_iter().flatten().find_map(|event| match event {
            ScheduledEvent::NoteOn(step) => Some(step),
            ScheduledEvent::NoteOff { .. } => None,
        })
    }

    #[test]
    fn default_is_sixteenth_notes_with_half_gate() {
        let scheduler = ArpScheduler::default();
        assert_eq!(scheduler.rate(), StepRate::Sixteenth);
        assert_eq!(scheduler.gate_percent(), 50);
        assert!(!scheduler.is_running());
    }

    #[test]
    fn first_note_is_due_immediately_then_waits_for_the_grid() {
        let mut scheduler = ArpScheduler::default();
        let mut engine = engine();
        assert!(scheduler.start(1_000_000, 120.0));

        assert_eq!(
            note_on(scheduler.tick(1_000_000, &mut engine)),
            Some(Step {
                pad: 2,
                velocity: 0.7,
                transpose: 0,
            })
        );
        assert_eq!(scheduler.tick(1_062_499, &mut engine), [None, None]);
        assert_eq!(
            scheduler.tick(1_125_000, &mut engine),
            [
                Some(ScheduledEvent::NoteOff { pad: 2 }),
                Some(ScheduledEvent::NoteOn(Step {
                    pad: 5,
                    velocity: 0.9,
                    transpose: 0,
                }))
            ]
        );
    }

    #[test]
    fn all_rates_have_the_expected_120_bpm_step_length() {
        let cases = [
            (StepRate::Quarter, 500_000),
            (StepRate::QuarterTriplet, 333_333),
            (StepRate::Eighth, 250_000),
            (StepRate::EighthTriplet, 166_666),
            (StepRate::Sixteenth, 125_000),
            (StepRate::SixteenthTriplet, 83_333),
            (StepRate::ThirtySecond, 62_500),
            (StepRate::ThirtySecondTriplet, 41_666),
            (StepRate::SixtyFourth, 31_250),
        ];

        for (rate, expected_us) in cases {
            let mut scheduler = ArpScheduler::new(rate, 50);
            let mut engine = engine();
            assert!(scheduler.start(0, 120.0));
            let _ = scheduler.tick(0, &mut engine);
            let next = scheduler.tick(expected_us, &mut engine);
            assert!(
                next.into_iter()
                    .flatten()
                    .any(|event| matches!(event, ScheduledEvent::NoteOn(_))),
                "{} did not fire at {expected_us} us",
                rate.label()
            );
        }
    }

    #[test]
    fn encoder_rate_selection_is_ordered_bounded_and_handles_large_deltas() {
        for pair in StepRate::ALL.windows(2) {
            assert_eq!(pair[0].shifted(1), pair[1]);
            assert_eq!(pair[1].shifted(-1), pair[0]);
        }
        assert_eq!(StepRate::Quarter.shifted(i32::MIN), StepRate::Quarter);
        assert_eq!(
            StepRate::SixtyFourth.shifted(i32::MAX),
            StepRate::SixtyFourth
        );
    }

    #[test]
    fn triplet_and_straight_deadlines_follow_bpm() {
        for (rate, bpm, expected) in [
            (StepRate::Eighth, 60.0, 500_000),
            (StepRate::Eighth, 120.0, 250_000),
            (StepRate::Eighth, 240.0, 125_000),
            (StepRate::QuarterTriplet, 120.0, 333_333),
            (StepRate::SixteenthTriplet, 120.0, 83_333),
        ] {
            let mut scheduler = ArpScheduler::new(rate, 50);
            assert!(scheduler.start(0, bpm));
            assert_eq!(scheduler.deadline_us(1), expected);
        }
    }

    #[test]
    fn gate_off_occurs_at_the_requested_fraction_of_a_step() {
        let mut scheduler = ArpScheduler::new(StepRate::Sixteenth, 50);
        let mut engine = engine();
        assert!(scheduler.start(0, 120.0));
        let _ = scheduler.tick(0, &mut engine);

        assert_eq!(scheduler.tick(62_499, &mut engine), [None, None]);
        assert_eq!(
            scheduler.tick(62_500, &mut engine),
            [Some(ScheduledEvent::NoteOff { pad: 2 }), None]
        );
    }

    #[test]
    fn full_gate_releases_the_old_note_before_starting_the_next() {
        let mut scheduler = ArpScheduler::new(StepRate::Sixteenth, 100);
        let mut engine = engine();
        assert!(scheduler.start(0, 120.0));
        let _ = scheduler.tick(0, &mut engine);

        assert_eq!(
            scheduler.tick(125_000, &mut engine),
            [
                Some(ScheduledEvent::NoteOff { pad: 2 }),
                Some(ScheduledEvent::NoteOn(Step {
                    pad: 5,
                    velocity: 0.9,
                    transpose: 0,
                }))
            ]
        );
    }

    #[test]
    fn releasing_the_last_source_pad_stops_and_releases_the_generated_note() {
        let mut scheduler = ArpScheduler::default();
        let mut engine = engine();
        assert!(scheduler.start(0, 120.0));
        let _ = scheduler.tick(0, &mut engine);
        engine.clear();

        assert_eq!(
            scheduler.tick(1_000, &mut engine),
            [Some(ScheduledEvent::NoteOff { pad: 2 }), None]
        );
        assert!(!scheduler.is_running());
    }

    #[test]
    fn a_late_tick_skips_missed_steps_instead_of_bursting_them() {
        let mut scheduler = ArpScheduler::default();
        let mut engine = engine();
        assert!(scheduler.start(0, 120.0));
        let _ = scheduler.tick(0, &mut engine);

        let late = scheduler.tick(10_000_000, &mut engine);
        assert_eq!(
            late.into_iter()
                .flatten()
                .filter(|event| matches!(event, ScheduledEvent::NoteOn(_)))
                .count(),
            1
        );
        assert_eq!(scheduler.tick(10_000_000, &mut engine), [None, None]);
    }

    #[test]
    fn invalid_tempo_is_rejected_without_starting() {
        for bpm in [f32::NAN, f32::INFINITY, 19.9, 400.1] {
            let mut scheduler = ArpScheduler::default();
            assert!(!scheduler.start(0, bpm));
            assert!(!scheduler.is_running());
        }
    }

    #[test]
    fn stop_returns_a_matching_note_off() {
        let mut scheduler = ArpScheduler::default();
        let mut engine = engine();
        assert!(scheduler.start(0, 120.0));
        let _ = scheduler.tick(0, &mut engine);

        assert_eq!(scheduler.stop(), Some(ScheduledEvent::NoteOff { pad: 2 }));
        assert!(!scheduler.is_running());
        assert_eq!(scheduler.stop(), None);
    }

    #[test]
    fn tempo_can_only_be_started_once_until_stopped() {
        let mut scheduler = ArpScheduler::default();
        assert!(scheduler.start(0, 120.0));
        assert!(!scheduler.start(50_000, 90.0));
        assert!(scheduler.is_running());
    }

    #[test]
    fn gate_is_clamped_to_supported_bounds() {
        let mut scheduler = ArpScheduler::new(StepRate::Sixteenth, 0);
        assert_eq!(scheduler.gate_percent(), 1);
        scheduler.set_gate_percent(200);
        assert_eq!(scheduler.gate_percent(), 100);
    }

    #[test]
    fn rate_cannot_change_while_running() {
        let mut scheduler = ArpScheduler::default();
        assert!(scheduler.start(0, 120.0));
        assert!(!scheduler.set_rate(StepRate::Eighth));
        assert_eq!(scheduler.rate(), StepRate::Sixteenth);
        let _ = scheduler.stop();
        assert!(scheduler.set_rate(StepRate::Eighth));
        assert_eq!(scheduler.rate(), StepRate::Eighth);
    }
}
