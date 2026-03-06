use crate::{
    io::{
        find_annotation_object_id, remove_annotation, remove_annotation_by_object_id,
        write_annotation,
    },
    types::{Annotation, AnnotationId},
};
use lopdf::{Object, ObjectId};
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
    fn description(&self) -> &str {
        "Add annotation"
    }

    fn execute(&mut self, doc: &mut Document) -> Result<(), PdfCoreError> {
        write_annotation(doc, &mut self.annotation)?;
        Ok(())
    }

    fn undo(&mut self, doc: &mut Document) -> Result<(), PdfCoreError> {
        if let Some(obj_id) = self.annotation.object_id {
            remove_annotation_by_object_id(doc, self.annotation.page_index, obj_id)
        } else {
            remove_annotation(doc, self.annotation.page_index, &self.annotation.id)
        }
    }
}

/// Removes an annotation from a page and supports undo by re-attaching the
/// original PDF object reference (no data loss, no PDF bloat).
///
/// `remove_annotation` only drops the `ObjectId` reference from the page's
/// `Annots` array; it never deletes the underlying annotation dictionary from
/// the document's object store.  We save that `ObjectId` in `execute()` so
/// that `undo()` can simply push the reference back into `Annots` instead of
/// recreating the object from a lossy in-memory snapshot.
#[derive(Debug)]
pub struct RemoveAnnotationCommand {
    page_index: u32,
    annotation_id: AnnotationId,
    /// The lopdf `ObjectId` of the removed annotation, set by `execute()` and
    /// consumed (via `take`) by `undo()` to prevent duplicate re-insertions.
    removed_object_id: Option<ObjectId>,
}

impl RemoveAnnotationCommand {
    pub fn new(page_index: u32, annotation_id: AnnotationId) -> Self {
        Self {
            page_index,
            annotation_id,
            removed_object_id: None,
        }
    }
}

impl DocumentCommand for RemoveAnnotationCommand {
    fn description(&self) -> &str {
        "Remove annotation"
    }

    fn execute(&mut self, doc: &mut Document) -> Result<(), PdfCoreError> {
        // Record the annotation's ObjectId *before* removing it so undo can
        // re-attach the same object without recreating or cloning it.
        self.removed_object_id = Some(find_annotation_object_id(
            doc,
            self.page_index,
            &self.annotation_id,
        )?);
        remove_annotation(doc, self.page_index, &self.annotation_id)
    }

    fn undo(&mut self, doc: &mut Document) -> Result<(), PdfCoreError> {
        // `take()` clears removed_object_id so repeated undo calls cannot
        // insert duplicate references.
        let obj_id = self
            .removed_object_id
            .take()
            .ok_or(PdfCoreError::NotUndoable)?;

        let page = doc.get_page(self.page_index)?;
        let page_id = page.object_id;
        let inner = doc.inner_mut();

        let page_dict = inner
            .get_object_mut(page_id)
            .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?
            .as_dict_mut()
            .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?;

        match page_dict.get(b"Annots") {
            Ok(Object::Array(existing)) => {
                let mut arr = existing.clone();
                arr.push(Object::Reference(obj_id));
                page_dict.set("Annots", Object::Array(arr));
            }
            _ => {
                page_dict.set("Annots", Object::Array(vec![Object::Reference(obj_id)]));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::read_annotations;
    use crate::types::{AnnotationKind, Color, Rect};
    use lopdf::{dictionary, Document as LopdfDoc, Object, Stream};
    use pdf_core::Document;

    use tempfile::NamedTempFile;

    fn minimal_pdf() -> NamedTempFile {
        let mut doc = LopdfDoc::with_version("1.7");
        let pages_id = doc.new_object_id();
        let page_id = doc.new_object_id();
        let content = Stream::new(
            dictionary! {},
            b"BT /F1 12 Tf 100 700 Td (Test) Tj ET".to_vec(),
        );
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
            Rect {
                x: 72.0,
                y: 700.0,
                width: 200.0,
                height: 20.0,
            },
            AnnotationKind::Highlight {
                color: Color::yellow(),
            },
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
        assert!(
            anns.iter().any(|a| a.id == id),
            "annotation should be present after execute"
        );
        cmd.undo(&mut doc).expect("undo");
        let anns_after = read_annotations(&doc, 0);
        assert!(
            !anns_after.iter().any(|a| a.id == id),
            "annotation should be gone after undo"
        );
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
        assert!(
            !anns.iter().any(|a| a.id == id),
            "annotation should be absent after remove"
        );

        rm_cmd.undo(&mut doc).expect("remove undo");
        let anns_after = read_annotations(&doc, 0);
        assert!(
            anns_after.iter().any(|a| a.id == id),
            "annotation should be restored after undo"
        );
    }

    #[test]
    fn remove_nonexistent_annotation_returns_error() {
        let f = minimal_pdf();
        let mut doc = open_doc(&f);
        let fake_id = AnnotationId("nonexistent".to_string());
        let mut cmd = RemoveAnnotationCommand::new(0, fake_id);
        assert!(cmd.execute(&mut doc).is_err());
    }

    #[test]
    fn remove_annotation_undo_idempotent_on_repeated_call() {
        // A second undo() call should fail (NotUndoable) rather than inserting a
        // duplicate reference, because `take()` clears `removed_object_id`.
        let f = minimal_pdf();
        let mut doc = open_doc(&f);
        let ann = highlight(0);
        let id = ann.id.clone();
        AddAnnotationCommand::new(ann)
            .execute(&mut doc)
            .expect("add");

        let mut rm_cmd = RemoveAnnotationCommand::new(0, id.clone());
        rm_cmd.execute(&mut doc).expect("remove");
        rm_cmd.undo(&mut doc).expect("first undo");

        // Second undo must fail — state was consumed by the first call.
        assert!(
            rm_cmd.undo(&mut doc).is_err(),
            "repeated undo should return an error, not insert a duplicate"
        );

        // Exactly one reference to the annotation should exist in Annots.
        let anns = read_annotations(&doc, 0);
        assert_eq!(
            anns.iter().filter(|a| a.id == id).count(),
            1,
            "annotation should appear exactly once after a single undo"
        );
    }
}
