//! Read-only current-project BPM discovery.
//!
//! MPC Sample keeps its active working project at Temp-Project.xpj. This file
//! is gzip-compressed and contains an ACVS header followed by JSON. The parser
//! never writes to it. If the project file is missing or changing, BPM is
//! reported as unavailable until a valid read succeeds.

use std::{
    fs::{self, File},
    io::{Read, Write},
    sync::atomic::{AtomicBool, AtomicU32, Ordering},
    thread,
    time::{Duration, SystemTime},
};

use flate2::read::GzDecoder;
use serde_json::Value;

const CURRENT_PROJECT: &str = "/data/TempProjects/Temp-Project.xpj";
static MONITOR_STARTED: AtomicBool = AtomicBool::new(false);
static BPM_BITS: AtomicU32 = AtomicU32::new(0);
static BPM_VALID: AtomicBool = AtomicBool::new(false);

/// Start a small read-only monitor for the active project BPM.
///
/// The worker only reads a short project header and JSON document every
/// 500 ms. It never touches the MPC model or writes to the project.
pub fn start_monitor() {
    if MONITOR_STARTED.swap(true, Ordering::AcqRel) {
        return;
    }

    let result = thread::Builder::new()
        .name("arpeggigator-tempo".to_string())
        .spawn(|| {
            let mut previous_modified: Option<SystemTime> = None;
            let mut first_scan = true;
            loop {
                let modified = fs::metadata(CURRENT_PROJECT)
                    .and_then(|metadata| metadata.modified())
                    .ok();

                if first_scan || modified != previous_modified {
                    first_scan = false;
                    previous_modified = modified;
                    match read_current_project_bpm() {
                        Some(bpm) => {
                            BPM_BITS.store(bpm.to_bits(), Ordering::Release);
                            BPM_VALID.store(true, Ordering::Release);
                        }
                        None => BPM_VALID.store(false, Ordering::Release),
                    }
                }

                thread::sleep(Duration::from_millis(500));
            }
        });

    if result.is_err() {
        MONITOR_STARTED.store(false, Ordering::Release);
    }
}

/// Current active project BPM, or None when it cannot be read reliably.
#[must_use]
pub fn current_bpm() -> Option<f32> {
    if !BPM_VALID.load(Ordering::Acquire) {
        return None;
    }
    let bpm = f32::from_bits(BPM_BITS.load(Ordering::Acquire));
    (bpm.is_finite() && (20.0..=400.0).contains(&bpm)).then_some(bpm)
}

fn read_current_project_bpm() -> Option<f32> {
    let file = File::open(CURRENT_PROJECT).ok()?;
    parse_compressed_project(file)
}

fn parse_compressed_project(reader: impl Read) -> Option<f32> {
    let mut decoder = GzDecoder::new(reader);
    let mut bytes = Vec::new();
    decoder.read_to_end(&mut bytes).ok()?;

    let json_start = bytes.iter().position(|byte| *byte == b'{')?;
    let document: Value = serde_json::from_slice(bytes.get(json_start..)?).ok()?;
    parse_project_document(&document)
}

fn parse_project_document(document: &Value) -> Option<f32> {
    let data = document.get("data")?;

    let bpm = if data
        .get("masterTempoEnabled")
        .and_then(Value::as_bool)
        .unwrap_or(false)
    {
        data.get("masterTempo")?.as_f64()?
    } else {
        let current_sequence = data.get("currentSequence")?.as_u64()?;
        let sequences = data.get("sequences")?.as_array()?;
        let sequence = sequences
            .iter()
            .find(|entry| entry.get("key").and_then(Value::as_u64) == Some(current_sequence))?;
        sequence.get("value")?.get("bpm")?.as_f64()?
    };

    if !bpm.is_finite() || !(20.0..=400.0).contains(&bpm) {
        return None;
    }
    let bpm = bpm as f32;
    bpm.is_finite().then_some(bpm)
}

#[cfg(test)]
mod tests {
    use flate2::{Compression, write::GzEncoder};
    use serde_json::json;

    use super::{parse_compressed_project, parse_project_document};

    #[test]
    fn reads_json_after_the_acvs_gzip_header() {
        let json = br#"{"data":{"masterTempoEnabled":true,"masterTempo":126.0,"currentSequence":0,"sequences":[]}}"#;
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder
            .write_all(b"ACVS\n1.3.0.12\nSerialisableProjectData\njson\nLinux\n")
            .expect("write ACVS header");
        encoder.write_all(json).expect("write project JSON");
        let compressed = encoder.finish().expect("finish gzip project");

        assert_eq!(parse_compressed_project(compressed.as_slice()), Some(126.0));
    }

    #[test]
    fn current_sequence_is_selected_by_key_not_array_position() {
        let project = json!({
            "data": {
                "masterTempoEnabled": false,
                "masterTempo": 128.0,
                "currentSequence": 7,
                "sequences": [
                    {"key": 0, "value": {"bpm": 90.0}},
                    {"key": 7, "value": {"bpm": 123.5}}
                ]
            }
        });
        assert_eq!(parse_project_document(&project), Some(123.5));
    }

    #[test]
    fn global_tempo_overrides_sequence_tempo_when_enabled() {
        let project = json!({
            "data": {
                "masterTempoEnabled": true,
                "masterTempo": 101.25,
                "currentSequence": 4,
                "sequences": [
                    {"key": 4, "value": {"bpm": 140.0}}
                ]
            }
        });
        assert_eq!(parse_project_document(&project), Some(101.25));
    }

    #[test]
    fn unavailable_or_implausible_tempo_is_rejected() {
        let missing = json!({"data": {"currentSequence": 0, "sequences": []}});
        assert_eq!(parse_project_document(&missing), None);

        let outside_range = json!({
            "data": {
                "masterTempoEnabled": true,
                "masterTempo": 999.0,
                "currentSequence": 0,
                "sequences": []
            }
        });
        assert_eq!(parse_project_document(&outside_range), None);
    }
}
