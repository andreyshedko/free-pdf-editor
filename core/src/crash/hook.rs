//! Installs the global panic hook that persists crash reports to disk.

use super::report::CrashReport;
use std::path::PathBuf;

/// Directory where crash JSON files are written.
pub fn crash_dir() -> PathBuf {
    let base = dirs_next::home_dir().unwrap_or_else(|| PathBuf::from("."));
    base.join(".freepdfeditor").join("crashes")
}

/// Install a panic hook that writes a `CrashReport` JSON file on every panic.
///
/// The previous hook (Rust's default backtrace handler or any earlier hook) is
/// captured via `take_hook()` and invoked **after** persisting the crash file,
/// so normal panic output and backtraces are preserved.
///
/// Should be called once at application startup, before spawning any threads.
pub fn install_panic_hook() {
    // Capture (and remove) whatever hook is currently installed so we can
    // chain into it after writing our crash file.
    let previous_hook = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |info| {
        let thread = std::thread::current();
        let thread_name = thread.name().unwrap_or("<unnamed>").to_string();

        let panic_message = if let Some(s) = info.payload().downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "unknown panic".to_string()
        };

        let panic_location = info
            .location()
            .map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()));

        let memory_bytes = resident_memory_bytes();

        let report = CrashReport::new(thread_name, panic_message, panic_location, memory_bytes);

        if let Ok(json) = serde_json::to_string_pretty(&report) {
            let dir = crash_dir();
            if std::fs::create_dir_all(&dir).is_ok() {
                let path = dir.join(format!("{}.json", report.crash_id));
                let _ = std::fs::write(path, json);
            }
        }

        // Invoke the previous hook so Rust's standard panic output / backtraces
        // are still printed to stderr and the OS crash reporter can catch them.
        previous_hook(info);
    }));
}

/// Best-effort resident memory in bytes (Linux /proc/self/status).
fn resident_memory_bytes() -> Option<u64> {
    #[cfg(target_os = "linux")]
    {
        let status = std::fs::read_to_string("/proc/self/status").ok()?;
        for line in status.lines() {
            if line.starts_with("VmRSS:") {
                let kb: u64 = line.split_whitespace().nth(1)?.parse().ok()?;
                return Some(kb * 1024);
            }
        }
        None
    }
    #[cfg(not(target_os = "linux"))]
    {
        None
    }
}
