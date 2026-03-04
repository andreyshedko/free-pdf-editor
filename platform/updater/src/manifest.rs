//! Update manifest types (mirrors the server-side releases.json contract).

use serde::Deserialize;

/// Top-level update server manifest fetched from the update URL.
#[derive(Debug, Deserialize)]
pub struct UpdateManifest {
    pub stable: Option<ChannelEntry>,
    pub beta: Option<ChannelEntry>,
    pub alpha: Option<ChannelEntry>,
}

/// Per-channel release entry.
#[derive(Debug, Clone, Deserialize)]
pub struct ChannelEntry {
    /// Semantic version string, e.g. "2.1.0".
    pub version: String,
    /// Direct download URL for the platform package.
    pub download_url: String,
    /// SHA-256 hex digest of the downloaded file.
    pub sha256: String,
    /// When `true` the client must install this update before continuing.
    pub mandatory_update: bool,
    /// Human-readable release notes (plain text or Markdown).
    pub release_notes: String,
}
