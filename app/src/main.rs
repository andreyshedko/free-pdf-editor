//! Application entry point.
//!
//! # Thread model
//!
//! ```text
//! ┌─────────────────┐  Command  ┌──────────────────┐  render task  ┌──────────────────┐
//! │   UI thread     │ ────────► │  Core loop thread │ ────────────► │  Worker threads  │
//! │  (Slint event   │           │  (AppState,        │               │  (PDF rendering) │
//! │   loop)         │ ◄──────── │   cache, router)  │               │                  │
//! └─────────────────┘   Event   └──────────────────┘               └──────────────────┘
//! ```

use core::command_loop::CoreLoop;
use shared::{Command, Event};
use std::{sync::mpsc, thread};
use tracing::info;
use ui::AppController;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ── Logging ──────────────────────────────────────────────────────────────
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("app=debug".parse().unwrap()),
        )
        .init();

    info!("Free PDF Editor starting");

    // ── Channels ─────────────────────────────────────────────────────────────
    let (cmd_tx, cmd_rx) = mpsc::channel::<Command>();
    let (evt_tx, evt_rx) = mpsc::channel::<Event>();

    // ── Core loop thread ─────────────────────────────────────────────────────
    thread::Builder::new()
        .name("core-loop".to_owned())
        .spawn(move || {
            CoreLoop::new(cmd_rx, evt_tx).run();
        })?;

    // ── UI (main thread) ─────────────────────────────────────────────────────
    let controller = AppController::new(cmd_tx)?;
    let weak = controller.as_weak();

    // Spawn an event-bridge thread: polls the event channel and posts UI
    // updates back to the Slint event loop via invoke_from_event_loop.
    thread::Builder::new()
        .name("event-bridge".to_owned())
        .spawn(move || {
            for event in evt_rx.iter() {
                let event_clone = event.clone();
                let weak = weak.clone();
                slint::invoke_from_event_loop(move || {
                    if let Some(window) = weak.upgrade() {
                        // Reconstructing AppController around the borrowed window
                        // is not possible; instead apply the event directly via
                        // the generated setters exposed by AppWindow.
                        apply_event_to_window(&window, event_clone);
                    }
                })
                .ok();
            }
        })?;

    // Block until the window is closed.
    controller.run()?;

    info!("Free PDF Editor exiting");
    Ok(())
}

/// Apply a core `Event` directly to the Slint-generated `AppWindow`.
///
/// This must only be called from the UI thread (inside invoke_from_event_loop).
fn apply_event_to_window(window: &ui::AppWindow, event: Event) {
    use slint::{Image, Rgba8Pixel, SharedPixelBuffer};

    match event {
        Event::DocumentOpened { page_count, title } => {
            window.set_page_count(page_count as i32);
            window.set_current_page(1);
            window.set_status_text(format!("\"{title}\" opened").into());
        }
        Event::DocumentClosed => {
            window.set_page_count(0);
            window.set_current_page(0);
            window.set_status_text("Document closed".into());
        }
        Event::PageRendered { width, height, data, .. } => {
            if data.len() == (width * height * 4) as usize {
                let mut buf = SharedPixelBuffer::<Rgba8Pixel>::new(width, height);
                buf.make_mut_bytes().copy_from_slice(&data);
                window.set_page_image(Image::from_rgba8(buf));
            }
        }
        Event::ZoomChanged(z) => {
            window.set_status_text(format!("Zoom {:.0}%", z * 100.0).into());
        }
        Event::PageChanged(page) => {
            window.set_current_page(page as i32 + 1);
        }
        Event::Error(msg) => {
            window.set_status_text(format!("Error: {}", msg).into());
        }
        Event::StatusChanged(msg) => {
            window.set_status_text(msg.into());
        }
    }
}
