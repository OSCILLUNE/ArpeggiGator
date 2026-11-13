//! Emulator-only probe for the repaint-to-paint dispatch candidate.
//!
//! This module intentionally delays a real PadEvent's native input pointers and
//! asks the SDK repaint path to get back into on_paint. It then calls the native
//! note hooks from on_paint. This is reverse-engineering instrumentation only:
//! never merge this module into main or install this branch on physical hardware.

use core::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, AtomicU64, Ordering};
use std::{
    hash::{Hash, Hasher},
    thread,
    time::{Duration, Instant},
};

use mpc_sample_sdk::{PadAction, PadEvent, draw, sites};

/// Exported diagnostics read by scripts/emulator_repaint_probe.py.
#[repr(C)]
pub struct ProbeDiagnostics {
    pub pad_thread_id: AtomicU64,
    pub paint_thread_id: AtomicU64,
    pub input_ptr: AtomicU64,
    pub properties_ptr: AtomicU64,
    pub source: AtomicI32,
    pub pad: AtomicI32,
    pub velocity_bits: AtomicU32,
    pub note_on_attempts: AtomicU32,
    pub note_on_succeeded: AtomicU32,
    pub note_off_attempts: AtomicU32,
    pub invalid_contexts: AtomicU32,
    pub worker_started: AtomicU32,
    pub worker_finished: AtomicU32,
    pub worker_timeouts: AtomicU32,
}

impl ProbeDiagnostics {
    const fn new() -> Self {
        Self {
            pad_thread_id: AtomicU64::new(0),
            paint_thread_id: AtomicU64::new(0),
            input_ptr: AtomicU64::new(0),
            properties_ptr: AtomicU64::new(0),
            source: AtomicI32::new(0),
            pad: AtomicI32::new(-1),
            velocity_bits: AtomicU32::new(0),
            note_on_attempts: AtomicU32::new(0),
            note_on_succeeded: AtomicU32::new(0),
            note_off_attempts: AtomicU32::new(0),
            invalid_contexts: AtomicU32::new(0),
            worker_started: AtomicU32::new(0),
            worker_finished: AtomicU32::new(0),
            worker_timeouts: AtomicU32::new(0),
        }
    }
}

mpc_sample_sdk::export_static!(
    /// Emulator probe diagnostics. Layout is repr(C); see this module.
    g_probe: ProbeDiagnostics = ProbeDiagnostics::new()
);

static WORKER_ACTIVE: AtomicBool = AtomicBool::new(false);
static SOURCE_PAD_HELD: AtomicBool = AtomicBool::new(false);
static REQUEST_NOTE_ON: AtomicBool = AtomicBool::new(false);
static REQUEST_NOTE_OFF: AtomicBool = AtomicBool::new(false);
static NOTE_ON_COMPLETED: AtomicBool = AtomicBool::new(false);
static NOTE_OFF_COMPLETED: AtomicBool = AtomicBool::new(false);

const NOTE_ON_DELAY: Duration = Duration::from_millis(250);
const NOTE_GATE: Duration = Duration::from_millis(120);
const HANDOFF_TIMEOUT: Duration = Duration::from_secs(2);

fn current_thread_fingerprint() -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    thread::current().id().hash(&mut hasher);
    hasher.finish().max(1)
}

