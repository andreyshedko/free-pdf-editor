//! UI crate — Slint-based view layer.
//!
//! # Thread ownership
//! All Slint calls must originate from the UI thread.
//! `AppController` is `!Send` because it holds a `slint::Weak<AppWindow>`.
//!
//! # Safety
//! No unsafe code.

slint::include_modules!();

use dark_light::Mode;
use shared::{Command, Event};
use slint::{Image, Rgba8Pixel, SharedPixelBuffer};
use std::sync::mpsc::Sender;
use tracing::instrument;

/// Bridges Slint UI events to `Command`s and applies `Event`s to the UI.
pub struct AppController {
    window: AppWindow,
}

impl AppController {
    /// Create the main window and wire all callbacks to `cmd_tx`.
    pub fn new(cmd_tx: Sender<Command>) -> Result<Self, slint::PlatformError> {
        let window = AppWindow::new()?;

        let system_dark = matches!(dark_light::detect(), Mode::Dark);
        window.set_dark_theme(system_dark);

        let tx = cmd_tx.clone();
        window.on_open_document(move || {
            tracing::info!("open-document callback");
            // In production: wire platform::pick_open_file() here (via the app
            // crate, which may inject a callback closure that calls into the
            // platform crate).  The stub below uses a cross-platform temp path
            // so that compilation succeeds on all targets.
            let placeholder = std::env::temp_dir().join("placeholder.pdf");
            let _ = tx.send(Command::OpenDocument(placeholder));
        });

        let tx = cmd_tx.clone();
        window.on_save_document(move || {
            tracing::info!("save-document callback");
            // Production: replace with platform::pick_save_file().
            let out = std::env::temp_dir().join("output.pdf");
            let _ = tx.send(Command::SaveDocument(out));
        });

        let tx = cmd_tx.clone();
        window.on_zoom_in(move || {
            let _ = tx.send(Command::Zoom(1.25));
        });

        let tx = cmd_tx.clone();
        window.on_zoom_out(move || {
            let _ = tx.send(Command::Zoom(0.75));
        });

        let tx = cmd_tx.clone();
        window.on_next_page(move || {
            let _ = tx.send(Command::NextPage);
        });

        let tx = cmd_tx.clone();
        window.on_prev_page(move || {
            let _ = tx.send(Command::PrevPage);
        });

        Ok(Self { window })
    }

    /// Apply an `Event` from the core loop to the UI.
    ///
    /// Must be called from the UI thread.
    #[instrument(skip(self, event))]
    pub fn apply_event(&self, event: Event) {
        match event {
            Event::DocumentOpened { page_count, title } => {
                self.window.set_page_count(page_count as i32);
                self.window.set_current_page(1);
                self.window
                    .set_status_text(format!("\"{title}\" opened").into());
            }
            Event::DocumentClosed => {
                self.window.set_page_count(0);
                self.window.set_current_page(0);
                self.window.set_status_text("Document closed".into());
            }
            Event::PageRendered { width, height, data, .. } => {
                if let Some(image) = make_image(width, height, data) {
                    self.window.set_page_image(image);
                }
            }
            Event::ZoomChanged(z) => {
                self.window
                    .set_status_text(format!("Zoom {:.0}%", z * 100.0).into());
            }
            Event::PageChanged(page) => {
                self.window.set_current_page(page as i32 + 1);
            }
            Event::Error(msg) => {
                self.window.set_status_text(format!("Error: {}", msg).into());
            }
            Event::StatusChanged(msg) => {
                self.window.set_status_text(msg.into());
            }
        }
    }

    /// Return a weak reference suitable for `invoke_from_event_loop`.
    pub fn as_weak(&self) -> slint::Weak<AppWindow> {
        self.window.as_weak()
    }

    /// Block until the window is closed.
    pub fn run(self) -> Result<(), slint::PlatformError> {
        self.window.run()
    }
}

/// Convert raw RGBA pixel data into a `slint::Image`.
fn make_image(width: u32, height: u32, data: Vec<u8>) -> Option<Image> {
    if data.len() != (width * height * 4) as usize {
        return None;
    }
    let mut buf = SharedPixelBuffer::<Rgba8Pixel>::new(width, height);
    buf.make_mut_bytes().copy_from_slice(&data);
    Some(Image::from_rgba8(buf))
}
