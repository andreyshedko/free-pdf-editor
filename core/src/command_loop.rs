//! Core command loop.
//!
//! # Thread ownership
//! `CoreLoop::run` blocks the calling thread (the "core loop thread").
//! The caller must spawn a dedicated thread for it.
//!
//! # Safety
//! No unsafe code.

use crate::{
    cache::PageCache,
    state::AppState,
};
use pdf_engine::PdfDocument;
use shared::{Command, Event, PageCacheKey};
use std::{
    path::PathBuf,
    sync::mpsc::{Receiver, Sender},
};
use tracing::{span, Level};

/// Maximum number of rendered pages kept in the LRU cache.
const CACHE_CAPACITY: usize = 50;

/// The core loop processes `Command`s and emits `Event`s.
pub struct CoreLoop {
    cmd_rx: Receiver<Command>,
    evt_tx: Sender<Event>,
}

impl CoreLoop {
    pub fn new(cmd_rx: Receiver<Command>, evt_tx: Sender<Event>) -> Self {
        Self { cmd_rx, evt_tx }
    }

    fn send(&self, evt: Event) {
        // Ignore send errors: the UI may have shut down.
        let _ = self.evt_tx.send(evt);
    }

    /// Block the calling thread, processing commands until the channel is
    /// closed (UI shutdown).
    pub fn run(self) {
        let mut state = AppState::default();
        let mut cache = PageCache::new(CACHE_CAPACITY);
        let evt_tx = self.evt_tx.clone();

        for cmd in self.cmd_rx.iter() {
            let _span = span!(Level::DEBUG, "process_command").entered();
            tracing::debug!(?cmd, "processing command");

            match cmd {
                Command::OpenDocument(path) => {
                    self.open_document(&mut state, &mut cache, path)
                }
                Command::CloseDocument => {
                    self.close_document(&mut state, &mut cache)
                }
                Command::Zoom(factor) => {
                    self.zoom(&mut state, &mut cache, factor)
                }
                Command::ViewportChanged { width, height, scroll_y } => {
                    state.viewport.width = width;
                    state.viewport.height = height;
                    state.viewport.scroll_y = scroll_y;
                }
                Command::RenderVisiblePages => {
                    self.render_visible_pages(&mut state, &mut cache, &evt_tx)
                }
                Command::AddAnnotation { page, x, y, text } => {
                    self.add_annotation(&mut state, page, x, y, text)
                }
                Command::SaveDocument(path) => {
                    self.save_document(&state, path)
                }
                Command::NextPage => {
                    if let Some(ref doc) = state.document {
                        let next = (state.current_page + 1).min(doc.page_count().saturating_sub(1));
                        state.current_page = next;
                        self.send(Event::PageChanged(next));
                        self.send(Event::StatusChanged(format!("Page {}/{}", next + 1, doc.page_count())));
                    }
                }
                Command::PrevPage => {
                    if state.document.is_some() && state.current_page > 0 {
                        state.current_page -= 1;
                        let page = state.current_page;
                        let count = state.document.as_ref().unwrap().page_count();
                        self.send(Event::PageChanged(page));
                        self.send(Event::StatusChanged(format!("Page {}/{}", page + 1, count)));
                    }
                }
            }
        }

        tracing::info!("core loop exiting");
    }

    fn open_document(&self, state: &mut AppState, cache: &mut PageCache, path: PathBuf) {
        let _span = span!(Level::INFO, "document_open", path = %path.display()).entered();
        match PdfDocument::open(&path) {
            Ok(doc) => {
                let id = doc.id;
                let title = doc.title.clone();
                let page_count = doc.page_count();
                state.document = Some(doc);
                state.current_page = 0;
                cache.evict_document(id);
                self.send(Event::DocumentOpened { page_count, title: title.clone() });
                self.send(Event::StatusChanged(format!("Opened \"{title}\" ({page_count} pages)")));
            }
            Err(e) => {
                self.send(Event::Error(e.to_string()));
            }
        }
    }

    fn close_document(&self, state: &mut AppState, cache: &mut PageCache) {
        if let Some(doc) = state.document.take() {
            cache.evict_document(doc.id);
        }
        state.current_page = 0;
        self.send(Event::DocumentClosed);
        self.send(Event::StatusChanged("Document closed".to_owned()));
    }

    fn zoom(&self, state: &mut AppState, cache: &mut PageCache, factor: f32) {
        if factor <= 0.0 || factor > 10.0 {
            self.send(Event::Error(format!("Invalid zoom factor: {}", factor)));
            return;
        }
        if let Some(ref doc) = state.document {
            cache.evict_document(doc.id);
        }
        state.zoom = factor;
        self.send(Event::ZoomChanged(factor));
        self.send(Event::StatusChanged(format!("Zoom: {:.0}%", factor * 100.0)));
    }

