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

    fn get_text_boxes(
        &self,
        doc: &Document,
        page_index: u32,
    ) -> Result<Vec<TextBox>, RenderError>;
}

pub struct SoftwareRenderer;

impl RenderEngine for SoftwareRenderer {
    fn render_page(
        &self,
        doc: &Document,
        page_index: u32,
        zoom: f32,
    ) -> Result<RenderedPage, RenderError> {
        if zoom <= 0.0 || zoom > 10.0 {
            return Err(RenderError::InvalidZoom(zoom));
        }
        let page = doc.get_page(page_index).map_err(RenderError::Document)?;
        let width = (page.media_box.width * zoom as f64).round() as u32;
        let height = (page.media_box.height * zoom as f64).round() as u32;
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

        tracing::debug!(page_index, width, height, zoom, "page rasterized (software)");
        Ok(RenderedPage { data, width, height, page_index })
    }

    fn get_text_boxes(
        &self,
        doc: &Document,
        page_index: u32,
    ) -> Result<Vec<TextBox>, RenderError> {
        let text = doc.extract_text(page_index).map_err(RenderError::Document)?;
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
