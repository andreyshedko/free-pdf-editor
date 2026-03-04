//! Uploads pending crash reports to the analytics backend.
//!
//! Called once at application startup (after the panic hook is installed).
//! Any `.json` files found in the crash directory are submitted via
//! HTTPS POST and deleted on success.  The operation is best-effort;
//! failures are logged but never propagate to the caller.

use super::hook::crash_dir;
use super::report::CrashReport;
use std::path::Path;
use tracing::{debug, warn};

/// Endpoint where crash reports are posted.
/// Override at compile time via `CRASH_ENDPOINT` env (set in `build.rs`).
const CRASH_ENDPOINT: &str = env!("CRASH_ENDPOINT");

/// Upload all pending crash report files from the crash directory.
///
/// This function is intentionally non-async and spawns a **detached** background
/// thread so the caller (including `main`) is not blocked.  The thread is
/// fire-and-forget: if the application exits before all reports are uploaded,
/// the remaining files are left on disk and will be retried on the next launch.
pub fn upload_pending_crashes() {
    let dir = crash_dir();
    if !dir.exists() {
        return;
    }

    std::thread::Builder::new()
        .name("crash-uploader".into())
        .spawn(move || {
            let Ok(entries) = std::fs::read_dir(&dir) else {
                return;
            };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("json") {
                    try_upload_and_delete(&path);
                }
            }
        })
        .ok();
}

fn try_upload_and_delete(path: &Path) {
    let Ok(raw) = std::fs::read_to_string(path) else {
        return;
    };
    let Ok(report) = serde_json::from_str::<CrashReport>(&raw) else {
        warn!("Skipping malformed crash report: {}", path.display());
        return;
    };

    debug!(
        "Uploading crash report {} (v{})",
        report.crash_id, report.app_version
    );

    if CRASH_ENDPOINT.is_empty() {
        debug!("CRASH_ENDPOINT not configured; skipping upload and leaving crash report on disk");
        return;
    }
    match ureq::post(CRASH_ENDPOINT).send_json(&report) {
        Ok(_) => {
            debug!("Crash report {} uploaded successfully", report.crash_id);
            let _ = std::fs::remove_file(path);
        }
        Err(e) => {
            warn!("Failed to upload crash report {}: {}", report.crash_id, e);
            // Leave file on disk; will be retried next launch.
        }
    }
}
