//! Telemetry settings loaded from `settings.json`.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tracing::warn;

/// User-facing telemetry preferences.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetrySettings {
    /// When `false` (the default) no events are ever transmitted.
    #[serde(default)]
    pub telemetry_enabled: bool,
}

impl Default for TelemetrySettings {
    fn default() -> Self {
        Self {
            telemetry_enabled: false,
        }
    }
}

impl TelemetrySettings {
    /// Load settings from `settings_dir/settings.json`.
    /// Falls back to the default (disabled) on any error.
    pub fn load(settings_dir: &Path) -> Self {
        let path: PathBuf = settings_dir.join("settings.json");
        let Ok(raw) = std::fs::read_to_string(&path) else {
            return Self::default();
        };
        match serde_json::from_str::<Self>(&raw) {
            Ok(s) => s,
            Err(e) => {
                warn!("Failed to parse settings.json: {e}; defaulting to telemetry disabled");
                Self::default()
            }
        }
    }

    /// Persist settings to `settings_dir/settings.json`.
    pub fn save(&self, settings_dir: &Path) -> std::io::Result<()> {
        let path = settings_dir.join("settings.json");
        std::fs::create_dir_all(settings_dir)?;
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        std::fs::write(path, json)
    }
}
