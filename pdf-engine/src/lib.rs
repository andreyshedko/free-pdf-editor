//! PDF engine crate — wraps MuPDF (or a stub) behind a safe Rust API.
//!
//! # Thread ownership
//! `PdfDocument` is `Send` but **not** `Sync`.  It must never be used on the
//! UI thread.  All rendering calls originate from dedicated worker threads.
//!
//! # Safety
//! All unsafe code is isolated in this crate.  Raw MuPDF pointers never escape
//! this module.  RAII wrappers ensure native resources are freed on drop.

use shared::{DocumentError, RenderError};
use std::path::Path;
use tracing::{instrument, span, Level};

// ──────────────────────────────────────────────────────────────────────────────
// Public data types
// ──────────────────────────────────────────────────────────────────────────────

/// A rendered page bitmap.
#[derive(Debug, Clone)]
pub struct RenderedPage {
    /// Raw RGBA pixel data, row-major, 4 bytes per pixel.
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

/// Text extracted from a page.
#[derive(Debug, Clone)]
pub struct PageText {
    pub page: u32,
    pub content: String,
}

/// A simple text annotation.
#[derive(Debug, Clone)]
pub struct Annotation {
    pub page: u32,
    pub x: f32,
    pub y: f32,
    pub text: String,
}

// ──────────────────────────────────────────────────────────────────────────────
// PdfDocument
// ──────────────────────────────────────────────────────────────────────────────

/// An open PDF document.
///
/// Drop releases all native (MuPDF) resources immediately.
#[derive(Debug)]
pub struct PdfDocument {
    /// Opaque identifier for cache key generation.
    pub id: u64,
    /// Human-readable title (file stem).
    pub title: String,
    page_count: u32,
    annotations: Vec<Annotation>,
}

impl PdfDocument {
    /// Open the document at `path`.
    #[instrument(name = "document_open", skip_all, fields(path = %path.as_ref().display()))]
    pub fn open(path: impl AsRef<Path>) -> Result<Self, DocumentError> {
        let path = path.as_ref();
        if !path.exists() {
            return Err(DocumentError::OpenFailed(format!(
                "file not found: {}",
                path.display()
            )));
        }

        // ── Real MuPDF integration ────────────────────────────────────────────
        #[cfg(feature = "mupdf")]
        {
            // SAFETY: mupdf context is created and freed within this block.
            // All raw pointers remain local to the mupdf feature implementation.
            todo!("wire up real MuPDF bindings")
        }

        // ── Stub implementation ───────────────────────────────────────────────
        #[cfg(not(feature = "mupdf"))]
        {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
            let mut h = DefaultHasher::new();
            path.hash(&mut h);
            let id = h.finish();

            let title = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("document")
                .to_owned();

            tracing::info!(%id, %title, "document opened (stub)");

            Ok(Self {
                id,
                title,
                page_count: 10, // stub: pretend document has 10 pages
                annotations: Vec::new(),
            })
        }
    }

    /// Return the total number of pages.
    pub fn page_count(&self) -> u32 {
        self.page_count
    }

    /// Render `page` (0-based) at `zoom` (1.0 = 100%).
    ///
    /// Returns an RGBA bitmap.
    #[instrument(name = "render_page", skip(self), fields(page, zoom))]
    pub fn render_page(&self, page: u32, zoom: f32) -> Result<RenderedPage, RenderError> {
        if page >= self.page_count {
            return Err(RenderError::RenderFailed(
                page,
                format!("page {} out of range (0..{})", page, self.page_count),
            ));
        }
        if zoom <= 0.0 || zoom > 10.0 {
            return Err(RenderError::InvalidZoom(zoom));
        }

        let _span = span!(Level::DEBUG, "render_page", page, zoom).entered();

        #[cfg(feature = "mupdf")]
        {
            todo!("render page via MuPDF")
        }

        #[cfg(not(feature = "mupdf"))]
        {
            // Stub: return an A4-sized white page with a light-gray border.
            let width = (595.0 * zoom) as u32;
            let height = (842.0 * zoom) as u32;
            let mut data = vec![255u8; (width * height * 4) as usize];

            // Draw a thin gray border (top & left edges)
            for x in 0..width {
                let idx = (x * 4) as usize;
                data[idx] = 180;
                data[idx + 1] = 180;
                data[idx + 2] = 180;
                data[idx + 3] = 255;
            }
            for y in 0..height {
                let idx = (y * width * 4) as usize;
                data[idx] = 180;
                data[idx + 1] = 180;
                data[idx + 2] = 180;
                data[idx + 3] = 255;
            }

            tracing::debug!(page, width, height, "page rendered (stub)");
            Ok(RenderedPage { data, width, height })
        }
    }

    /// Extract plain text from `page` (0-based).
    pub fn extract_text(&self, page: u32) -> Result<PageText, DocumentError> {
        if page >= self.page_count {
            return Err(DocumentError::InvalidPage(page));
        }

        #[cfg(feature = "mupdf")]
        {
            todo!("extract text via MuPDF")
        }

        #[cfg(not(feature = "mupdf"))]
        {
            Ok(PageText {
                page,
                content: format!("[stub] text on page {}", page + 1),
            })
        }
    }

    /// Add a text annotation to `page`.
    pub fn add_annotation(
        &mut self,
        page: u32,
        x: f32,
        y: f32,
        text: String,
    ) -> Result<(), DocumentError> {
        if page >= self.page_count {
            return Err(DocumentError::InvalidPage(page));
        }
        self.annotations.push(Annotation { page, x, y, text });
        Ok(())
    }

    /// Save the document incrementally to `path`.
    #[instrument(name = "save_document", skip(self), fields(path = %path.as_ref().display()))]
    pub fn save_incremental(&self, path: impl AsRef<Path>) -> Result<(), DocumentError> {
        #[cfg(feature = "mupdf")]
        {
            todo!("save via MuPDF")
        }

        #[cfg(not(feature = "mupdf"))]
        {
            tracing::info!(path = %path.as_ref().display(), "document saved (stub)");
            Ok(())
        }
    }
}

impl Drop for PdfDocument {
    fn drop(&mut self) {
        tracing::debug!(id = self.id, "PdfDocument dropped — native resources freed");
        // Real MuPDF context cleanup would happen here.
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Tests
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn make_temp_pdf() -> NamedTempFile {
        let mut f = NamedTempFile::new().expect("temp file");
        f.write_all(b"%PDF-1.4 stub").expect("write");
        f
    }

    #[test]
    fn open_nonexistent_document_returns_error() {
        let result = PdfDocument::open("/nonexistent/path/doc.pdf");
        assert!(result.is_err());
        match result.unwrap_err() {
            DocumentError::OpenFailed(_) => {}
            e => panic!("unexpected error: {e}"),
        }
    }

    #[test]
    fn open_existing_file_succeeds() {
        let f = make_temp_pdf();
        let doc = PdfDocument::open(f.path()).expect("open");
        assert!(doc.page_count() > 0);
    }

    #[test]
    fn render_page_out_of_range_returns_error() {
        let f = make_temp_pdf();
        let doc = PdfDocument::open(f.path()).expect("open");
        let result = doc.render_page(doc.page_count(), 1.0);
        assert!(result.is_err());
    }

    #[test]
    fn render_page_valid() {
        let f = make_temp_pdf();
        let doc = PdfDocument::open(f.path()).expect("open");
        let page = doc.render_page(0, 1.0).expect("render");
        assert!(page.width > 0);
        assert!(page.height > 0);
        assert_eq!(page.data.len(), (page.width * page.height * 4) as usize);
    }
}
