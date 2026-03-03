use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CacheKey {
    pub document_id: u64,
    pub page_index: u32,
    pub zoom_hundredths: u32,
}

impl CacheKey {
    pub fn new(document_id: u64, page_index: u32, zoom: f32) -> Self {
        Self {
            document_id,
            page_index,
            zoom_hundredths: (zoom * 100.0).round() as u32,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RenderedPage {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub page_index: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextBox {
    pub text: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}
