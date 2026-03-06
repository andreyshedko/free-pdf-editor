mod controller;

use controller::AppController;
use pdf_core::event::DocumentEvent;
use std::{sync::mpsc, thread};
use tracing::info;

slint::include_modules!();

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("app_desktop=debug".parse().unwrap()),
        )
        .init();

    info!("Free PDF Editor starting");

    let (evt_tx, evt_rx) = mpsc::channel::<DocumentEvent>();
    let window = AppWindow::new()?;

    let mut controller = AppController::new(window.as_weak(), evt_tx);
    controller.wire_callbacks();

    let weak = window.as_weak();
    thread::Builder::new()
        .name("event-bridge".into())
        .spawn(move || {
            for event in evt_rx {
                let event = event.clone();
                let weak = weak.clone();
                slint::invoke_from_event_loop(move || {
                    if let Some(win) = weak.upgrade() {
                        apply_event(&win, event);
                    }
                })
                .ok();
            }
        })?;

    window.run()?;
    info!("Free PDF Editor exiting");
    Ok(())
}

fn apply_event(window: &AppWindow, event: DocumentEvent) {
    match event {
        DocumentEvent::DocumentOpened { title, page_count } => {
            window.set_document_title(title.into());
            window.set_page_count(page_count as i32);
            window.set_current_page(1);
            window.set_status_text("Document opened".into());
        }
        DocumentEvent::DocumentClosed => {
            window.set_document_title("".into());
            window.set_page_count(0);
            window.set_current_page(0);
            window.set_page_image(Default::default());
            window.set_thumbnail_image(Default::default());
            window.set_status_text("Document closed".into());
        }
        DocumentEvent::DocumentSaved { path } => {
            window.set_status_text(format!("Saved: {path}").into());
        }
        DocumentEvent::PageChanged { index } => {
            window.set_current_page(index as i32 + 1);
        }
        DocumentEvent::ZoomChanged { factor } => {
            window.set_zoom_level(factor);
            window.set_status_text(format!("Zoom {:.0}%", factor * 100.0).into());
        }
        DocumentEvent::PageDeleted { .. } | DocumentEvent::PageRotated { .. } => {
            window.set_status_text("Page modified".into());
        }
        DocumentEvent::Error { message } => {
            window.set_status_text(format!("Error: {message}").into());
        }
        DocumentEvent::StatusChanged { message } => {
            window.set_status_text(message.into());
        }
        _ => {}
    }
}
