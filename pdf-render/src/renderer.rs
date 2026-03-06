use crate::types::{RenderedPage, TextBox};
use pdf_core::{Document, PdfCoreError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RenderError {
    #[error("page {0} is out of range")]
    PageOutOfRange(u32),
    #[error("invalid zoom {0}")]
    InvalidZoom(f32),
    #[error("document error: {0}")]
    Document(#[from] PdfCoreError),
    #[error("render backend error: {0}")]
    Backend(String),
}

pub trait RenderEngine: Send {
    fn render_page(
        &self,
        doc: &Document,
        page_index: u32,
        zoom: f32,
    ) -> Result<RenderedPage, RenderError>;

    fn get_text_boxes(&self, doc: &Document, page_index: u32) -> Result<Vec<TextBox>, RenderError>;
}

// ---------------------------------------------------------------------------
// MuPdfRenderer — real rasterization via libmupdf
// ---------------------------------------------------------------------------

/// A [`RenderEngine`] backed by MuPDF.
///
/// Each call opens the PDF from disk using `mupdf::Document::open`, renders
/// the requested page to an RGBA pixel buffer, and returns it.  The renderer
/// itself is a zero-sized type and therefore both `Send` and `Sync`.
///
/// This type is only available when the `mupdf` Cargo feature is enabled.
/// Without it, use [`SoftwareRenderer`] instead.
#[cfg(feature = "mupdf")]
pub struct MuPdfRenderer;

#[cfg(feature = "mupdf")]
impl MuPdfRenderer {
    pub fn new() -> Self {
        Self
    }

    /// Render a single page directly from a file path.
    ///
    /// This free-standing method is suitable for background threads because it
    /// requires no shared state — it opens and closes its own `mupdf::Document`
    /// for each call.
    pub fn render_from_path(
        path: &std::path::Path,
        page_index: u32,
        zoom: f32,
    ) -> Result<RenderedPage, RenderError> {
        if zoom <= 0.0 || zoom > 10.0 {
            return Err(RenderError::InvalidZoom(zoom));
        }
        let path_str = path
            .to_str()
            .ok_or_else(|| RenderError::Backend("non-UTF-8 path".into()))?;

        let mdoc = mupdf::Document::open(path_str)
            .map_err(|e| RenderError::Backend(format!("mupdf open: {e}")))?;

        let page_count = mdoc
            .page_count()
            .map_err(|e| RenderError::Backend(e.to_string()))? as u32;
        if page_index >= page_count {
            return Err(RenderError::PageOutOfRange(page_index));
        }

        let page = mdoc
            .load_page(page_index as i32)
            .map_err(|e| RenderError::Backend(e.to_string()))?;

        let matrix = mupdf::Matrix::new_scale(zoom, zoom);
        let cs = mupdf::Colorspace::device_rgb();
        let pixmap = page
            .to_pixmap(&matrix, &cs, false, true)
            .map_err(|e| RenderError::Backend(e.to_string()))?;

        let width = pixmap.width();
        let height = pixmap.height();
        let stride = pixmap.stride() as usize;
        let samples = pixmap.samples();

        // Convert packed-RGB rows (stride may be wider than width * 3) to RGBA.
        let mut data = Vec::with_capacity((width * height * 4) as usize);
        for y in 0..height as usize {
            for x in 0..width as usize {
                let off = y * stride + x * 3;
                data.push(samples[off]); // R
                data.push(samples[off + 1]); // G
                data.push(samples[off + 2]); // B
                data.push(255); // A
            }
        }

        tracing::debug!(page_index, width, height, zoom, "page rasterized (mupdf)");
        Ok(RenderedPage {
            data,
            width,
            height,
            page_index,
        })
    }

    /// Render a single page directly from in-memory PDF bytes.
    pub fn render_from_bytes(
        bytes: &[u8],
        page_index: u32,
        zoom: f32,
    ) -> Result<RenderedPage, RenderError> {
        if zoom <= 0.0 || zoom > 10.0 {
            return Err(RenderError::InvalidZoom(zoom));
        }

        let mdoc = mupdf::Document::from_bytes(bytes, "pdf")
            .map_err(|e| RenderError::Backend(format!("mupdf open bytes: {e}")))?;

        let page_count = mdoc
            .page_count()
            .map_err(|e| RenderError::Backend(e.to_string()))? as u32;
        if page_index >= page_count {
            return Err(RenderError::PageOutOfRange(page_index));
        }

        let page = mdoc
            .load_page(page_index as i32)
            .map_err(|e| RenderError::Backend(e.to_string()))?;

        let matrix = mupdf::Matrix::new_scale(zoom, zoom);
        let cs = mupdf::Colorspace::device_rgb();
        let pixmap = page
            .to_pixmap(&matrix, &cs, false, true)
            .map_err(|e| RenderError::Backend(e.to_string()))?;

        let width = pixmap.width();
        let height = pixmap.height();
        let stride = pixmap.stride() as usize;
        let samples = pixmap.samples();

        let mut data = Vec::with_capacity((width * height * 4) as usize);
        for y in 0..height as usize {
            for x in 0..width as usize {
                let off = y * stride + x * 3;
                data.push(samples[off]);
                data.push(samples[off + 1]);
                data.push(samples[off + 2]);
                data.push(255);
            }
        }

        tracing::debug!(page_index, width, height, zoom, "page rasterized (mupdf bytes)");
        Ok(RenderedPage {
            data,
            width,
            height,
            page_index,
        })
    }
}

#[cfg(feature = "mupdf")]
impl Default for MuPdfRenderer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "mupdf")]
impl RenderEngine for MuPdfRenderer {
    fn render_page(
        &self,
        doc: &Document,
        page_index: u32,
        zoom: f32,
    ) -> Result<RenderedPage, RenderError> {
        Self::render_from_path(&doc.path, page_index, zoom).or_else(|mupdf_err| {
            tracing::debug!("MuPDF render failed ({mupdf_err}), falling back to software renderer");
            SoftwareRenderer.render_page(doc, page_index, zoom)
        })
    }

