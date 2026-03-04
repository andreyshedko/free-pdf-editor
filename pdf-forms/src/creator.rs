use crate::types::FormFieldKind;
use lopdf::{dictionary, Object, ObjectId};
use pdf_core::{Document, DocumentCommand, PdfCoreError};

/// Creates a new AcroForm field on a page and registers it in the document's
/// `/AcroForm` dictionary.  If no `/AcroForm` exists in the document catalog,
/// one is created automatically.
///
/// The field is represented as a combined Widget + field dictionary (the
/// single-entry-point style described in the PDF specification §12.7.4.2),
/// placed at `rect = [x1, y1, x2, y2]` in PDF user units.
#[derive(Debug)]
pub struct CreateFieldCommand {
    field_name: String,
    kind: FormFieldKind,
    page_index: u32,
    rect: [f32; 4],
    snapshot: Option<Vec<u8>>,
}

impl CreateFieldCommand {
    pub fn new(
        field_name: impl Into<String>,
        kind: FormFieldKind,
        page_index: u32,
        rect: [f32; 4],
    ) -> Self {
        Self {
            field_name: field_name.into(),
            kind,
            page_index,
            rect,
            snapshot: None,
        }
    }
}

/// Map a `FormFieldKind` to the PDF `/FT` name bytes.
/// Returns an error for `Unknown` because creating a field of an unspecified
/// type would silently produce a malformed AcroForm entry.
fn ft_bytes(kind: &FormFieldKind) -> Result<Vec<u8>, PdfCoreError> {
    match kind {
        FormFieldKind::TextField => Ok(b"Tx".to_vec()),
        FormFieldKind::Checkbox | FormFieldKind::Radio => Ok(b"Btn".to_vec()),
        FormFieldKind::Dropdown => Ok(b"Ch".to_vec()),
        FormFieldKind::SignatureField => Ok(b"Sig".to_vec()),
        FormFieldKind::Unknown => Err(PdfCoreError::InvalidArgument(
            "cannot create a form field of kind Unknown; specify a concrete field type".into(),
        )),
    }
}

impl DocumentCommand for CreateFieldCommand {
    fn description(&self) -> &str {
        "Create form field"
    }

