use serde::{Deserialize, Serialize};

/// Features that can be gated by the license.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Feature {
    Editor,
    Ocr,
    Forms,
    BatchProcessing,
}

impl Feature {
    /// Returns the string token used inside the license `features` array.
    pub fn as_token(&self) -> &'static str {
        match self {
            Feature::Editor => "editor",
            Feature::Ocr => "ocr",
            Feature::Forms => "forms",
            Feature::BatchProcessing => "batch",
        }
    }
}
