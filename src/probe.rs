//! Safe emulator observer for the repaint-to-paint handoff.
//!
//! This probe measures callback delivery and thread identity only. It never
//! stores or dereferences PadEvent.input / PadEvent.properties and never calls
//! sites::send_note_on or sites::send_note_off. It is not a playback test.

use core::sync::atomic::{
    AtomicBool, AtomicI32, AtomicI64, AtomicU32, AtomicU64, Ordering,
};
use std::{
    hash::{Hash, Hasher},
    thread,
    time::{Duration, Instant},
};

use mpc_sample_sdk::{draw, platform::clock, PadAction, PadEvent};

/// Exported diagnostics read by scripts/emulator_repaint_observer.py.
#[repr(C)]
pub struct ProbeDiagnostics {
    pub pad_thread_id: AtomicU64,
    pub paint_thread_id: AtomicU64,
    pub source: AtomicI32,
    pub pad: AtomicI32,
    pub velocity_bits: AtomicU32,
    pub repaint_requests: AtomicU32,
    pub repaint_callbacks: AtomicU32,
    pub worker_started: AtomicU32,
    pub worker_finished: AtomicU32,
    pub worker_timeouts: AtomicU32,
    pub callback_latency_ms: AtomicI32,
}

impl ProbeDiagnostics {
    const fn new() -> Self {
        Self {
            pad_thread_id: AtomicU64::new(0),
            paint_thread_id: AtomicU64::new(0),
            source: AtomicI32::new(0),
            pad: AtomicI32::new(-1),
            velocity_bits: AtomicU32::new(0),
            repaint_requests: AtomicU32::new(0),
            repaint_callbacks: AtomicU32::new(0),
            worker_started: AtomicU32::new(0),
            worker_finished: AtomicU32::new(0),
            worker_timeouts: AtomicU32::new(0),
            callback_latency_ms: AtomicI32::new(-1),
        }
    }
}

mpc_sample_sdk::export_static!(
    /// Emulator observer diagnostics. The repr(C) layout is fixed for the script.
    g_probe: ProbeDiagnostics = ProbeDiagnostics::new()
);

static WORKER_ACTIVE: AtomicBool = AtomicBool::new(false);
static SOURCE_PAD_HELD: AtomicBool = AtomicBool::new(false);
static REQUEST_PENDING: AtomicBool = AtomicBool::new(false);
static PAINT_COMPLETED: AtomicBool = AtomicBool::new(false);
static REQUESTED_AT_MS: AtomicI64 = AtomicI64::new(0);

const REQUEST_DELAY: Duration = Duration::from_millis(250);
const HANDOFF_TIMEOUT: Duration = Duration::from_secs(2);

fn current_thread_fingerprint() -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    thread::current().id().hash(&mut hasher);
    hasher.finish().max(1)
}

/// Capture scalar event data and schedule one repaint request.
///
/// The native input and properties addresses are intentionally not copied.
pub fn capture_and_schedule(event: PadEvent) {
    if event.action != PadAction::Press || !(0..16).contains(&event.pad) {
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
    g_probe.source.store(event.source, Ordering::Release);
    g_probe.pad.store(event.pad, Ordering::Release);
    g_probe
        .velocity_bits
        .store(event.velocity.to_bits(), Ordering::Release);

    SOURCE_PAD_HELD.store(true, Ordering::Release);
    REQUEST_PENDING.store(false, Ordering::Release);
    PAINT_COMPLETED.store(false, Ordering::Release);
    REQUESTED_AT_MS.store(0, Ordering::Release);

    let spawned = thread::Builder::new()
        .name("arpeggigator-observer".to_string())
        .spawn(|| {
            g_probe.worker_started.fetch_add(1, Ordering::AcqRel);
            thread::sleep(REQUEST_DELAY);

            // The source pad must remain down for the requested observation.
            if SOURCE_PAD_HELD.load(Ordering::Acquire) {
                PAINT_COMPLETED.store(false, Ordering::Release);
                REQUESTED_AT_MS.store(clock::now_ms(), Ordering::Release);
                REQUEST_PENDING.store(true, Ordering::Release);
                g_probe.repaint_requests.fetch_add(1, Ordering::AcqRel);

                // This exercises only the SDK repaint path. No note hook is called.
                draw::repaint_painted();

                let deadline = Instant::now() + HANDOFF_TIMEOUT;
                while !PAINT_COMPLETED.load(Ordering::Acquire) && Instant::now() < deadline {
                    thread::sleep(Duration::from_millis(1));
                }

                if !PAINT_COMPLETED.load(Ordering::Acquire) {
                    REQUEST_PENDING.store(false, Ordering::Release);
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
    if event.action == PadAction::Release && g_probe.pad.load(Ordering::Acquire) == event.pad {
        SOURCE_PAD_HELD.store(false, Ordering::Release);
    }
}

/// Called from the native Sample-page paint hook.
pub fn on_paint() {
    if REQUEST_PENDING.swap(false, Ordering::AcqRel) {
        let now = clock::now_ms();
        let requested = REQUESTED_AT_MS.load(Ordering::Acquire);
        let latency = now.saturating_sub(requested).clamp(0, i64::from(i32::MAX));

        g_probe
            .paint_thread_id
            .store(current_thread_fingerprint(), Ordering::Release);
        g_probe
            .callback_latency_ms
            .store(i32::try_from(latency).unwrap_or(i32::MAX), Ordering::Release);
        g_probe.repaint_callbacks.fetch_add(1, Ordering::AcqRel);
        PAINT_COMPLETED.store(true, Ordering::Release);
    }
}

#[cfg(test)]
mod tests {
    use super::{ProbeDiagnostics, current_thread_fingerprint};

    #[test]
    fn thread_fingerprint_is_stable_on_one_thread() {
        assert_eq!(current_thread_fingerprint(), current_thread_fingerprint());
    }

    #[test]
    fn exported_diagnostics_have_the_expected_c_layout() {
        assert_eq!(core::mem::size_of::<ProbeDiagnostics>(), 56);
    }
}