    fn execute(&mut self, doc: &mut Document) -> Result<(), PdfCoreError> {
        // Snapshot for undo.
        let mut buf = std::io::Cursor::new(Vec::new());
        doc.inner_mut()
            .save_to(&mut buf)
            .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?;
        self.snapshot = Some(buf.into_inner());

        let page_id = doc.get_page(self.page_index)?.object_id;

        let [x1, y1, x2, y2] = self.rect;

        // Validate and resolve the field type before building the dictionary.
        let ft = ft_bytes(&self.kind)?;

        // Build the combined Widget + field dictionary.
        let mut field_dict = dictionary! {
            "Type"    => Object::Name(b"Annot".to_vec()),
            "Subtype" => Object::Name(b"Widget".to_vec()),
            "FT"      => Object::Name(ft),
            "T"       => Object::string_literal(self.field_name.clone()),
            "V"       => Object::string_literal(""),
            "Rect"    => Object::Array(vec![
                Object::Real(x1), Object::Real(y1),
                Object::Real(x2), Object::Real(y2),
            ]),
            "DA"      => Object::string_literal("/Helvetica 12 Tf 0 g"),
        };
        // Radio buttons need Ff bit 15 set to distinguish them from checkboxes.
        if self.kind == FormFieldKind::Radio {
            field_dict.set("Ff", Object::Integer(1 << 15));
        }

        let field_id = doc.inner_mut().add_object(Object::Dictionary(field_dict));

        // ---- Add widget to page /Annots --------------------------------
        {
            let inner = doc.inner_mut();

            // Inspect /Annots outside of any mutable borrow so we can handle
            // both inline arrays and indirect references cleanly.
            let annots_state = inner
                .get_object(page_id)
                .ok()
                .and_then(|o| o.as_dict().ok())
                .and_then(|d| d.get(b"Annots").ok().cloned());

            match annots_state {
                Some(Object::Array(mut arr)) => {
                    // Direct inline array — append and write back.
                    arr.push(Object::Reference(field_id));
                    inner
                        .get_object_mut(page_id)
                        .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?
                        .as_dict_mut()
                        .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?
                        .set("Annots", Object::Array(arr));
                }
                Some(Object::Reference(array_id)) => {
                    // Indirect reference to an existing array — append to it
                    // without overwriting /Annots (preserves existing annotations).
                    inner
                        .get_object_mut(array_id)
                        .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?
                        .as_array_mut()
                        .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?
                        .push(Object::Reference(field_id));
                }
                _ => {
                    // No /Annots or unexpected type — create a new inline array.
                    inner
                        .get_object_mut(page_id)
                        .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?
                        .as_dict_mut()
                        .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?
                        .set("Annots", Object::Array(vec![Object::Reference(field_id)]));
                }
            }
        }

        // ---- Locate the catalog and (optionally) existing AcroForm ----
        let catalog_id: ObjectId = {
            doc.inner()
                .trailer
                .get(b"Root")
                .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?
                .as_reference()
                .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?
        };

        let acroform_id: Option<ObjectId> = doc
            .inner()
            .get_object(catalog_id)
            .ok()
            .and_then(|o| o.as_dict().ok())
            .and_then(|d| d.get(b"AcroForm").ok())
            .and_then(|o| o.as_reference().ok());

        if let Some(af_id) = acroform_id {
            // AcroForm already exists — append to /Fields.
            let inner = doc.inner_mut();

            // First, inspect the existing /Fields entry without mutably borrowing
            // the AcroForm dictionary, so we can safely resolve any references.
            let fields_obj = inner
                .get_object(af_id)
                .ok()
                .and_then(|o| o.as_dict().ok())
                .and_then(|d| d.get(b"Fields").ok().cloned());

            match fields_obj {
                // /AcroForm/Fields is a direct array — clone, append, and write back.
                Some(Object::Array(mut arr)) => {
                    arr.push(Object::Reference(field_id));
                    inner
                        .get_object_mut(af_id)
                        .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?
                        .as_dict_mut()
                        .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?
                        .set("Fields", Object::Array(arr));
                }
                // /AcroForm/Fields is an indirect reference to an array — resolve
                // and append to the underlying array without overwriting /Fields.
                Some(Object::Reference(array_id)) => {
                    inner
                        .get_object_mut(array_id)
                        .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?
                        .as_array_mut()
                        .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?
                        .push(Object::Reference(field_id));
                }
                // No /Fields entry, or it is of an unexpected type — create a new array.
                _ => {
                    inner
                        .get_object_mut(af_id)
                        .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?
                        .as_dict_mut()
                        .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?
                        .set("Fields", Object::Array(vec![Object::Reference(field_id)]));
                }
            }
        } else {
            // No AcroForm yet — create a minimal one and wire it to the catalog.
            let new_af_id = doc.inner_mut().add_object(Object::Dictionary(dictionary! {
                "Fields" => Object::Array(vec![Object::Reference(field_id)]),
            }));

            doc.inner_mut()
                .get_object_mut(catalog_id)
                .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?
                .as_dict_mut()
                .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?
                .set("AcroForm", Object::Reference(new_af_id));
        }

        tracing::debug!(
            field_name = %self.field_name,
            page_index = self.page_index,
            "form field created"
        );
        Ok(())
    }

    fn undo(&mut self, doc: &mut Document) -> Result<(), PdfCoreError> {
        let snap = self.snapshot.as_ref().ok_or(PdfCoreError::NotUndoable)?;
        let restored =
            lopdf::Document::load_mem(snap).map_err(|e| PdfCoreError::LopdfError(e.to_string()))?;
        *doc.inner_mut() = restored;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        detector::detect_form_fields,
        types::{FormFieldKind, FormFieldValue},
    };
    use lopdf::{dictionary, Document as LopdfDoc, Object, Stream};
    use pdf_core::Document;
    use tempfile::NamedTempFile;

