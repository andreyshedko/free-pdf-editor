use crate::{detector::detect_form_fields, types::FormFieldValue};
use lopdf::Object;
use pdf_core::{Document, DocumentCommand, PdfCoreError};
use tracing::debug;

#[derive(Debug)]
pub struct SetFieldValueCommand {
    field_name: String,
    new_value: FormFieldValue,
    old_value: Option<FormFieldValue>,
}

impl SetFieldValueCommand {
    pub fn new(field_name: impl Into<String>, new_value: FormFieldValue) -> Self {
        Self {
            field_name: field_name.into(),
            new_value,
            old_value: None,
        }
    }
}

impl DocumentCommand for SetFieldValueCommand {
    fn description(&self) -> &str {
        "Set form field value"
    }

    fn execute(&mut self, doc: &mut Document) -> Result<(), PdfCoreError> {
        let fields = detect_form_fields(doc);
        let field = fields
            .iter()
            .find(|f| f.full_name == self.field_name || f.name == self.field_name)
            .ok_or_else(|| PdfCoreError::FieldNotFound(self.field_name.clone()))?;

        let obj_id = field
            .object_id
            .ok_or_else(|| PdfCoreError::FieldNotFound(self.field_name.clone()))?;

        self.old_value = Some(field.value.clone());

        let new_obj = match &self.new_value {
            FormFieldValue::Text(s) => Object::string_literal(s.clone()),
            FormFieldValue::Boolean(b) => {
                Object::Name(if *b { b"Yes".to_vec() } else { b"Off".to_vec() })
            }
            FormFieldValue::Selected(s) => Object::string_literal(s.clone()),
            FormFieldValue::None => Object::Null,
        };

        let inner = doc.inner_mut();
        inner
            .get_object_mut(obj_id)
            .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?
            .as_dict_mut()
            .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?
            .set("V", new_obj);

        debug!(field = %self.field_name, "form field value set");
        Ok(())
    }

    fn undo(&mut self, doc: &mut Document) -> Result<(), PdfCoreError> {
        let old = match self.old_value.take() {
            Some(v) => v,
            None => return Ok(()),
        };
        let fields = detect_form_fields(doc);
        let field = fields
            .iter()
            .find(|f| f.full_name == self.field_name || f.name == self.field_name)
            .ok_or_else(|| PdfCoreError::FieldNotFound(self.field_name.clone()))?;
        let obj_id = field
            .object_id
            .ok_or_else(|| PdfCoreError::FieldNotFound(self.field_name.clone()))?;
        let restored_obj = match &old {
            FormFieldValue::Text(s) => Object::string_literal(s.clone()),
            FormFieldValue::Boolean(b) => {
                Object::Name(if *b { b"Yes".to_vec() } else { b"Off".to_vec() })
            }
            FormFieldValue::Selected(s) => Object::string_literal(s.clone()),
            FormFieldValue::None => Object::Null,
        };
        doc.inner_mut()
            .get_object_mut(obj_id)
            .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?
            .as_dict_mut()
            .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?
            .set("V", restored_obj);
        self.old_value = None;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::detector::detect_form_fields;
    use crate::types::FormFieldKind;
    use lopdf::{dictionary, Document as LopdfDoc, Object};
    use pdf_core::Document;

    use tempfile::NamedTempFile;

    /// Build a minimal PDF with one AcroForm text field named "Name".
    fn pdf_with_text_field() -> NamedTempFile {
        let mut doc = LopdfDoc::with_version("1.7");
        let pages_id = doc.new_object_id();
        let page_id = doc.new_object_id();

        // The form field object
        let field_id = doc.add_object(Object::Dictionary(dictionary! {
            "Type"  => Object::Name(b"Annot".to_vec()),
            "FT"    => Object::Name(b"Tx".to_vec()),
            "T"     => Object::string_literal("Name"),
            "V"     => Object::string_literal(""),
            "Rect"  => Object::Array(vec![
                Object::Integer(100), Object::Integer(700),
                Object::Integer(300), Object::Integer(720),
            ]),
        }));

        let page = Object::Dictionary(dictionary! {
            "Type"     => Object::Name(b"Page".to_vec()),
            "Parent"   => Object::Reference(pages_id),
            "MediaBox" => Object::Array(vec![
                Object::Integer(0), Object::Integer(0),
                Object::Integer(595), Object::Integer(842),
            ]),
        });
        doc.objects.insert(page_id, page);

        let pages = Object::Dictionary(dictionary! {
            "Type"  => Object::Name(b"Pages".to_vec()),
            "Kids"  => Object::Array(vec![Object::Reference(page_id)]),
            "Count" => Object::Integer(1),
        });
        doc.objects.insert(pages_id, pages);

        let acroform_id = doc.add_object(Object::Dictionary(dictionary! {
            "Fields" => Object::Array(vec![Object::Reference(field_id)]),
        }));

        let catalog_id = doc.add_object(dictionary! {
            "Type"     => Object::Name(b"Catalog".to_vec()),
            "Pages"    => Object::Reference(pages_id),
            "AcroForm" => Object::Reference(acroform_id),
        });
        doc.trailer.set("Root", Object::Reference(catalog_id));

        let mut f = NamedTempFile::new().expect("temp");
        doc.save_to(f.as_file_mut()).expect("save");
        f
    }

    #[test]
    fn detect_form_fields_finds_text_field() {
        let f = pdf_with_text_field();
        let doc = Document::open(f.path()).expect("open");
        let fields = detect_form_fields(&doc);
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].name, "Name");
        assert_eq!(fields[0].kind, FormFieldKind::TextField);
    }

    #[test]
    fn set_field_value_execute_and_undo() {
        let f = pdf_with_text_field();
        let mut doc = Document::open(f.path()).expect("open");
        let mut cmd = SetFieldValueCommand::new("Name", FormFieldValue::Text("Alice".into()));
        cmd.execute(&mut doc).expect("execute");

        let fields = detect_form_fields(&doc);
        let field = fields.iter().find(|f| f.name == "Name").expect("field");
        assert_eq!(field.value, FormFieldValue::Text("Alice".into()));

        cmd.undo(&mut doc).expect("undo");
        let fields_after = detect_form_fields(&doc);
        let field_after = fields_after
            .iter()
            .find(|f| f.name == "Name")
            .expect("field");
        assert_eq!(field_after.value, FormFieldValue::Text("".into()));
    }

    #[test]
    fn set_field_value_nonexistent_field_fails() {
        let f = pdf_with_text_field();
        let mut doc = Document::open(f.path()).expect("open");
        let mut cmd = SetFieldValueCommand::new("NoSuchField", FormFieldValue::Text("x".into()));
        assert!(cmd.execute(&mut doc).is_err());
    }
}
