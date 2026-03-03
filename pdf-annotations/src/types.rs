use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AnnotationId(pub String);

impl AnnotationId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

impl Default for AnnotationId {
    fn default() -> Self { Self::new() }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn yellow() -> Self { Self { r: 1.0, g: 1.0, b: 0.0, a: 0.5 } }
    pub fn red()    -> Self { Self { r: 1.0, g: 0.0, b: 0.0, a: 1.0 } }
    pub fn black()  -> Self { Self { r: 0.0, g: 0.0, b: 0.0, a: 1.0 } }
    pub fn blue()   -> Self { Self { r: 0.0, g: 0.0, b: 1.0, a: 1.0 } }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AnnotationKind {
    Highlight { color: Color },
    Underline { color: Color },
    Strikeout { color: Color },
    Note { author: String, content: String },
    Drawing { color: Color, line_width: f32, points: Vec<(f32, f32)> },
    Stamp { label: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Annotation {
    pub id: AnnotationId,
    pub page_index: u32,
    pub rect: Rect,
    pub kind: AnnotationKind,
    #[serde(skip)]
    pub object_id: Option<(u32, u16)>,
}

impl Annotation {
    pub fn new(page_index: u32, rect: Rect, kind: AnnotationKind) -> Self {
        Self {
            id: AnnotationId::new(),
            page_index,
            rect,
            kind,
            object_id: None,
        }
    }

    pub fn pdf_subtype(&self) -> &'static str {
        match &self.kind {
            AnnotationKind::Highlight { .. } => "Highlight",
            AnnotationKind::Underline { .. } => "Underline",
            AnnotationKind::Strikeout { .. } => "StrikeOut",
            AnnotationKind::Note { .. }      => "Text",
            AnnotationKind::Drawing { .. }   => "Ink",
            AnnotationKind::Stamp { .. }     => "Stamp",
        }
    }
}
