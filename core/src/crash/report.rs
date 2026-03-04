//! Crash report data structure and serialisation.

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// A single crash event captured by the panic hook.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrashReport {
    /// Random UUID for this crash; never tied to a device.
    pub crash_id: String,
    /// Unix timestamp (seconds) of the crash.
    pub timestamp: u64,
    /// Application semantic version from `APP_VERSION` build env.
    pub app_version: String,
    /// Release channel (alpha/beta/stable) from `APP_CHANNEL` build env.
    pub channel: String,
    /// Build number from `APP_BUILD_NUMBER` build env.
    pub build_number: u32,
    /// Operating system name and version.
    pub os_version: String,
    /// Name of the thread that panicked.
    pub thread_name: String,
    /// Panic message.
    pub panic_message: String,
    /// File and line of the panic, if available.
    pub panic_location: Option<String>,
    /// Approximate resident memory in bytes at time of crash (best-effort).
    pub memory_bytes: Option<u64>,
}

impl CrashReport {
    /// Create a new report from the pieces captured by the panic hook.
    pub fn new(
        thread_name: impl Into<String>,
        panic_message: impl Into<String>,
        panic_location: Option<String>,
        memory_bytes: Option<u64>,
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            crash_id: simple_uuid(),
            timestamp,
            app_version: option_env!("APP_VERSION")
                .unwrap_or("dev")
                .to_string(),
            channel: option_env!("APP_CHANNEL")
                .unwrap_or("dev")
                .to_string(),
            build_number: option_env!("APP_BUILD_NUMBER")
                .and_then(|s| s.parse().ok())
                .unwrap_or(0),
            os_version: os_version_string(),
            thread_name: thread_name.into(),
            panic_message: panic_message.into(),
            panic_location,
            memory_bytes,
        }
    }
}

/// Minimal UUID v4 without an external crate.
fn simple_uuid() -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::SystemTime;

    let mut h = DefaultHasher::new();
    SystemTime::now().hash(&mut h);
    std::thread::current().id().hash(&mut h);
    let a = h.finish();
    h.write_u64(a);
    let b = h.finish();

    format!("{:016x}-{:016x}", a, b)
}

fn os_version_string() -> String {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("sw_vers")
            .arg("-productVersion")
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| format!("macOS {}", s.trim()))
            .unwrap_or_else(|| "macOS unknown".to_string())
    }
    #[cfg(target_os = "windows")]
    {
        "Windows".to_string()
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        "Linux".to_string()
    }
}
