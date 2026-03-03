#[derive(Debug, Clone)]
pub struct TextRegion {
    pub text: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub confidence: f32,
}

#[derive(Debug, Clone)]
pub struct OcrResult {
    pub page_index: u32,
    pub regions: Vec<TextRegion>,
    pub full_text: String,
}

pub trait OcrProvider: Send + Sync {
    fn recognize(
        &self,
        page_index: u32,
        page_image: &[u8],
        width: u32,
        height: u32,
    ) -> Result<OcrResult, Box<dyn std::error::Error + Send + Sync>>;
}