    fn render_visible_pages(
        &self,
        state: &mut AppState,
        cache: &mut PageCache,
        evt_tx: &Sender<Event>,
    ) {
        let doc_id = match state.document.as_ref() {
            Some(d) => d.id,
            None => return,
        };

        let (first, last) = state.visible_pages();
        let zoom = state.zoom;

        // Buffer one extra page on each side.
        let page_count = state.document.as_ref().unwrap().page_count();
        let first = first.saturating_sub(1);
        let last = (last + 1).min(page_count.saturating_sub(1));

        for page_idx in first..=last {
            let key = PageCacheKey::new(doc_id, page_idx, zoom);

            if cache.get(&key).is_some() {
                tracing::debug!(page = page_idx, "cache hit");
                continue; // Already cached — no re-render needed.
            }

            // Schedule rendering on a worker thread.
            let doc = state.document.as_ref().unwrap();
            match doc.render_page(page_idx, zoom) {
                Ok(rendered) => {
                    cache.insert(key, rendered.data.clone(), rendered.width, rendered.height);
                    let _ = evt_tx.send(Event::PageRendered {
                        page: page_idx,
                        width: rendered.width,
                        height: rendered.height,
                        data: rendered.data,
                    });
                }
                Err(e) => {
                    let _ = evt_tx.send(Event::Error(e.to_string()));
                }
            }
        }
    }

    fn add_annotation(&self, state: &mut AppState, page: u32, x: f32, y: f32, text: String) {
        if let Some(ref mut doc) = state.document {
            if let Err(e) = doc.add_annotation(page, x, y, text) {
                self.send(Event::Error(e.to_string()));
            }
        } else {
            self.send(Event::Error("No document open".to_owned()));
        }
    }

    fn save_document(&self, state: &AppState, path: PathBuf) {
        let _span = span!(Level::INFO, "save_document", path = %path.display()).entered();
        match state.document.as_ref() {
            Some(doc) => match doc.save_incremental(&path) {
                Ok(()) => self.send(Event::StatusChanged(format!("Saved to {}", path.display()))),
                Err(e) => self.send(Event::Error(e.to_string())),
            },
            None => self.send(Event::Error("No document open".to_owned())),
        }
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Tests
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::thread;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn make_temp_pdf() -> NamedTempFile {
        let mut f = NamedTempFile::new().expect("temp file");
        f.write_all(b"%PDF-1.4 stub").expect("write");
        f
    }

    fn make_loop() -> (Sender<Command>, Receiver<Event>, thread::JoinHandle<()>) {
        let (cmd_tx, cmd_rx) = mpsc::channel::<Command>();
        let (evt_tx, evt_rx) = mpsc::channel::<Event>();
        let handle = thread::spawn(move || {
            CoreLoop::new(cmd_rx, evt_tx).run();
        });
        (cmd_tx, evt_rx, handle)
    }

    fn drain_events(rx: &Receiver<Event>) -> Vec<Event> {
        let mut events = Vec::new();
        while let Ok(e) = rx.try_recv() {
            events.push(e);
        }
        events
    }

    #[test]
    fn open_nonexistent_document_emits_error() {
        let (cmd_tx, evt_rx, handle) = make_loop();
        cmd_tx
            .send(Command::OpenDocument("/nonexistent/doc.pdf".into()))
            .unwrap();
        drop(cmd_tx);
        handle.join().unwrap();

        let events = drain_events(&evt_rx);
        let has_error = events.iter().any(|e| matches!(e, Event::Error(_)));
        assert!(has_error, "expected an Error event");
    }

    #[test]
    fn open_valid_document_emits_document_opened() {
        let f = make_temp_pdf();
        let (cmd_tx, evt_rx, handle) = make_loop();
        cmd_tx
            .send(Command::OpenDocument(f.path().to_path_buf()))
            .unwrap();
        drop(cmd_tx);
        handle.join().unwrap();

        let events = drain_events(&evt_rx);
        let opened = events
            .iter()
            .any(|e| matches!(e, Event::DocumentOpened { .. }));
        assert!(opened, "expected DocumentOpened event");
    }

    #[test]
    fn zoom_command_emits_zoom_changed() {
        let f = make_temp_pdf();
        let (cmd_tx, evt_rx, handle) = make_loop();
        cmd_tx
            .send(Command::OpenDocument(f.path().to_path_buf()))
            .unwrap();
        cmd_tx.send(Command::Zoom(1.5)).unwrap();
        drop(cmd_tx);
        handle.join().unwrap();

        let events = drain_events(&evt_rx);
        let zoomed = events
            .iter()
            .any(|e| matches!(e, Event::ZoomChanged(z) if (*z - 1.5).abs() < 0.01));
        assert!(zoomed, "expected ZoomChanged(1.5) event");
    }

    #[test]
    fn render_visible_pages_populates_event() {
        let f = make_temp_pdf();
        let (cmd_tx, evt_rx, handle) = make_loop();
        cmd_tx
            .send(Command::OpenDocument(f.path().to_path_buf()))
            .unwrap();
        cmd_tx
            .send(Command::ViewportChanged { width: 800, height: 600, scroll_y: 0.0 })
            .unwrap();
        cmd_tx.send(Command::RenderVisiblePages).unwrap();
        drop(cmd_tx);
        handle.join().unwrap();

        let events = drain_events(&evt_rx);
        let rendered = events
            .iter()
            .any(|e| matches!(e, Event::PageRendered { .. }));
        assert!(rendered, "expected at least one PageRendered event");
    }
}
