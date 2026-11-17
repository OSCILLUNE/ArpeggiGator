//! Safe emulator observer for the repaint-to-paint handoff.
//!
//! This probe measures callback delivery and thread identity only. It never
//! stores or dereferences PadEvent.input / PadEvent.properties and never calls
//! sites::send_note_on or sites::send_note_off. It is not a playback test.

#![allow(dead_code)]

use core::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, AtomicU64, Ordering, Ordering::Release};
use std::{
    thread,
    time::{Duration, Instant},
};

#[cfg(not(all(target_os = "linux", target_arch = "aarch64")))]
use std::hash::{Hash, Hasher};

use crate::REPAINT_REQUESTED;
use mpc_sample_sdk::{PadAction, PadEvent, draw};

/// Exported diagnostics read by scripts/emulator_repaint_observer.py.
#[repr(C)]
pub struct ProbeDiagnostics {
    pub pad_thread_id: AtomicU64,
    pub paint_thread_id: AtomicU64,
    pub worker_thread_id: AtomicU64,
    pub source: AtomicI32,
    pub pad: AtomicI32,
    pub velocity_bits: AtomicU32,
    pub repaint_requests: AtomicU32,
    pub repaint_callbacks: AtomicU32,
    pub worker_started: AtomicU32,
    pub worker_finished: AtomicU32,
    pub worker_timeouts: AtomicU32,
    pub callback_latency_ms: AtomicI32,
    pub paint_interval_min_us: AtomicU64,
    pub paint_interval_max_us: AtomicU64,
    pub paint_interval_sum_us: AtomicU64,
    pub paint_interval_count: AtomicU32,
    pub button_last: AtomicU32,
    pub button_b1_press: AtomicU32,
    pub button_b2_press: AtomicU32,
    pub button_levels_press: AtomicU32,
}

impl ProbeDiagnostics {
    const fn new() -> Self {
        Self {
            pad_thread_id: AtomicU64::new(0),
            paint_thread_id: AtomicU64::new(0),
            worker_thread_id: AtomicU64::new(0),
            source: AtomicI32::new(0),
            pad: AtomicI32::new(-1),
            velocity_bits: AtomicU32::new(0),
            repaint_requests: AtomicU32::new(0),
            repaint_callbacks: AtomicU32::new(0),
            worker_started: AtomicU32::new(0),
            worker_finished: AtomicU32::new(0),
            worker_timeouts: AtomicU32::new(0),
            callback_latency_ms: AtomicI32::new(-1),
            paint_interval_min_us: AtomicU64::new(u64::MAX),
            paint_interval_max_us: AtomicU64::new(0),
            paint_interval_sum_us: AtomicU64::new(0),
            paint_interval_count: AtomicU32::new(0),
            button_last: AtomicU32::new(0),
            button_b1_press: AtomicU32::new(0),
            button_b2_press: AtomicU32::new(0),
            button_levels_press: AtomicU32::new(0),
        }
    }
}

/// Store a compact snapshot for the emulator control probe.
///
/// Bits: code 0–7, pressed 8, SHIFT 9, 16 Levels 10, action 11–15.
pub fn button_event(code: u32, pressed: bool, shifted: bool, in_16_levels: bool, action: u32) {
    let snapshot = encode_button_snapshot(code, pressed, shifted, in_16_levels, action);
    g_probe.button_last.store(snapshot, Ordering::Release);
    if pressed {
        match code {
            12 => g_probe.button_b1_press.store(snapshot, Ordering::Release),
            13 => g_probe.button_b2_press.store(snapshot, Ordering::Release),
            40 => g_probe
                .button_levels_press
                .store(snapshot, Ordering::Release),
            _ => {}
        }
    }
}

fn encode_button_snapshot(
    code: u32,
    pressed: bool,
    shifted: bool,
    in_16_levels: bool,
    action: u32,
) -> u32 {
    (code & 0xff)
        | (u32::from(pressed) << 8)
        | (u32::from(shifted) << 9)
        | (u32::from(in_16_levels) << 10)
        | ((action & 0x1f) << 11)
}

mpc_sample_sdk::export_static!(
    /// Emulator observer diagnostics. The repr(C) layout is fixed for the script.
    g_probe: ProbeDiagnostics = ProbeDiagnostics::new()
);

static WORKER_ACTIVE: AtomicBool = AtomicBool::new(false);
static SOURCE_PAD_HELD: AtomicBool = AtomicBool::new(false);
static REQUEST_PENDING: AtomicBool = AtomicBool::new(false);
static CADENCE_ACTIVE: AtomicBool = AtomicBool::new(false);
static REQUESTED_AT_NS: AtomicU64 = AtomicU64::new(0);
static LAST_PAINT_NS: AtomicU64 = AtomicU64::new(0);

const REQUEST_DELAY: Duration = Duration::from_millis(250);
const REPAINT_INTERVAL: Duration = Duration::from_millis(4);
const REPAINT_REQUESTS: u32 = 128;
const CALLBACK_DRAIN: Duration = Duration::from_millis(750);

#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
fn current_thread_id() -> u64 {
    let mut result = 0_usize;
    // Linux AArch64 gettid syscall; the observer uses the ID only for logging.
    unsafe {
        core::arch::asm!(
            "svc 0",
            inlateout("x0") result,
            in("x8") 178_usize,
            options(nostack)
        );
    }
    result as u64
}

#[cfg(not(all(target_os = "linux", target_arch = "aarch64")))]
fn current_thread_id() -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    thread::current().id().hash(&mut hasher);
    hasher.finish().max(1)
}

