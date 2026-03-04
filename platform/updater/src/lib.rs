//! Self-update module for non-store builds.
//!
//! Store builds (Microsoft Store / App Store) must set the `STORE_BUILD`
//! compile-time env to `"1"` to disable the self-updater entirely at compile
//! time.
//!
//! # Flow
//! 1. At startup, `check_for_update()` fetches the update manifest.
//! 2. If a newer version exists on the current channel, `UpdateInfo` is returned.
//! 3. The caller can present UI and then call `download_and_install()`.
//! 4. The download is verified against the SHA256 in the manifest.
//! 5. The installer is launched and the app exits.
//!
//! # Safety
//! No unsafe code in this crate.

pub mod checker;
pub mod installer;
pub mod manifest;

pub use checker::{check_for_update, UpdateInfo};
pub use installer::download_and_install;