    fn get_text_boxes(&self, doc: &Document, page_index: u32) -> Result<Vec<TextBox>, RenderError> {
        let path_str = doc
            .path
            .to_str()
            .ok_or_else(|| RenderError::Backend("non-UTF-8 path".into()))?;

        let mdoc = mupdf::Document::open(path_str)
            .map_err(|e| RenderError::Backend(format!("mupdf open: {e}")))?;

        let page = mdoc
            .load_page(page_index as i32)
            .map_err(|e| RenderError::Backend(e.to_string()))?;

        let text_page = page
            .to_text_page(mupdf::TextPageFlags::empty())
            .map_err(|e| RenderError::Backend(e.to_string()))?;

        let mut boxes = Vec::new();
        for block in text_page.blocks() {
            let bounds = block.bounds();
            let mut text = String::new();
            let mut lines = block.lines().peekable();
            while let Some(line) = lines.next() {
                for ch in line.chars() {
                    if let Some(c) = ch.char() {
                        text.push(c);
                    }
                }
                if lines.peek().is_some() {
                    text.push('\n');
                }
            }
            if !text.is_empty() {
                boxes.push(TextBox {
                    text,
                    x: bounds.x0 as f64,
                    y: bounds.y0 as f64,
                    width: (bounds.x1 - bounds.x0) as f64,
                    height: (bounds.y1 - bounds.y0) as f64,
                });
            }
        }
        Ok(boxes)
    }
}

// ---------------------------------------------------------------------------
// SoftwareRenderer — white-rectangle stub used as a fallback
// ---------------------------------------------------------------------------

pub struct SoftwareRenderer;

impl SoftwareRenderer {
    /// Render a page given only its physical dimensions (in PDF user units) and
    /// the desired zoom factor.  This method is `Send`-safe because it does not
    /// require access to the `Document` object, making it suitable for use on
    /// background render threads.
    pub fn render_from_dims(
        page_index: u32,
        page_width: f64,
        page_height: f64,
        zoom: f32,
    ) -> Result<RenderedPage, RenderError> {
        if zoom <= 0.0 || zoom > 10.0 {
            return Err(RenderError::InvalidZoom(zoom));
        }
        let width = (page_width * zoom as f64).round() as u32;
        let height = (page_height * zoom as f64).round() as u32;
        let width = width.max(1);
        let height = height.max(1);

        let mut data = vec![255u8; (width * height * 4) as usize];

        let border_color = [180u8, 180, 180, 255];
        for x in 0..width {
            let idx = (x * 4) as usize;
            data[idx..idx + 4].copy_from_slice(&border_color);
            let idx = ((height - 1) * width * 4 + x * 4) as usize;
            data[idx..idx + 4].copy_from_slice(&border_color);
        }
        for y in 0..height {
            let idx = (y * width * 4) as usize;
            data[idx..idx + 4].copy_from_slice(&border_color);
            let idx = (y * width * 4 + (width - 1) * 4) as usize;
            data[idx..idx + 4].copy_from_slice(&border_color);
        }

        tracing::debug!(
            page_index,
            width,
            height,
            zoom,
            "page rasterized (software stub)"
        );
        Ok(RenderedPage {
            data,
            width,
            height,
            page_index,
        })
    }
}

impl RenderEngine for SoftwareRenderer {
    fn render_page(
        &self,
        doc: &Document,
        page_index: u32,
        zoom: f32,
    ) -> Result<RenderedPage, RenderError> {
        let page = doc.get_page(page_index).map_err(RenderError::Document)?;
        Self::render_from_dims(
            page_index,
            page.media_box.width,
            page.media_box.height,
            zoom,
        )
    }

    fn get_text_boxes(&self, doc: &Document, page_index: u32) -> Result<Vec<TextBox>, RenderError> {
        let text = doc
            .extract_text(page_index)
            .map_err(RenderError::Document)?;
        if text.trim().is_empty() {
            return Ok(Vec::new());
        }
        let page = doc.get_page(page_index).map_err(RenderError::Document)?;
        Ok(vec![TextBox {
            text,
            x: page.media_box.x,
            y: page.media_box.y,
            width: page.media_box.width,
            height: page.media_box.height,
        }])
    }
}
