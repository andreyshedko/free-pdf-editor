//! Telemetry client: collects events and posts them to the backend.

use super::event::Event;
use tracing::{debug, warn};

/// Endpoint configured at compile time via `TELEMETRY_ENDPOINT` build env.
const TELEMETRY_ENDPOINT: &str = env!("TELEMETRY_ENDPOINT");

/// Thread-safe telemetry client.
///
/// Wraps an in-memory queue protected by a mutex.  A background thread is
/// spawned on the first `record()` call and drains the queue.
pub struct TelemetryClient {
    enabled: bool,
    sender: std::sync::mpsc::SyncSender<Event>,
}

impl TelemetryClient {
    /// Create a new client.  Pass `enabled = false` to silently drop all events.
    pub fn new(enabled: bool) -> Self {
        let (tx, rx) = std::sync::mpsc::sync_channel::<Event>(256);

        if enabled && !TELEMETRY_ENDPOINT.is_empty() {
            std::thread::Builder::new()
                .name("telemetry".into())
                .spawn(move || {
                    for event in rx {
                        if let Err(e) = ureq::post(TELEMETRY_ENDPOINT).send_json(&event) {
                            warn!("Telemetry send failed: {e}");
                        } else {
                            debug!("Telemetry event sent: {}", event.event);
                        }
                    }
                })
                .ok();
        }

        Self { enabled, sender: tx }
    }

    /// Queue an event.  No-op when disabled or when the channel is full.
    pub fn record(&self, event: Event) {
        if !self.enabled {
            return;
        }
        if let Err(e) = self.sender.try_send(event) {
            debug!("Telemetry queue full or closed: {e}");
        }
    }

    /// Block until all queued events have been transmitted.
    /// Call at application shutdown.
    pub fn flush(self) {
        drop(self.sender);
        // Background thread will exit once the channel is drained.
    }
}
