use thiserror::Error;

#[derive(Debug, Error)]
pub enum PdfCoreError {
    #[error("failed to open document: {0}")]
    Open(String),
    #[error("failed to save document: {0}")]
    Save(String),
    #[error("no document is currently open")]
    NoDocument,
    #[error("page index {0} is out of range")]
    PageOutOfRange(u32),
    #[error("annotation not found: {0}")]
    AnnotationNotFound(String),
    #[error("form field not found: {0}")]
    FieldNotFound(String),
    #[error("command cannot be undone")]
    NotUndoable,
    #[error("undo stack is empty")]
    NothingToUndo,
    #[error("redo stack is empty")]
    NothingToRedo,
    #[error("invalid argument: {0}")]
    InvalidArgument(String),
    #[error("lopdf error: {0}")]
    LopdfError(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl From<lopdf::Error> for PdfCoreError {
    fn from(e: lopdf::Error) -> Self {
        PdfCoreError::LopdfError(e.to_string())
    }
}
