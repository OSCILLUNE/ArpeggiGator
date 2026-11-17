//! Low-volume diagnostics for the staged 0.0.6 test builds.
//!
//! Audio threads only bump atomics. The once-per-second summary is emitted from
//! `on_paint` (UI thread) to stderr (captured in the guest journal) and to a
//! size-capped rotating file under /tmp. No allocation or file I/O on RT paths.

use core::sync::atomic::{AtomicI64, AtomicU32, AtomicU64, Ordering};

const LOG_PATH: &str = "/tmp/arpeggigator-6-events.log";
const MAX_LOG_BYTES: u64 = 128 * 1024;

static STEPS: AtomicU64 = AtomicU64::new(0);
static LAST_PAD: AtomicU32 = AtomicU32::new(0xff);
static LAST_NOTE: AtomicU32 = AtomicU32::new(0);
static LAST_PITCH: AtomicI64 = AtomicI64::new(0);
static LAST_ACTIVITY_MS: AtomicI64 = AtomicI64::new(0);
static LAST_LOG_MS: AtomicI64 = AtomicI64::new(0);

/// Called from the audio render path. Cheap atomics only.
pub fn note_on(pad: u8, note: u8, pitch: i8, now_ms: i64) {
    STEPS.fetch_add(1, Ordering::Relaxed);
    LAST_PAD.store(u32::from(pad), Ordering::Relaxed);
    LAST_NOTE.store(u32::from(note), Ordering::Relaxed);
    LAST_PITCH.store(i64::from(pitch), Ordering::Relaxed);
    LAST_ACTIVITY_MS.store(now_ms, Ordering::Relaxed);
}

/// Called from `on_paint`. Emits at most one line per second.
pub fn tick(now_ms: i64) {
    let last = LAST_LOG_MS.load(Ordering::Relaxed);
    if now_ms.saturating_sub(last) < 1_000 {
        return;
    }
    if LAST_LOG_MS
        .compare_exchange(last, now_ms, Ordering::Relaxed, Ordering::Relaxed)
        .is_err()
    {
        return;
    }
    let idle = now_ms.saturating_sub(LAST_ACTIVITY_MS.load(Ordering::Relaxed));
    let line = format!(
        "[AG6] t={now_ms} steps={} last_pad={} note={} pitch={} idle_ms={idle}\n",
        STEPS.load(Ordering::Relaxed),
        LAST_PAD.load(Ordering::Relaxed),
        LAST_NOTE.load(Ordering::Relaxed),
        LAST_PITCH.load(Ordering::Relaxed),
    );
    eprintln!("{}", line.trim_end());
    append_rotated(&line);
}

fn append_rotated(line: &str) {
    let result = (|| {
        if let Ok(meta) = std::fs::metadata(LOG_PATH) {
            if meta.len() >= MAX_LOG_BYTES {
                let _ = std::fs::remove_file(format!("{LOG_PATH}.3"));
                let _ = std::fs::rename(format!("{LOG_PATH}.2"), format!("{LOG_PATH}.3"));
                let _ = std::fs::rename(format!("{LOG_PATH}.1"), format!("{LOG_PATH}.2"));
                let _ = std::fs::rename(LOG_PATH, format!("{LOG_PATH}.1"));
            }
        }
        use std::io::Write;
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(LOG_PATH)?;
        file.write_all(line.as_bytes())
    })();
    let _ = result;
}
