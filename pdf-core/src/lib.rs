pub mod document;
pub mod command;
pub mod event;
pub mod error;
pub mod ocr;
pub mod plugin;

pub use document::{Document, Page, MediaBox};
pub use command::{DocumentCommand, CommandHistory};
pub use event::{DocumentEvent, EventBus};
pub use error::PdfCoreError;
pub use ocr::{OcrProvider, OcrResult, TextRegion};
pub use plugin::{Plugin, PluginContext};
