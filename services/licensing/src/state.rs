use crate::types::LicenseType;
use chrono::NaiveDate;

/// The validated, immutable license state derived from cryptographic checks.
///
/// This struct is the only source of truth for license status in the
/// application. It is **never** constructed from boolean flags; it is always
/// derived from a successful signature verification.
#[derive(Debug, Clone)]
pub struct LicenseState {
    pub license_type: LicenseType,
    pub issued_to: String,
    pub expiry: NaiveDate,
    pub features: Vec<String>,
    pub seats: u32,
}

impl LicenseState {
    /// Returns `true` when this license is still within its validity period.
    pub fn is_valid(&self) -> bool {
        let today = chrono::Utc::now().date_naive();
        self.expiry >= today
    }

    /// Returns `true` when the license allows commercial use.
    pub fn is_commercial_allowed(&self) -> bool {
        self.is_valid()
            && matches!(
                self.license_type,
                LicenseType::Commercial | LicenseType::Enterprise
            )
    }

    /// Returns `true` when a specific feature token is included in this license.
    pub fn feature_enabled(&self, feature_token: &str) -> bool {
        self.is_valid() && self.features.iter().any(|f| f == feature_token)
    }
}
