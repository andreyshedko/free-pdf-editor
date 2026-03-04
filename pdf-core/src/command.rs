use crate::document::Document;
use crate::error::PdfCoreError;

pub trait DocumentCommand: std::fmt::Debug + Send {
    fn description(&self) -> &str;
    fn execute(&mut self, doc: &mut Document) -> Result<(), PdfCoreError>;
    fn undo(&mut self, doc: &mut Document) -> Result<(), PdfCoreError>;
}

pub struct CommandHistory {
    undo_stack: Vec<Box<dyn DocumentCommand>>,
    redo_stack: Vec<Box<dyn DocumentCommand>>,
    max_depth: usize,
}

impl std::fmt::Debug for CommandHistory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CommandHistory")
            .field("undo_depth", &self.undo_stack.len())
            .field("redo_depth", &self.redo_stack.len())
            .finish()
    }
}

impl CommandHistory {
    pub fn new(max_depth: usize) -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_depth,
        }
    }

    pub fn execute(
        &mut self,
        mut cmd: Box<dyn DocumentCommand>,
        doc: &mut Document,
    ) -> Result<(), PdfCoreError> {
        cmd.execute(doc)?;
        self.redo_stack.clear();
        self.undo_stack.push(cmd);
        if self.undo_stack.len() > self.max_depth {
            self.undo_stack.remove(0);
        }
        Ok(())
    }

    pub fn undo(&mut self, doc: &mut Document) -> Result<(), PdfCoreError> {
        let mut cmd = self.undo_stack.pop().ok_or(PdfCoreError::NothingToUndo)?;
        cmd.undo(doc)?;
        self.redo_stack.push(cmd);
        Ok(())
    }

    pub fn redo(&mut self, doc: &mut Document) -> Result<(), PdfCoreError> {
        let mut cmd = self.redo_stack.pop().ok_or(PdfCoreError::NothingToRedo)?;
        cmd.execute(doc)?;
        self.undo_stack.push(cmd);
        Ok(())
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    pub fn undo_description(&self) -> Option<&str> {
        self.undo_stack.last().map(|c| c.description())
    }

    pub fn redo_description(&self) -> Option<&str> {
        self.redo_stack.last().map(|c| c.description())
    }

    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::PdfCoreError;

    /// A command that appends a marker to a vec (stored in doc title for testing).
    #[derive(Debug)]
    struct AppendTitleCommand {
        suffix: String,
        prev: String,
    }

    impl AppendTitleCommand {
        fn new(suffix: impl Into<String>) -> Self {
            Self {
                suffix: suffix.into(),
                prev: String::new(),
            }
        }
    }

    impl DocumentCommand for AppendTitleCommand {
        fn description(&self) -> &str {
            "Append title"
        }
        fn execute(&mut self, doc: &mut Document) -> Result<(), PdfCoreError> {
            self.prev = doc.title.clone();
            doc.title = format!("{}{}", doc.title, self.suffix);
            Ok(())
        }
        fn undo(&mut self, doc: &mut Document) -> Result<(), PdfCoreError> {
            doc.title = self.prev.clone();
            Ok(())
        }
    }

    fn empty_doc() -> Document {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        Document::create_new(tmp.path()).unwrap()
    }

    #[test]
    fn execute_and_undo() {
        let mut doc = empty_doc();
        doc.title = "base".into();
        let mut history = CommandHistory::new(10);

        history
            .execute(Box::new(AppendTitleCommand::new("_A")), &mut doc)
            .unwrap();
        assert_eq!(doc.title, "base_A");
        assert!(history.can_undo());
        assert!(!history.can_redo());

        history.undo(&mut doc).unwrap();
        assert_eq!(doc.title, "base");
        assert!(!history.can_undo());
        assert!(history.can_redo());
    }

    #[test]
    fn execute_clears_redo_stack() {
        let mut doc = empty_doc();
        doc.title = "base".into();
        let mut history = CommandHistory::new(10);

        history
            .execute(Box::new(AppendTitleCommand::new("_A")), &mut doc)
            .unwrap();
        history.undo(&mut doc).unwrap();
        assert!(history.can_redo());

        // New execute clears redo
        history
            .execute(Box::new(AppendTitleCommand::new("_B")), &mut doc)
            .unwrap();
        assert!(!history.can_redo());
        assert_eq!(doc.title, "base_B");
    }

    #[test]
    fn undo_on_empty_returns_error() {
        let mut doc = empty_doc();
        let mut history = CommandHistory::new(10);
        assert!(matches!(
            history.undo(&mut doc),
            Err(PdfCoreError::NothingToUndo)
        ));
    }

    #[test]
    fn redo_on_empty_returns_error() {
        let mut doc = empty_doc();
        let mut history = CommandHistory::new(10);
        assert!(matches!(
            history.redo(&mut doc),
            Err(PdfCoreError::NothingToRedo)
        ));
    }

    #[test]
    fn max_depth_respected() {
        let mut doc = empty_doc();
        doc.title = String::new();
        let mut history = CommandHistory::new(3);

        for i in 0..5u32 {
            history
                .execute(Box::new(AppendTitleCommand::new(format!("{i}"))), &mut doc)
                .unwrap();
        }
        assert_eq!(history.undo_stack.len(), 3);
    }
}
