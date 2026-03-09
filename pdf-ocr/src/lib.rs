//! Tesseract-backed [`OcrProvider`] for the pdf-core OCR pipeline.
//!
//! # Overview
//! [`TesseractOcrProvider`] wraps the [`tesseract`](https://docs.rs/tesseract) crate to
//! expose Tesseract OCR as a [`pdf_core::OcrProvider`].  It ingests a raw
//! **RGBA8** pixel buffer (as produced by `MuPdfRenderer`) and returns a
//! [`pdf_core::OcrResult`] whose [`pdf_core::TextRegion`] coordinates are
//! expressed in **PDF user-space units (points)**.
//!
//! # Coordinate system
//! Tesseract reports bounding boxes in image pixels with the origin at the
//! **top-left** corner, while PDF places the origin at the **bottom-left**
//! corner.  This crate converts automatically using the DPI value supplied
//! at construction time (default: 72 DPI, one pixel = one point).
//!
//! # Language data
//! Tesseract requires trained language data (`*.traineddata` files) at
//! runtime.  Use [`TesseractOcrProvider::with_datapath`] to specify a custom
//! path, or ensure the system-wide tessdata directory (usually
//! `/usr/share/tesseract-ocr/5/tessdata`) is populated.
//!
//! # Example
//! ```no_run
//! use pdf_ocr::TesseractOcrProvider;
//! use pdf_core::OcrProvider;
//!
//! let provider = TesseractOcrProvider::new("eng");
//! // `rgba8_bytes` is a raw RGBA8 pixel buffer (width × height × 4 bytes) at 72 DPI.
//! let rgba8_bytes: Vec<u8> = vec![0u8; 595 * 842 * 4];
//! let result = provider.recognize(0, &rgba8_bytes, 595, 842).unwrap();
//! println!("{}", result.full_text);
//! ```

use pdf_core::{OcrProvider, OcrResult, TextRegion};

/// Tesseract-backed [`OcrProvider`].
///
/// Construct with [`TesseractOcrProvider::new`] for the default system
/// tessdata directory, or with [`TesseractOcrProvider::with_datapath`] to
/// specify a custom path.
///
/// The `dpi` setting (default `72`) controls how pixel coordinates returned
/// by Tesseract are converted to PDF user-space points:
/// `points = pixels * 72 / dpi`.  Use [`TesseractOcrProvider::with_dpi`] to
/// override.
#[derive(Debug, Clone)]
pub struct TesseractOcrProvider {
    /// Optional override for the tessdata directory.
    datapath: Option<String>,
    /// Tesseract language code(s), e.g. `"eng"` or `"eng+deu"`.
    language: String,
    /// Source image resolution in dots per inch.  Used to convert pixel
    /// coordinates to PDF points.
    dpi: f64,
}

impl TesseractOcrProvider {
    /// Create a provider that uses the system tessdata directory and the
    /// given language (e.g. `"eng"`).
    pub fn new(language: impl Into<String>) -> Self {
        Self {
            datapath: None,
            language: language.into(),
            dpi: 72.0,
        }
    }

    /// Create a provider with a custom tessdata directory path.
    pub fn with_datapath(datapath: impl Into<String>, language: impl Into<String>) -> Self {
        Self {
            datapath: Some(datapath.into()),
            language: language.into(),
            dpi: 72.0,
        }
    }

    /// Override the assumed source image DPI (default `72`).
    ///
    /// Set this to match the actual rendering resolution so that bounding-box
    /// coordinates in the returned [`OcrResult`] are accurate in PDF
    /// user-space points.
    pub fn with_dpi(mut self, dpi: f64) -> Self {
        self.dpi = dpi;
        self
    }
}

impl OcrProvider for TesseractOcrProvider {
    fn recognize(
        &self,
        page_index: u32,
        page_image: &[u8],
        width: u32,
        height: u32,
    ) -> Result<OcrResult, Box<dyn std::error::Error + Send + Sync>> {
        #[cfg(feature = "tesseract")]
        {
        use tesseract::Tesseract;

        let datapath = self.datapath.as_deref();
        let language = self.language.as_str();

        // Build the API instance.
        let mut api = Tesseract::new(datapath, Some(language))
            .map_err(|e| format!("Tesseract init error: {e}"))?;

        // Feed the raw RGBA8 frame: 4 bytes per pixel, stride = width * 4.
        let bytes_per_pixel = 4i32;
        let bytes_per_line = width as i32 * bytes_per_pixel;
        api = api
            .set_frame(
                page_image,
                width as i32,
                height as i32,
                bytes_per_pixel,
                bytes_per_line,
            )
            .map_err(|e| format!("Tesseract set_frame error: {e}"))?;

        // Hint Tesseract about the source resolution so its size heuristics
        // work correctly when DPI != 70 (its internal default).
        api = api.set_source_resolution(self.dpi.round() as i32);

        // Run recognition and fetch TSV output which includes per-word
        // bounding boxes and confidence scores.
        let mut api = api
            .recognize()
            .map_err(|e| format!("Tesseract recognize error: {e}"))?;

        let full_text = api
            .get_text()
            .map_err(|e| format!("Tesseract get_text error: {e}"))?
            .trim()
            .to_owned();

        let tsv = api
            .get_tsv_text(0)
            .map_err(|e| format!("Tesseract get_tsv_text error: {e}"))?;

        let regions = parse_tsv_regions(&tsv, height, self.dpi);

        tracing::debug!(
            page_index,
            regions = regions.len(),
            "TesseractOcrProvider: recognized page"
        );

        Ok(OcrResult {
            page_index,
            regions,
            full_text,
        })
        }

        #[cfg(not(feature = "tesseract"))]
        {
            let _ = (page_index, page_image, width, height);
            Err(
                "pdf-ocr was built without the `tesseract` feature; enable it to run OCR"
                    .into(),
            )
        }
    }
}

