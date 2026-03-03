//! Shared types for cross-crate communication.
//!
//! # Thread ownership
//! All types in this crate are `Send + Sync` and may be transmitted across thread
//! boundaries via channels.
//!
//! # Safety
//! This crate contains no unsafe code.

use std::path::PathBuf;

// ──────────────────────────────────────────────────────────────────────────────
// Error types
// ──────────────────────────────────────────────────────────────────────────────

/// Top-level application error.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Document error: {0}")]
    Document(#[from] DocumentError),
    #[error("Render error: {0}")]
    Render(#[from] RenderError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Errors arising from document operations.
#[derive(Debug, thiserror::Error)]
pub enum DocumentError {
    #[error("failed to open document: {0}")]
    OpenFailed(String),
    #[error("failed to save document: {0}")]
    SaveFailed(String),
    #[error("no document is currently open")]
    NoDocument,
    #[error("invalid page index: {0}")]
    InvalidPage(u32),
}

/// Errors arising from page rendering.
#[derive(Debug, thiserror::Error)]
pub enum RenderError {
    #[error("failed to render page {0}: {1}")]
    RenderFailed(u32, String),
    #[error("invalid zoom level: {0}")]
    InvalidZoom(f32),
}

// ──────────────────────────────────────────────────────────────────────────────
// Commands  (UI → Core)
// ──────────────────────────────────────────────────────────────────────────────

/// Commands sent from the UI layer to the core loop.
///
/// Commands are the *only* mechanism through which state mutations are allowed.
/// This enables undo/redo support in the future.
#[derive(Debug, Clone)]
pub enum Command {
    /// Open the document at `path`.
    OpenDocument(PathBuf),
    /// Close the currently open document.
    CloseDocument,
    /// Change the zoom level (1.0 = 100%).
    Zoom(f32),
    /// Notify the core that the visible viewport has changed.
    ViewportChanged { width: u32, height: u32, scroll_y: f32 },
    /// Ask the core to render all currently visible pages.
    RenderVisiblePages,
    /// Add a text annotation at the given position on `page`.
    AddAnnotation { page: u32, x: f32, y: f32, text: String },
    /// Save the document to `path`.
    SaveDocument(PathBuf),
    /// Navigate to the next page.
    NextPage,
    /// Navigate to the previous page.
    PrevPage,
}

// ──────────────────────────────────────────────────────────────────────────────
// Events  (Core → UI)
// ──────────────────────────────────────────────────────────────────────────────

/// Events emitted by the core loop to the UI layer.
#[derive(Debug, Clone)]
pub enum Event {
    /// A document was opened successfully.
    DocumentOpened { page_count: u32, title: String },
    /// The open document was closed.
    DocumentClosed,
    /// A page has been rendered and its bitmap is ready for display.
    PageRendered {
        page: u32,
        width: u32,
        height: u32,
        /// Raw RGBA pixel data, row-major, 4 bytes per pixel.
        data: Vec<u8>,
    },
    /// The zoom level changed.
    ZoomChanged(f32),
    /// The active page changed.
    PageChanged(u32),
    /// A recoverable error occurred.
    Error(String),
    /// A status message for the status bar.
    StatusChanged(String),
}

// ──────────────────────────────────────────────────────────────────────────────
// Cache key
// ──────────────────────────────────────────────────────────────────────────────

/// Key that uniquely identifies a cached rendered page.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PageCacheKey {
    pub document_id: u64,
    pub page_index: u32,
    /// Zoom encoded as integer hundredths: 100 → 1.0×, 150 → 1.5×.
    pub zoom_hundredths: u32,
}

impl PageCacheKey {
    pub fn new(document_id: u64, page_index: u32, zoom: f32) -> Self {
        Self {
            document_id,
            page_index,
            zoom_hundredths: (zoom * 100.0).round() as u32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_cache_key_zoom_encoding() {
        let k1 = PageCacheKey::new(1, 0, 1.0);
        let k2 = PageCacheKey::new(1, 0, 1.0);
        let k3 = PageCacheKey::new(1, 0, 1.5);
        assert_eq!(k1, k2);
        assert_ne!(k1, k3);
        assert_eq!(k1.zoom_hundredths, 100);
        assert_eq!(k3.zoom_hundredths, 150);
    }
}
