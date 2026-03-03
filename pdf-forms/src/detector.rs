use crate::types::{FormField, FormFieldKind, FormFieldValue};
use lopdf::Object;
use pdf_core::Document;
use tracing::debug;

pub fn detect_form_fields(doc: &Document) -> Vec<FormField> {
    let inner = doc.inner();

    let catalog_dict = match inner.catalog() {
        Ok(d) => d,
        Err(_) => return Vec::new(),
    };

    let acroform_obj = match catalog_dict.get(b"AcroForm").ok() {
        Some(o) => o.clone(),
        None => return Vec::new(),
    };

    let acroform = match &acroform_obj {
        Object::Reference(id) => {
            match inner.get_object(*id).ok().and_then(|o| o.as_dict().ok()) {
                Some(d) => d.clone(),
                None => return Vec::new(),
            }
        }
        Object::Dictionary(d) => d.clone(),
        _ => return Vec::new(),
    };

    let fields_arr = match acroform.get(b"Fields")
        .ok()
        .and_then(|o| o.as_array().ok())
    {
        Some(arr) => arr.clone(),
        None => return Vec::new(),
    };

    let mut result = Vec::new();
    for field_ref in &fields_arr {
        collect_fields(inner, field_ref, "", &mut result);
    }
    debug!(field_count = result.len(), "AcroForm fields detected");
    result
}

fn collect_fields(
    doc: &lopdf::Document,
    field_obj: &Object,
    parent_name: &str,
    out: &mut Vec<FormField>,
) {
    let obj_id = match field_obj.as_reference() {
        Ok(id) => id,
        Err(_) => return,
    };
    let dict = match doc.get_object(obj_id).ok().and_then(|o| o.as_dict().ok()) {
        Some(d) => d.clone(),
        None => return,
    };

    let partial_name = dict.get(b"T").ok()
        .and_then(|o| o.as_str().ok())
        .map(|b| String::from_utf8_lossy(b).into_owned())
        .unwrap_or_default();

    let full_name = if parent_name.is_empty() {
        partial_name.clone()
    } else {
        format!("{}.{}", parent_name, partial_name)
    };

    if let Ok(kids) = dict.get(b"Kids").and_then(|o| o.as_array()) {
        let kids = kids.clone();
        for kid in &kids {
            collect_fields(doc, kid, &full_name, out);
        }
        return;
    }

    let ft = dict.get(b"FT").ok()
        .and_then(|o| o.as_name().ok())
        .map(|b| String::from_utf8_lossy(b).into_owned())
        .unwrap_or_default();
    let kind = match ft.as_str() {
        "Tx" => FormFieldKind::TextField,
        "Btn" => {
            let ff = dict.get(b"Ff").ok()
                .and_then(|o| o.as_i64().ok())
                .unwrap_or(0);
            if ff & (1 << 15) != 0 { FormFieldKind::Radio } else { FormFieldKind::Checkbox }
        }
        "Ch" => FormFieldKind::Dropdown,
        "Sig" => FormFieldKind::SignatureField,
        _ => FormFieldKind::Unknown,
    };

    let value = match &kind {
        FormFieldKind::TextField => {
            let v = dict.get(b"V").ok()
                .and_then(|o| o.as_str().ok())
                .map(|b| String::from_utf8_lossy(b).into_owned())
                .unwrap_or_default();
            FormFieldValue::Text(v)
        }
        FormFieldKind::Checkbox | FormFieldKind::Radio => {
            let v = dict.get(b"V").ok()
                .and_then(|o| o.as_name().ok())
                .map(|b| String::from_utf8_lossy(b).into_owned())
                .unwrap_or_else(|| "Off".to_owned());
            FormFieldValue::Boolean(v != "Off")
        }
        FormFieldKind::Dropdown => {
            let v = dict.get(b"V").ok()
                .and_then(|o| o.as_str().ok())
                .map(|b| String::from_utf8_lossy(b).into_owned())
                .unwrap_or_default();
            FormFieldValue::Selected(v)
        }
        _ => FormFieldValue::None,
    };

    let rect = dict.get(b"Rect").ok()
        .and_then(|o| o.as_array().ok())
        .and_then(|arr| {
            let ns: Vec<f32> = arr.iter().filter_map(|o| match o {
                Object::Integer(i) => Some(*i as f32),
                Object::Real(r) => Some(*r as f32),
                _ => None,
            }).collect();
            if ns.len() == 4 { Some([ns[0], ns[1], ns[2], ns[3]]) } else { None }
        });

    let options = dict.get(b"Opt").ok()
        .and_then(|o| o.as_array().ok())
        .map(|arr| arr.iter().filter_map(|o| {
            o.as_str().ok().map(|b| String::from_utf8_lossy(b).into_owned())
        }).collect())
        .unwrap_or_default();

    out.push(FormField {
        name: partial_name,
        full_name,
        kind,
        value,
        page_index: None,
        rect,
        options,
        object_id: Some(obj_id),
    });
}
