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

    pub fn can_undo(&self) -> bool { !self.undo_stack.is_empty() }
    pub fn can_redo(&self) -> bool { !self.redo_stack.is_empty() }

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
