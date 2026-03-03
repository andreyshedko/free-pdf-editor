use crate::{
    io::{read_annotations, remove_annotation, write_annotation},
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
    removed: Option<Annotation>,
}

impl RemoveAnnotationCommand {
    pub fn new(page_index: u32, annotation_id: AnnotationId) -> Self {
        Self { page_index, annotation_id, removed: None }
    }
}

impl DocumentCommand for RemoveAnnotationCommand {
    fn description(&self) -> &str { "Remove annotation" }

    fn execute(&mut self, doc: &mut Document) -> Result<(), PdfCoreError> {
        let annotations = read_annotations(doc, self.page_index);
        self.removed = annotations.into_iter().find(|a| a.id == self.annotation_id);
        remove_annotation(doc, self.page_index, &self.annotation_id)
    }

    fn undo(&mut self, doc: &mut Document) -> Result<(), PdfCoreError> {
        let mut ann = self.removed.clone().ok_or(PdfCoreError::NotUndoable)?;
        write_annotation(doc, &mut ann)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{AnnotationKind, Color, Rect};
    use lopdf::{dictionary, Document as LopdfDoc, Object, Stream};
    use pdf_core::Document;
    
    use tempfile::NamedTempFile;

    fn minimal_pdf() -> NamedTempFile {
        let mut doc = LopdfDoc::with_version("1.7");
        let pages_id = doc.new_object_id();
        let page_id = doc.new_object_id();
        let content = Stream::new(dictionary! {}, b"BT /F1 12 Tf 100 700 Td (Test) Tj ET".to_vec());
        let content_id = doc.add_object(content);
        let page = lopdf::Object::Dictionary(dictionary! {
            "Type"     => Object::Name(b"Page".to_vec()),
            "Parent"   => Object::Reference(pages_id),
            "MediaBox" => Object::Array(vec![
                Object::Integer(0), Object::Integer(0),
                Object::Integer(595), Object::Integer(842),
            ]),
            "Contents" => Object::Reference(content_id),
        });
        doc.objects.insert(page_id, page);
        let pages = lopdf::Object::Dictionary(dictionary! {
            "Type"  => Object::Name(b"Pages".to_vec()),
            "Kids"  => Object::Array(vec![Object::Reference(page_id)]),
            "Count" => Object::Integer(1),
        });
        doc.objects.insert(pages_id, pages);
        let catalog_id = doc.add_object(dictionary! {
            "Type"  => Object::Name(b"Catalog".to_vec()),
            "Pages" => Object::Reference(pages_id),
        });
        doc.trailer.set("Root", Object::Reference(catalog_id));
        let mut f = NamedTempFile::new().expect("temp file");
        doc.save_to(f.as_file_mut()).expect("save");
        f
    }

    fn open_doc(f: &NamedTempFile) -> Document {
        Document::open(f.path()).expect("open")
    }

    fn highlight(page: u32) -> Annotation {
        Annotation::new(
            page,
            Rect { x: 72.0, y: 700.0, width: 200.0, height: 20.0 },
            AnnotationKind::Highlight { color: Color::yellow() },
        )
    }

    #[test]
    fn add_annotation_execute_and_undo() {
        let f = minimal_pdf();
        let mut doc = open_doc(&f);
        let ann = highlight(0);
        let id = ann.id.clone();
        let mut cmd = AddAnnotationCommand::new(ann);
        cmd.execute(&mut doc).expect("execute");
        let anns = read_annotations(&doc, 0);
        assert!(anns.iter().any(|a| a.id == id), "annotation should be present after execute");
        cmd.undo(&mut doc).expect("undo");
        let anns_after = read_annotations(&doc, 0);
        assert!(!anns_after.iter().any(|a| a.id == id), "annotation should be gone after undo");
    }

    #[test]
    fn remove_annotation_execute_and_undo() {
        let f = minimal_pdf();
        let mut doc = open_doc(&f);
        // First add an annotation so we can remove it
        let ann = highlight(0);
        let id = ann.id.clone();
        let mut add_cmd = AddAnnotationCommand::new(ann);
        add_cmd.execute(&mut doc).expect("add");

        let mut rm_cmd = RemoveAnnotationCommand::new(0, id.clone());
        rm_cmd.execute(&mut doc).expect("remove execute");
        let anns = read_annotations(&doc, 0);
        assert!(!anns.iter().any(|a| a.id == id), "annotation should be absent after remove");

        rm_cmd.undo(&mut doc).expect("remove undo");
        let anns_after = read_annotations(&doc, 0);
        assert!(anns_after.iter().any(|a| a.id == id), "annotation should be restored after undo");
    }

    #[test]
    fn remove_nonexistent_annotation_returns_error() {
        let f = minimal_pdf();
        let mut doc = open_doc(&f);
        let fake_id = AnnotationId("nonexistent".to_string());
        let mut cmd = RemoveAnnotationCommand::new(0, fake_id);
        assert!(cmd.execute(&mut doc).is_err());
    }
}
