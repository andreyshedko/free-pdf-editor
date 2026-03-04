pub mod command;
pub mod document;
pub mod error;
pub mod event;
pub mod ocr;
pub mod plugin;

pub use command::{CommandHistory, DocumentCommand};
pub use document::{Document, MediaBox, Page};
pub use error::PdfCoreError;
pub use event::{DocumentEvent, EventBus};
pub use ocr::{OcrProvider, OcrResult, TextRegion};
pub use plugin::{Plugin, PluginContext};
