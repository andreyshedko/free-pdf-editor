mod controller;
mod i18n;

use controller::AppController;
use dark_light::Mode;
use i18n::Localization;
use pdf_core::event::DocumentEvent;
use std::{sync::mpsc, thread, time::Duration};
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
    let localization = Localization::detect_system();
    localization.apply_to_window(&window);
    let system_dark = matches!(dark_light::detect(), Mode::Dark | Mode::Default);
    window.set_dark_theme(system_dark);

    let mut controller = AppController::new(window.as_weak(), evt_tx);
    controller.wire_callbacks();

    let theme_weak = window.as_weak();
    thread::Builder::new()
        .name("theme-watch".into())
        .spawn(move || {
            let mut last_dark = system_dark;
            loop {
                thread::sleep(Duration::from_millis(1200));
                let current_dark = matches!(dark_light::detect(), Mode::Dark | Mode::Default);
                if current_dark == last_dark {
                    continue;
                }
                last_dark = current_dark;

                let weak = theme_weak.clone();
                if slint::invoke_from_event_loop(move || {
                    if let Some(win) = weak.upgrade() {
                        win.set_dark_theme(current_dark);
                    }
                })
                .is_err()
                {
                    break;
                }
            }
        })?;

    let weak = window.as_weak();
    thread::Builder::new()
        .name("event-bridge".into())
        .spawn(move || {
            let localization = localization;
            for event in evt_rx {
                let event = event.clone();
                let weak = weak.clone();
                slint::invoke_from_event_loop(move || {
                    if let Some(win) = weak.upgrade() {
                        apply_event(&win, event, localization);
                    }
                })
                .ok();
            }
        })?;

    window.run()?;
    info!("Free PDF Editor exiting");
    Ok(())
}

fn apply_event(window: &AppWindow, event: DocumentEvent, localization: Localization) {
    let tr = localization.ui;
    match event {
        DocumentEvent::DocumentOpened { title, page_count } => {
            window.set_document_title(title.into());
            window.set_page_count(page_count as i32);
            window.set_current_page(1);
            window.set_status_text(tr.status_document_opened.into());
        }
        DocumentEvent::DocumentClosed => {
            window.set_document_title("".into());
            window.set_page_count(0);
            window.set_current_page(0);
            window.set_page_image(Default::default());
            window.set_thumbnail_image(Default::default());
            window.set_status_text(tr.status_document_closed.into());
        }
        DocumentEvent::DocumentSaved { path } => {
            window.set_status_text(format!("{}: {path}", tr.status_saved_prefix).into());
        }
        DocumentEvent::PageChanged { index } => {
            window.set_current_page(index as i32 + 1);
        }
        DocumentEvent::ZoomChanged { factor } => {
            window.set_zoom_level(factor);
            window.set_status_text(format!("{} {:.0}%", tr.zoom, factor * 100.0).into());
        }
        DocumentEvent::PageDeleted { .. } | DocumentEvent::PageRotated { .. } => {
            window.set_status_text(tr.status_page_modified.into());
        }
        DocumentEvent::Error { message } => {
            window.set_status_text(format!("{}: {message}", tr.status_error_prefix).into());
        }
        DocumentEvent::StatusChanged { message } => {
            let localized = localization.localize_status_message(&message);
            window.set_status_text(localized.into());
        }
        _ => {}
    }
}
