//! Telemetry event definitions.

use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

/// A single anonymous telemetry event.
#[derive(Debug, Clone, Serialize)]
pub struct Event {
    /// Event name, e.g. "document_opened".
    pub event: String,
    /// Semantic version of the app (from build env).
    pub version: String,
    /// Target platform: "windows", "macos", or "linux".
    pub platform: String,
    /// Release channel: alpha / beta / stable.
    pub channel: String,
    /// Unix epoch timestamp (seconds since 1970-01-01T00:00:00Z).
    pub timestamp: String,
    /// Optional numeric value (e.g. duration in ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

impl Event {
    fn new(event: impl Into<String>) -> Self {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        Self {
            event: event.into(),
            version: env!("APP_VERSION").to_string(),
            platform: current_platform(),
            channel: env!("APP_CHANNEL").to_string(),
            timestamp: format!("{ts}"),
            value: None,
        }
    }

    /// Application launched successfully.
    pub fn startup_success() -> Self {
        Self::new("startup_success")
    }

    /// User opened a document.
    pub fn document_opened() -> Self {
        Self::new("document_opened")
    }

    /// A document was kept open for `duration_ms` milliseconds.
    pub fn document_duration(duration_ms: f64) -> Self {
        let mut e = Self::new("document_duration");
        e.value = Some(duration_ms);
        e
    }

    /// A recoverable error occurred in `category`.
    pub fn error_category(category: impl Into<String>) -> Self {
        let mut e = Self::new("error");
        e.value = None;
        e.event = format!("error_{}", category.into());
        e
    }
}

fn current_platform() -> String {
    #[cfg(target_os = "windows")]
    return "windows".to_string();
    #[cfg(target_os = "macos")]
    return "macos".to_string();
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    return "linux".to_string();
}