    fn blank_pdf() -> NamedTempFile {
        let mut doc = LopdfDoc::with_version("1.7");
        let pages_id = doc.new_object_id();
        let page_id = doc.new_object_id();
        let content = Stream::new(dictionary! {}, b"BT ET".to_vec());
        let content_id = doc.add_object(content);
        let page = Object::Dictionary(dictionary! {
            "Type"     => Object::Name(b"Page".to_vec()),
            "Parent"   => Object::Reference(pages_id),
            "MediaBox" => Object::Array(vec![
                Object::Integer(0), Object::Integer(0),
                Object::Integer(595), Object::Integer(842),
            ]),
            "Contents" => Object::Reference(content_id),
        });
        doc.objects.insert(page_id, page);
        let pages = Object::Dictionary(dictionary! {
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
        let mut f = NamedTempFile::new().expect("temp");
        doc.save_to(f.as_file_mut()).expect("save");
        f
    }

    #[test]
    fn create_text_field_on_fresh_document() {
        let f = blank_pdf();
        let mut doc = Document::open(f.path()).expect("open");

        let mut cmd = CreateFieldCommand::new(
            "EmailAddress",
            FormFieldKind::TextField,
            0,
            [72.0, 700.0, 300.0, 720.0],
        );
        cmd.execute(&mut doc).expect("execute");

        let fields = detect_form_fields(&doc);
        assert_eq!(fields.len(), 1, "one field should exist");
        assert_eq!(fields[0].name, "EmailAddress");
        assert_eq!(fields[0].kind, FormFieldKind::TextField);
        assert_eq!(fields[0].value, FormFieldValue::Text("".into()));
    }

    #[test]
    fn create_field_undo_removes_it() {
        let f = blank_pdf();
        let mut doc = Document::open(f.path()).expect("open");

        let mut cmd = CreateFieldCommand::new(
            "Phone",
            FormFieldKind::TextField,
            0,
            [72.0, 650.0, 300.0, 670.0],
        );
        cmd.execute(&mut doc).expect("execute");
        assert_eq!(detect_form_fields(&doc).len(), 1);

        cmd.undo(&mut doc).expect("undo");
        assert_eq!(
            detect_form_fields(&doc).len(),
            0,
            "field should be gone after undo"
        );
    }

    #[test]
    fn create_checkbox_field() {
        let f = blank_pdf();
        let mut doc = Document::open(f.path()).expect("open");
        let mut cmd = CreateFieldCommand::new(
            "Agree",
            FormFieldKind::Checkbox,
            0,
            [72.0, 600.0, 90.0, 618.0],
        );
        cmd.execute(&mut doc).expect("execute");
        let fields = detect_form_fields(&doc);
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].kind, FormFieldKind::Checkbox);
    }

    #[test]
    fn create_multiple_fields_appends_to_acroform() {
        let f = blank_pdf();
        let mut doc = Document::open(f.path()).expect("open");

        CreateFieldCommand::new(
            "First",
            FormFieldKind::TextField,
            0,
            [72.0, 780.0, 300.0, 800.0],
        )
        .execute(&mut doc)
        .expect("first");
        CreateFieldCommand::new(
            "Last",
            FormFieldKind::TextField,
            0,
            [72.0, 750.0, 300.0, 770.0],
        )
        .execute(&mut doc)
        .expect("second");

        let fields = detect_form_fields(&doc);
        assert_eq!(fields.len(), 2);
        assert!(fields.iter().any(|f| f.name == "First"));
        assert!(fields.iter().any(|f| f.name == "Last"));
    }

    #[test]
    fn create_field_on_nonexistent_page_fails() {
        let f = blank_pdf();
        let mut doc = Document::open(f.path()).expect("open");
        let mut cmd =
            CreateFieldCommand::new("X", FormFieldKind::TextField, 99, [0.0, 0.0, 100.0, 20.0]);
        assert!(cmd.execute(&mut doc).is_err());
    }

    #[test]
    fn create_unknown_kind_field_fails() {
        let f = blank_pdf();
        let mut doc = Document::open(f.path()).expect("open");
        let mut cmd =
            CreateFieldCommand::new("X", FormFieldKind::Unknown, 0, [0.0, 0.0, 100.0, 20.0]);
        assert!(
            cmd.execute(&mut doc).is_err(),
            "Unknown kind should return an error"
        );
    }
}