fn monotonic_time_ns() -> u64 {
    static EPOCH: std::sync::OnceLock<Instant> = std::sync::OnceLock::new();
    EPOCH
        .get_or_init(Instant::now)
        .elapsed()
        .as_nanos()
        .min(u128::from(u64::MAX)) as u64
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
        .store(current_thread_id(), Ordering::Release);
    g_probe.source.store(event.source, Ordering::Release);
    g_probe.pad.store(event.pad, Ordering::Release);
    g_probe
        .velocity_bits
        .store(event.velocity.to_bits(), Ordering::Release);

    SOURCE_PAD_HELD.store(true, Ordering::Release);
    REQUEST_PENDING.store(false, Ordering::Release);
    CADENCE_ACTIVE.store(false, Ordering::Release);
    REQUESTED_AT_NS.store(0, Ordering::Release);
    LAST_PAINT_NS.store(0, Ordering::Release);
    g_probe.repaint_requests.store(0, Ordering::Release);
    g_probe.repaint_callbacks.store(0, Ordering::Release);
    g_probe.worker_timeouts.store(0, Ordering::Release);
    g_probe.callback_latency_ms.store(-1, Ordering::Release);
    g_probe
        .paint_interval_min_us
        .store(u64::MAX, Ordering::Release);
    g_probe.paint_interval_max_us.store(0, Ordering::Release);
    g_probe.paint_interval_sum_us.store(0, Ordering::Release);
    g_probe.paint_interval_count.store(0, Ordering::Release);

    let spawned = thread::Builder::new()
        .name("arpeggigator-repaint-cadence".to_string())
        .spawn(|| {
            let _ = monotonic_time_ns();
            g_probe
                .worker_thread_id
                .store(current_thread_id(), Ordering::Release);
            g_probe.worker_started.fetch_add(1, Ordering::AcqRel);
            thread::sleep(REQUEST_DELAY);

            // The source pad must remain down for the requested observation.
            if SOURCE_PAD_HELD.load(Ordering::Acquire) {
                CADENCE_ACTIVE.store(true, Ordering::Release);
                for request in 0..REPAINT_REQUESTS {
                    if !SOURCE_PAD_HELD.load(Ordering::Acquire) {
                        break;
                    }

                    if request == 0 {
                        REQUESTED_AT_NS.store(monotonic_time_ns(), Ordering::Release);
                        REQUEST_PENDING.store(true, Ordering::Release);
                    }
                    g_probe.repaint_requests.fetch_add(1, Ordering::AcqRel);

                    // Request a repaint on the UI thread instead of calling draw::repaint_painted()
                    // directly from this background thread, which caused heap corruption.
                    crate::REPAINT_REQUESTED.store(true, Release);
                    thread::sleep(REPAINT_INTERVAL);
                }

                thread::sleep(CALLBACK_DRAIN);
                CADENCE_ACTIVE.store(false, Ordering::Release);
                REQUEST_PENDING.store(false, Ordering::Release);
                if g_probe.repaint_callbacks.load(Ordering::Acquire) == 0 {
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
    let first_request_callback = REQUEST_PENDING.swap(false, Ordering::AcqRel);
    let cadence_active = CADENCE_ACTIVE.load(Ordering::Acquire);
    if !first_request_callback && !cadence_active {
        return;
    }

    let now_ns = monotonic_time_ns();
    g_probe
        .paint_thread_id
        .store(current_thread_id(), Ordering::Release);

    if first_request_callback {
        let requested_ns = REQUESTED_AT_NS.load(Ordering::Acquire);
        let latency_ms = now_ns
            .saturating_sub(requested_ns)
            .checked_div(1_000_000)
            .unwrap_or(0)
            .min(i32::MAX as u64) as i32;
        g_probe
            .callback_latency_ms
            .store(latency_ms, Ordering::Release);
    }

    if cadence_active {
        let previous_ns = LAST_PAINT_NS.swap(now_ns, Ordering::AcqRel);
        if previous_ns != 0 && now_ns >= previous_ns {
            let interval_us = (now_ns - previous_ns) / 1_000;
            if interval_us != 0 {
                g_probe
                    .paint_interval_min_us
                    .fetch_min(interval_us, Ordering::Relaxed);
                g_probe
                    .paint_interval_max_us
                    .fetch_max(interval_us, Ordering::Relaxed);
                g_probe
                    .paint_interval_sum_us
                    .fetch_add(interval_us, Ordering::Relaxed);
                g_probe.paint_interval_count.fetch_add(1, Ordering::Relaxed);
            }
        }
        g_probe.repaint_callbacks.fetch_add(1, Ordering::AcqRel);
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Duration, ProbeDiagnostics, REPAINT_INTERVAL, REPAINT_REQUESTS, current_thread_id,
        encode_button_snapshot,
    };

    #[test]
    fn thread_id_is_stable_on_one_thread() {
        assert_eq!(current_thread_id(), current_thread_id());
    }

    #[test]
    fn button_snapshot_keeps_code_and_control_state() {
        let snapshot = encode_button_snapshot(12, true, true, true, 1);
        assert_eq!(snapshot & 0xff, 12);
        assert_ne!(snapshot & (1 << 8), 0);
        assert_ne!(snapshot & (1 << 9), 0);
        assert_ne!(snapshot & (1 << 10), 0);
        assert_eq!((snapshot >> 11) & 0x1f, 1);
    }

    #[test]
    fn exported_diagnostics_have_the_expected_c_layout() {
        assert_eq!(core::mem::size_of::<ProbeDiagnostics>(), 112);
    }

    #[test]
    fn repaint_cadence_is_bounded_and_matches_the_observation_target() {
        assert_eq!(REPAINT_INTERVAL, Duration::from_millis(4));
        assert_eq!(REPAINT_REQUESTS, 128);
    }
}
