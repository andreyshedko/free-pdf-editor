//! Privacy-safe, opt-in telemetry service.
//!
//! # Design
//! - Strictly opt-in: all events are silently dropped when telemetry is disabled.
//! - No personal data is ever captured (no user ID, no file paths, no content).
//! - Events are queued in-memory and flushed in a background thread.
//! - The backend endpoint is configured at compile time via `TELEMETRY_ENDPOINT`.
//!
//! # Usage
//! ```rust
//! use telemetry::{TelemetryClient, Event};
//!
//! let client = TelemetryClient::new(true /* enabled */);
//! client.record(Event::startup_success());
//! // …later at shutdown:
//! client.flush();
//! ```
//!
//! # Safety
//! No unsafe code in this crate.

pub mod client;
pub mod event;
pub mod settings;

pub use client::TelemetryClient;
pub use event::Event;
pub use settings::TelemetrySettings;
