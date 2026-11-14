//! Experimental, read-only firmware transport snapshot for the emulator.
//!
//! The guarded offsets below were inferred from the 1.3.0.12 binary and
//! observed externally in the lab; they are not a published MPC API.

use mpc_sample_sdk::image;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Snapshot {
    pub playing: bool,
    pub pulse: u64,
    pub loop_start: u64,
    pub loop_end: u64,
    pub looping: bool,
}

/// Read scalar fields only from the engine obtained in the active render call.
/// Every dependent read uses the SDK's checked process-memory reader.
pub(crate) fn snapshot(engine: usize, base: usize) -> Option<Snapshot> {
    if image::read_pointer(engine)? != base.checked_add(0x6e7f2d0)? {
        return None;
    }
    let transport = image::read_pointer(engine.checked_add(0x790)?)?;
    let vtable = image::read_pointer(transport)?;
    if image::read_pointer(vtable.checked_add(0x18)?)? != base.checked_add(0x1445640)?
        || image::read_pointer(vtable.checked_add(0x28)?)? != base.checked_add(0x2145b10)?
        || image::read_pointer(vtable.checked_add(0x30)?)? != base.checked_add(0x2145b20)?
    {
        return None;
    }
    let sequence = image::read_pointer(transport.checked_add(0x258)?)?;
    let extra = image::read_pointer(engine.checked_add(0x418)?).is_some();
    let raw = image::read_u64(transport.checked_add(0x130)?)?;
    let pulse = if extra {
        raw.checked_add(image::read_u64(engine.checked_add(0x668)?)?)?
    } else {
        raw
    };
    let loop_start = image::read_u64(sequence.checked_add(0x58)?)?;
    let loop_end = image::read_u64(sequence.checked_add(0xb0)?)?;
    let playing = image::read_u32(transport.checked_add(0x1f0)?)? as u8 != 0;
    let looping = image::read_u32(sequence.checked_add(0x108)?)? as u8 != 0;
    (loop_end > loop_start && pulse < (1_u64 << 48)).then_some(Snapshot {
        playing,
        pulse,
        loop_start,
        loop_end,
        looping,
    })
}

#[derive(Default)]
pub(crate) struct UnwrapClock {
    previous: Option<(Snapshot, u64, u64)>,
}

impl UnwrapClock {
    /// Returns an unwrapped pulse and whether this observation begins a new
    /// timeline after a seek, a start, or a change to the loop definition.
    pub(crate) fn observe(&mut self, snapshot: Snapshot, now_ms: u64, bpm: f32) -> (u64, bool) {
        let mut reset = true;
        let mut total = snapshot.pulse;
        if let Some((previous, previous_total, previous_ms)) = self.previous {
            let elapsed = now_ms.saturating_sub(previous_ms);
            let max_delta =
                ((f64::from(bpm) * 960.0 * elapsed as f64 / 60_000.0) as u64).saturating_add(120);
            let same_loop = previous.looping == snapshot.looping
                && previous.loop_start == snapshot.loop_start
                && previous.loop_end == snapshot.loop_end;
            let delta = if snapshot.pulse >= previous.pulse {
                Some(snapshot.pulse - previous.pulse)
            } else if snapshot.looping
                && previous.pulse <= snapshot.loop_end
                && snapshot.pulse >= snapshot.loop_start
            {
                Some(snapshot.loop_end - previous.pulse + snapshot.pulse - snapshot.loop_start)
            } else {
                None
            };
            if previous.playing
                && snapshot.playing
                && same_loop
                && let Some(delta) = delta.filter(|delta| *delta <= max_delta)
            {
                total = previous_total.saturating_add(delta);
                reset = false;
            }
        }
        self.previous = Some((snapshot, total, now_ms));
        (total, reset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loop_wrap_is_unwrapped_and_a_seek_starts_a_new_timeline() {
        let mut clock = UnwrapClock::default();
        let mut s = Snapshot {
            playing: true,
            pulse: 7600,
            loop_start: 0,
            loop_end: 7680,
            looping: true,
        };
        assert_eq!(clock.observe(s, 1000, 120.0), (7600, true));
        s.pulse = 20;
        assert_eq!(clock.observe(s, 1050, 120.0), (7700, false));
        s.pulse = 3000;
        assert_eq!(clock.observe(s, 1100, 120.0), (3000, true));
        s.playing = false;
        assert_eq!(clock.observe(s, 1150, 120.0), (3000, true));
    }

    #[test]
    fn loop_endpoint_can_be_observed_before_position_wraps() {
        let mut clock = UnwrapClock::default();
        let mut s = Snapshot {
            playing: true,
            pulse: 7680,
            loop_start: 0,
            loop_end: 7680,
            looping: true,
        };
        assert_eq!(clock.observe(s, 1000, 128.0), (7680, true));
        s.pulse = 6;
        assert_eq!(clock.observe(s, 1003, 128.0), (7686, false));
    }
}