/// Capture one real pad press and schedule the intentionally delayed probe.
pub fn capture_and_schedule(event: PadEvent) {
    if event.action != PadAction::Press {
        return;
    }

    if event.input == 0 || event.properties == 0 || !(0..16).contains(&event.pad) {
        g_probe.invalid_contexts.fetch_add(1, Ordering::AcqRel);
        return;
    }

    if WORKER_ACTIVE
        .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
        .is_err()
    {
        return;
    }

    g_probe
        .pad_thread_id
        .store(current_thread_fingerprint(), Ordering::Release);
    g_probe.input_ptr.store(event.input as u64, Ordering::Release);
    g_probe
        .properties_ptr
        .store(event.properties as u64, Ordering::Release);
    g_probe.source.store(event.source, Ordering::Release);
    g_probe.pad.store(event.pad, Ordering::Release);
    g_probe
        .velocity_bits
        .store(event.velocity.to_bits(), Ordering::Release);

    SOURCE_PAD_HELD.store(true, Ordering::Release);
    REQUEST_NOTE_ON.store(false, Ordering::Release);
    REQUEST_NOTE_OFF.store(false, Ordering::Release);
    NOTE_ON_COMPLETED.store(false, Ordering::Release);
    NOTE_OFF_COMPLETED.store(false, Ordering::Release);

    let spawned = thread::Builder::new()
        .name("arpeggigator-probe".to_string())
        .spawn(|| {
            g_probe.worker_started.fetch_add(1, Ordering::AcqRel);
            thread::sleep(NOTE_ON_DELAY);

            // The physical source pad must still be down; otherwise the probe
            // does not create a note after the user has let go.
            if SOURCE_PAD_HELD.load(Ordering::Acquire) {
                REQUEST_NOTE_ON.store(true, Ordering::Release);
                draw::repaint_painted();

                let deadline = Instant::now() + HANDOFF_TIMEOUT;
                while !NOTE_ON_COMPLETED.load(Ordering::Acquire) && Instant::now() < deadline {
                    thread::sleep(Duration::from_millis(1));
                }

                if NOTE_ON_COMPLETED.load(Ordering::Acquire)
                    && g_probe.note_on_succeeded.load(Ordering::Acquire) != 0
                {
                    thread::sleep(NOTE_GATE);
                    REQUEST_NOTE_OFF.store(true, Ordering::Release);
                    draw::repaint_painted();

                    let off_deadline = Instant::now() + HANDOFF_TIMEOUT;
                    while !NOTE_OFF_COMPLETED.load(Ordering::Acquire)
                        && Instant::now() < off_deadline
                    {
                        thread::sleep(Duration::from_millis(1));
                    }
                    if !NOTE_OFF_COMPLETED.load(Ordering::Acquire) {
                        g_probe.worker_timeouts.fetch_add(1, Ordering::AcqRel);
                    }
                } else if !NOTE_ON_COMPLETED.load(Ordering::Acquire) {
                    g_probe.worker_timeouts.fetch_add(1, Ordering::AcqRel);
                }
            }

            g_probe.worker_finished.fetch_add(1, Ordering::AcqRel);
            WORKER_ACTIVE.store(false, Ordering::Release);
        });

    if spawned.is_err() {
        WORKER_ACTIVE.store(false, Ordering::Release);
        g_probe.worker_timeouts.fetch_add(1, Ordering::AcqRel);
    }
}

/// Track release of the physical source pad.
pub fn source_pad_released(event: PadEvent) {
    if event.action == PadAction::Release
        && g_probe.pad.load(Ordering::Acquire) == event.pad
    {
        SOURCE_PAD_HELD.store(false, Ordering::Release);
    }
}

/// Called from the SDK's native Sample-page paint hook.
pub fn on_paint() {
    g_probe
        .paint_thread_id
        .store(current_thread_fingerprint(), Ordering::Release);

    if REQUEST_NOTE_ON.swap(false, Ordering::AcqRel) {
        let input = g_probe.input_ptr.load(Ordering::Acquire) as usize;
        let properties = g_probe.properties_ptr.load(Ordering::Acquire) as usize;
        let source = g_probe.source.load(Ordering::Acquire);
        let pad = g_probe.pad.load(Ordering::Acquire);
        let velocity = f32::from_bits(g_probe.velocity_bits.load(Ordering::Acquire));

        g_probe.note_on_attempts.fetch_add(1, Ordering::AcqRel);
        if input == 0 || properties == 0 || !(0..16).contains(&pad) || !sites::notes_ready() {
            g_probe.invalid_contexts.fetch_add(1, Ordering::AcqRel);
            NOTE_ON_COMPLETED.store(true, Ordering::Release);
        } else {
            // SAFETY (EMULATOR-ONLY EXPERIMENT): these native pointers came from
            // the original PadEvent, but are deliberately reused 250 ms later.
            // The experiment exists to test their lifetime and callback thread.
            // Never ship this call without a passing emulator probe and review.
            let sent = unsafe {
                sites::send_note_on(input, source, pad, properties, velocity)
            };
            g_probe
                .note_on_succeeded
                .store(u32::from(sent), Ordering::Release);
            NOTE_ON_COMPLETED.store(true, Ordering::Release);
        }
    }

    if REQUEST_NOTE_OFF.swap(false, Ordering::AcqRel) {
        let input = g_probe.input_ptr.load(Ordering::Acquire) as usize;
        let source = g_probe.source.load(Ordering::Acquire);
        let pad = g_probe.pad.load(Ordering::Acquire);

        if input != 0 && (0..16).contains(&pad) && sites::notes_ready() {
            g_probe.note_off_attempts.fetch_add(1, Ordering::AcqRel);
            // SAFETY (EMULATOR-ONLY EXPERIMENT): matching note-off is invoked
            // from the native paint callback using the captured input object.
            unsafe { sites::send_note_off(input, source, pad) };
        }
        NOTE_OFF_COMPLETED.store(true, Ordering::Release);
    }
}

#[cfg(test)]
mod tests {
    use super::current_thread_fingerprint;

    #[test]
    fn thread_fingerprint_is_stable_on_one_thread() {
        assert_eq!(current_thread_fingerprint(), current_thread_fingerprint());
    }
}
