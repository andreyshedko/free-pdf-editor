use thiserror::Error;

#[derive(Debug, Error)]
pub enum LicenseError {
    #[error("license file not found")]
    NotFound,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid JSON: {0}")]
    InvalidJson(#[from] serde_json::Error),
    #[error("invalid signature")]
    InvalidSignature,
    #[error("invalid expiry date format: expected YYYY-MM-DD")]
    InvalidExpiry,
    #[error("license has expired")]
    Expired,
    #[error("invalid public key")]
    InvalidPublicKey,
    #[error("invalid base64: {0}")]
    InvalidBase64(String),
}
