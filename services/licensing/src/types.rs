use serde::{Deserialize, Serialize};

/// License types supported by the application.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LicenseType {
    Personal,
    Commercial,
    Trial,
    Enterprise,
}

/// The on-disk license file format.
///
/// The `signature` field is **excluded** from the signed payload; the payload
/// is the JSON of this struct with the `signature` field removed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseFile {
    pub license_id: String,
    pub license_type: LicenseType,
    pub issued_to: String,
    pub seats: u32,
    /// ISO 8601 date string, e.g. `"2028-01-01"`.
    pub expiry: String,
    pub features: Vec<String>,
    /// Product identifier that must match the compiled-in `PRODUCT_NAME`.
    pub product: String,
    /// Base64-encoded ED25519 signature over the payload (all other fields).
    pub signature: String,
}
