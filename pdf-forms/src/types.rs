use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FormFieldKind {
    TextField,
    Checkbox,
    Radio,
    Dropdown,
    SignatureField,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FormFieldValue {
    Text(String),
    Boolean(bool),
    Selected(String),
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormField {
    pub name: String,
    pub full_name: String,
    pub kind: FormFieldKind,
    pub value: FormFieldValue,
    pub page_index: Option<u32>,
    pub rect: Option<[f32; 4]>,
    pub options: Vec<String>,
    #[serde(skip)]
    pub object_id: Option<(u32, u16)>,
}
