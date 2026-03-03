use crate::detector::detect_form_fields;
use crate::types::FormFieldValue;
use pdf_core::Document;
use serde_json::{json, Value};

pub fn export_form_data(doc: &Document) -> Value {
    let fields = detect_form_fields(doc);
    let mut map = serde_json::Map::new();
    for field in fields {
        let v: Value = match field.value {
            FormFieldValue::Text(s)    => json!(s),
            FormFieldValue::Boolean(b) => json!(b),
            FormFieldValue::Selected(s)=> json!(s),
            FormFieldValue::None       => Value::Null,
        };
        map.insert(field.full_name, v);
    }
    Value::Object(map)
}
