use crate::AppWindow;
use pdf_render::PageCache;
use slint::Weak;
use std::sync::atomic::AtomicU64;
use std::sync::{Arc, Mutex};

/// A render task submitted to the background render worker.
pub(crate) struct RenderTask {
    pub(crate) doc_id: u64,
    /// Serialized current in-memory document state (includes unsaved edits).
    pub(crate) doc_bytes: Vec<u8>,
    #[cfg_attr(not(feature = "mupdf"), allow(dead_code))]
    pub(crate) doc_path: std::path::PathBuf,
    pub(crate) page_index: u32,
    pub(crate) page_width: f64,
    pub(crate) page_height: f64,
    pub(crate) zoom: f32,
    pub(crate) page_count: u32,
    pub(crate) cache: Arc<Mutex<PageCache>>,
    pub(crate) window: Weak<AppWindow>,
    /// Shared generation counter. The UI thread increments this whenever the
    /// desired render target changes (page nav, zoom, open/close).
    pub(crate) render_generation: Arc<AtomicU64>,
    /// The generation value at submit time. If it no longer matches when
    /// rendering completes, the frame is stale and must be discarded.
    pub(crate) expected_generation: u64,
}
