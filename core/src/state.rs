//! Application state, owned exclusively by the core loop thread.
//!
//! # Thread ownership
//! `AppState` lives on the core loop thread.  It must not be shared across
//! threads.  All mutations go through `Command` messages.
//!
//! # Safety
//! No unsafe code.

use pdf_engine::PdfDocument;

/// Viewport dimensions and scroll position (in logical pixels).
#[derive(Debug, Clone, Default)]
pub struct Viewport {
    pub width: u32,
    pub height: u32,
    pub scroll_y: f32,
}

/// The single source of truth for the application.
pub struct AppState {
    /// Currently open document, if any.
    pub document: Option<PdfDocument>,
    /// Current zoom level (1.0 = 100%).
    pub zoom: f32,
    /// Visible viewport.
    pub viewport: Viewport,
    /// Currently displayed page index (0-based).
    pub current_page: u32,
    /// Logical page height in points used for viewport math (A4 = 842 pt).
    pub page_height_pts: f32,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            document: None,
            zoom: 1.0,
            viewport: Viewport::default(),
            current_page: 0,
            page_height_pts: 842.0,
        }
    }
}

impl AppState {
    /// Compute which pages are visible given the current viewport and zoom.
    ///
    /// Returns a range of page indices `[first, last]` (inclusive).
    pub fn visible_pages(&self) -> (u32, u32) {
        let page_count = self
            .document
            .as_ref()
            .map(|d| d.page_count())
            .unwrap_or(0);

        if page_count == 0 {
            return (0, 0);
        }

        let page_height_px = self.page_height_pts * self.zoom;
        let first = (self.viewport.scroll_y / page_height_px).floor() as u32;
        let visible_height = self.viewport.height as f32;
        let last_f = (self.viewport.scroll_y + visible_height) / page_height_px;
        let last = (last_f.ceil() as u32).saturating_sub(1).min(page_count - 1);

        let first = first.min(page_count - 1);
        (first, last.max(first))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn state_with_pages(page_count: u32, zoom: f32) -> AppState {
        use std::path::PathBuf;
        // We cannot open a real document in unit tests; build the state manually.
        let mut s = AppState {
            zoom,
            viewport: Viewport {
                width: 800,
                height: 600,
                scroll_y: 0.0,
            },
            ..Default::default()
        };
        // Inject a stub document by exploiting the public page_height_pts field.
        s.page_height_pts = 842.0;
        s
    }

    #[test]
    fn visible_pages_no_document() {
        let s = AppState::default();
        assert_eq!(s.visible_pages(), (0, 0));
    }
}
