use crate::{
    io::{remove_annotation, write_annotation},
    types::{Annotation, AnnotationId},
};
use pdf_core::{Document, DocumentCommand, PdfCoreError};

#[derive(Debug)]
pub struct AddAnnotationCommand {
    annotation: Annotation,
}

impl AddAnnotationCommand {
    pub fn new(annotation: Annotation) -> Self {
        Self { annotation }
    }
}

impl DocumentCommand for AddAnnotationCommand {
    fn description(&self) -> &str { "Add annotation" }

    fn execute(&mut self, doc: &mut Document) -> Result<(), PdfCoreError> {
        write_annotation(doc, &mut self.annotation)?;
        Ok(())
    }

    fn undo(&mut self, doc: &mut Document) -> Result<(), PdfCoreError> {
        remove_annotation(doc, self.annotation.page_index, &self.annotation.id)
    }
}

#[derive(Debug)]
pub struct RemoveAnnotationCommand {
    page_index: u32,
    annotation_id: AnnotationId,
}

impl RemoveAnnotationCommand {
    pub fn new(page_index: u32, annotation_id: AnnotationId) -> Self {
        Self { page_index, annotation_id }
    }
}

impl DocumentCommand for RemoveAnnotationCommand {
    fn description(&self) -> &str { "Remove annotation" }

    fn execute(&mut self, doc: &mut Document) -> Result<(), PdfCoreError> {
        remove_annotation(doc, self.page_index, &self.annotation_id)
    }

    fn undo(&mut self, _doc: &mut Document) -> Result<(), PdfCoreError> {
        Err(PdfCoreError::NotUndoable)
    }
}
