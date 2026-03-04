//! Update availability checker.

use super::manifest::{ChannelEntry, UpdateManifest};
use semver::Version;
use tracing::debug;

/// Update server URL configured at compile time via `UPDATE_SERVER_URL` build env.
const UPDATE_SERVER_URL: &str = env!("UPDATE_SERVER_URL");

/// Information about an available update.
#[derive(Debug, Clone)]
pub struct UpdateInfo {
    pub new_version: String,
    pub download_url: String,
    pub sha256: String,
    pub mandatory: bool,
    pub release_notes: String,
}

/// Check for an update on the given channel.
///
/// Returns `Ok(Some(info))` when a newer version is available,
/// `Ok(None)` when already up-to-date, or an `Err` on network/parse failure.
///
/// On store builds (STORE_BUILD=1) this always returns `Ok(None)`.
pub fn check_for_update(channel: &str) -> Result<Option<UpdateInfo>, Box<dyn std::error::Error>> {
    if env!("STORE_BUILD") == "1" {
        debug!("Store build detected; self-update disabled");
        return Ok(None);
    }

    if UPDATE_SERVER_URL.is_empty() {
        debug!("UPDATE_SERVER_URL not configured; skipping update check");
        return Ok(None);
    }

    let current_str = env!("APP_VERSION");
    let current = Version::parse(current_str)?;

    let manifest: UpdateManifest = ureq::get(UPDATE_SERVER_URL)
        .call()
        .map_err(|e| format!("update check failed: {e}"))?
        .into_json()
        .map_err(|e| format!("update manifest parse failed: {e}"))?;

    let entry: Option<&ChannelEntry> = match channel {
        "alpha" => manifest.alpha.as_ref(),
        "beta" => manifest.beta.as_ref(),
        _ => manifest.stable.as_ref(),
    };

    let Some(entry) = entry else {
        debug!("No entry for channel '{channel}' in update manifest");
        return Ok(None);
    };

    let available = Version::parse(&entry.version)?;
    if available > current {
        debug!("Update available: {} -> {}", current_str, entry.version);
        Ok(Some(UpdateInfo {
            new_version: entry.version.clone(),
            download_url: entry.download_url.clone(),
            sha256: entry.sha256.clone(),
            mandatory: entry.mandatory_update,
            release_notes: entry.release_notes.clone(),
        }))
    } else {
        debug!("Already up-to-date ({})", current_str);
        Ok(None)
    }
}
