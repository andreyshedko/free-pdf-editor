//! Offline-first commercial licensing system for PDF Editor.
//!
//! # Architecture
//!
//! ```text
//! LicenseManager
//!   ├── load()          – reads license.json from platform storage path
//!   ├── verify()        – ED25519 signature check + expiry + type validation
//!   ├── current_license() – returns immutable LicenseState
//!   ├── is_commercial_allowed() – true only for Commercial/Enterprise
//!   └── feature_enabled(Feature) – per-feature gate
//! ```
//!
//! License state is derived **only** from cryptographic validation; no boolean
//! override flags exist.

pub mod error;
pub mod feature;
pub mod manager;
pub mod state;
pub mod storage;
pub mod types;

pub use error::LicenseError;
pub use feature::Feature;
pub use manager::LicenseManager;
pub use state::LicenseState;
pub use types::{LicenseFile, LicenseType};
