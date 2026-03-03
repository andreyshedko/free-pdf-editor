use crate::{
    detector::detect_form_fields,
    types::FormFieldValue,
};
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
    fn description(&self) -> &str { "Set form field value" }

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
            FormFieldValue::Boolean(b) => Object::Name(if *b { b"Yes".to_vec() } else { b"Off".to_vec() }),
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
            FormFieldValue::Boolean(b) => Object::Name(if *b { b"Yes".to_vec() } else { b"Off".to_vec() }),
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
