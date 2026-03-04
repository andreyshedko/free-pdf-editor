//! Crash reporting module.
//!
//! Installs a global panic hook that captures:
//! - panic message and location
//! - thread name
//! - system info (OS version, memory usage)
//! - app version / channel / build number from compile-time env
//!
//! Reports are written as JSON files to `~/.freepdfeditor/crashes/`.
//! An uploader runs on the next application launch and ships pending
//! reports to the configured backend over HTTPS, then deletes them.
//!
//! All identifiers are anonymous (random UUID per crash, no device ID).
//!
//! # Safety
//! This crate contains no unsafe code.

pub mod hook;
pub mod report;
pub mod uploader;

pub use hook::install_panic_hook;
pub use report::CrashReport;
pub use uploader::upload_pending_crashes;