/// Parse Tesseract TSV output into a list of [`TextRegion`]s.
///
/// TSV columns (tab-separated, with header row):
/// ```text
/// level  page_num  block_num  par_num  line_num  word_num
/// left  top  right  bottom  conf  text
/// ```
///
/// We only emit rows at **level 5** (word level) with confidence ≥ 0.
/// Coordinates are converted from image pixels (top-left origin) to PDF
/// user-space points (bottom-left origin).
fn parse_tsv_regions(tsv: &str, image_height_px: u32, dpi: f64) -> Vec<TextRegion> {
    // Column indices in Tesseract TSV output.
    const COL_LEVEL: usize = 0;
    const COL_LEFT: usize = 6;
    const COL_TOP: usize = 7;
    const COL_RIGHT: usize = 8;
    const COL_BOTTOM: usize = 9;
    const COL_CONF: usize = 10;
    const COL_TEXT: usize = 11;
    const TSV_MIN_COLS: usize = 12;
    const WORD_LEVEL: i32 = 5;

    let scale = 72.0 / dpi; // pixels → PDF points
    let page_height_pts = image_height_px as f64 * scale;

    let mut regions = Vec::new();

    for line in tsv.lines().skip(1) {
        // Skip the header row.
        let cols: Vec<&str> = line.split('\t').collect();
        if cols.len() < TSV_MIN_COLS {
            continue;
        }

        let level: i32 = cols[COL_LEVEL].parse().unwrap_or(0);
        if level != WORD_LEVEL {
            continue;
        }

        let left: f64 = cols[COL_LEFT].parse().unwrap_or(0.0);
        let top: f64 = cols[COL_TOP].parse().unwrap_or(0.0);
        let right: f64 = cols[COL_RIGHT].parse().unwrap_or(0.0);
        let bottom: f64 = cols[COL_BOTTOM].parse().unwrap_or(0.0);
        let conf: f64 = cols[COL_CONF].parse().unwrap_or(-1.0);
        let text = cols[COL_TEXT].trim().to_owned();

        if text.is_empty() || conf < 0.0 {
            continue;
        }

        let x = left * scale;
        let width = (right - left) * scale;
        let height_pts = (bottom - top) * scale;
        // Convert from top-left origin (Tesseract) to bottom-left (PDF).
        let y = page_height_pts - (top * scale) - height_pts;

        regions.push(TextRegion {
            text,
            x,
            y,
            width,
            height: height_pts,
            confidence: (conf / 100.0) as f32,
        });
    }

    regions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_tsv_regions_basic() {
        // Minimal synthetic TSV with one word at level 5.
        let tsv = "level\tpage_num\tblock_num\tpar_num\tline_num\tword_num\tleft\ttop\tright\tbottom\tconf\ttext\n\
                   5\t1\t1\t1\t1\t1\t10\t20\t110\t40\t95\tHello\n";

        let regions = parse_tsv_regions(tsv, 100, 72.0);
        assert_eq!(regions.len(), 1);

        let r = &regions[0];
        assert_eq!(r.text, "Hello");
        // At 72 DPI, scale = 1.0
        assert!((r.x - 10.0).abs() < 0.01, "x should be ~10");
        assert!((r.width - 100.0).abs() < 0.01, "width should be ~100");
        assert!((r.height - 20.0).abs() < 0.01, "height should be ~20");
        // y = 100 - 20 - 20 = 60
        assert!((r.y - 60.0).abs() < 0.01, "y should be ~60");
        assert!((r.confidence - 0.95).abs() < 0.01);
    }

    #[test]
    fn parse_tsv_regions_skips_non_word_levels() {
        let tsv = "level\tpage_num\tblock_num\tpar_num\tline_num\tword_num\tleft\ttop\tright\tbottom\tconf\ttext\n\
                   1\t1\t1\t0\t0\t0\t0\t0\t595\t842\t-1\t\n\
                   4\t1\t1\t1\t1\t0\t10\t20\t200\t40\t-1\t\n\
                   5\t1\t1\t1\t1\t1\t10\t20\t110\t40\t90\tWord\n";

        let regions = parse_tsv_regions(tsv, 842, 72.0);
        assert_eq!(regions.len(), 1);
        assert_eq!(regions[0].text, "Word");
    }

    #[test]
    fn parse_tsv_regions_dpi_scaling() {
        // Image rendered at 144 DPI; scale factor = 72/144 = 0.5
        let tsv = "level\tpage_num\tblock_num\tpar_num\tline_num\tword_num\tleft\ttop\tright\tbottom\tconf\ttext\n\
                   5\t1\t1\t1\t1\t1\t100\t200\t300\t250\t80\tTest\n";

        let regions = parse_tsv_regions(tsv, 1200, 144.0);
        assert_eq!(regions.len(), 1);
        let r = &regions[0];
        // scale = 0.5
        assert!((r.x - 50.0).abs() < 0.01, "x={}", r.x);
        assert!((r.width - 100.0).abs() < 0.01, "w={}", r.width);
        assert!((r.height - 25.0).abs() < 0.01, "h={}", r.height);
    }

    #[test]
    fn parse_tsv_regions_empty_input() {
        let tsv = "level\tpage_num\tblock_num\tpar_num\tline_num\tword_num\tleft\ttop\tright\tbottom\tconf\ttext\n";
        let regions = parse_tsv_regions(tsv, 842, 72.0);
        assert!(regions.is_empty());
    }
}
