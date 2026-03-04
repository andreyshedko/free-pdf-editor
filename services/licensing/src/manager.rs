use crate::{
    error::LicenseError,
    feature::Feature,
    state::LicenseState,
    storage::{license_file_path, trial_start_file_path},
    types::{LicenseFile, LicenseType},
};
use chrono::{Duration, NaiveDate, Utc};
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use serde_json::Value;
use std::path::Path;
use tracing::{debug, info, warn};

/// Duration of the auto-generated trial period.
const TRIAL_DAYS: i64 = 14;

/// Compile-time embedded ED25519 public key (32 bytes, hex-encoded).
///
/// Override at build time by setting the `APP_PUBLIC_KEY` environment variable
/// (see `build.rs`).  The corresponding private key must exist only on the
/// licensing server / CLI tool.
const PUBLIC_KEY_HEX: &str = env!("APP_PUBLIC_KEY");

/// Product identifier that licenses must carry to be accepted by this binary.
const PRODUCT_NAME: &str = "PdfEditor";

/// Manages license loading, verification and state exposure.
///
/// # Usage
///
/// ```no_run
/// use licensing::{LicenseManager, Feature};
///
/// let mgr = LicenseManager::new();
/// let state = mgr.current_license();
/// println!("commercial: {}", mgr.is_commercial_allowed());
/// println!("ocr: {}", mgr.feature_enabled(Feature::Ocr));
/// ```
pub struct LicenseManager {
    state: LicenseState,
}

impl LicenseManager {
    /// Creates a new `LicenseManager`.
    ///
    /// Order of precedence:
    /// 1. Valid signed license file at the platform storage path.
    /// 2. Trial period (auto-generated on first launch, lasts 14 days).
    /// 3. Personal (fallback after trial expires).
    pub fn new() -> Self {
        let state = Self::resolve_state();
        info!(?state.license_type, "license resolved");
        Self { state }
    }

    /// Returns the current immutable [`LicenseState`].
    pub fn current_license(&self) -> &LicenseState {
        &self.state
    }

    /// Returns `true` when the current license permits commercial usage.
    pub fn is_commercial_allowed(&self) -> bool {
        self.state.is_commercial_allowed()
    }

    /// Returns `true` when the given [`Feature`] is enabled by the current license.
    pub fn feature_enabled(&self, feature: Feature) -> bool {
        self.state.feature_enabled(feature.as_token())
    }

    /// Activates a license from the file at `source_path`.
    ///
    /// The license is validated, then copied to the platform storage path.
    /// On success the internal state is updated immediately (no restart needed).
    ///
    /// # Errors
    ///
    /// Returns a [`LicenseError`] if the file is invalid or cannot be copied.
    pub fn activate(&mut self, source_path: &Path) -> Result<(), LicenseError> {
        let new_state = Self::load_and_verify(source_path)?;

        // Resolve the platform storage path; fail rather than silently
        // activating a license that will disappear on the next restart.
        let dest = license_file_path().ok_or(LicenseError::NoStoragePath)?;

        if let Some(parent) = dest.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Avoid truncating/corrupting the file when source and dest are the same.
        let same_file = std::fs::canonicalize(source_path)
            .ok()
            .zip(std::fs::canonicalize(&dest).ok())
            .map(|(a, b)| a == b)
            .unwrap_or(false);

        if !same_file {
            std::fs::copy(source_path, &dest)?;
            info!("license installed to {}", dest.display());
        }

        self.state = new_state;
        info!(?self.state.license_type, "license activated");
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Internal helpers
    // -----------------------------------------------------------------------

    fn resolve_state() -> LicenseState {
        // 1. Try loading a signed license file.
        if let Some(path) = license_file_path() {
            match Self::load_and_verify(&path) {
                Ok(state) => {
                    debug!("signed license loaded from {}", path.display());
                    return state;
                }
                Err(LicenseError::NotFound) => {
                    debug!("no license file at {}", path.display());
                }
                Err(e) => {
                    warn!("license file rejected: {e}");
                }
            }
        }

        // 2. Trial / Personal fallback.
        Self::trial_or_personal()
    }

    /// Loads and cryptographically verifies a license file.
    pub(crate) fn load_and_verify(path: &Path) -> Result<LicenseState, LicenseError> {
        if !path.exists() {
            return Err(LicenseError::NotFound);
        }

        let json_str = std::fs::read_to_string(path)?;
        let license: LicenseFile = serde_json::from_str(&json_str)?;

        // Validate product name first (fast check before the cryptographic op).
        if license.product != PRODUCT_NAME {
            return Err(LicenseError::WrongProduct);
        }

        // Verify the ED25519 signature.
        Self::verify_signature(&license)?;

        // Parse and validate expiry date.
        let expiry = NaiveDate::parse_from_str(&license.expiry, "%Y-%m-%d")
            .map_err(|_| LicenseError::InvalidExpiry)?;

        Ok(LicenseState {
            license_type: license.license_type,
            issued_to: license.issued_to,
            expiry,
            features: license.features,
            seats: license.seats,
        })
    }

    /// Verifies the ED25519 signature on a [`LicenseFile`].
    ///
    /// The signed payload is the JSON of the license **without** the
    /// `signature` field, serialised with keys in sorted order (deterministic).
    fn verify_signature(license: &LicenseFile) -> Result<(), LicenseError> {
        // Decode the compile-time public key.
        let key_bytes = decode_hex(PUBLIC_KEY_HEX).map_err(|_| LicenseError::InvalidPublicKey)?;
        let key_arr: [u8; 32] = key_bytes
            .try_into()
            .map_err(|_| LicenseError::InvalidPublicKey)?;
        let verifying_key =
            VerifyingKey::from_bytes(&key_arr).map_err(|_| LicenseError::InvalidPublicKey)?;

        // Build the canonical payload (all fields except `signature`).
        let payload = build_payload(license)?;

        // Decode the base64 signature stored in the file.
        let sig_bytes =
            decode_base64(&license.signature).map_err(|e| LicenseError::InvalidBase64(e))?;
        let sig_arr: [u8; 64] = sig_bytes
            .try_into()
            .map_err(|_| LicenseError::InvalidSignature)?;
        let signature = Signature::from_bytes(&sig_arr);

        verifying_key
            .verify(payload.as_bytes(), &signature)
            .map_err(|_| LicenseError::InvalidSignature)
    }

    /// Returns a trial `LicenseState` (auto-created on first launch) or a
    /// Personal state when the trial has expired.
    fn trial_or_personal() -> LicenseState {
        let today = Utc::now().date_naive();
        let trial_start = Self::load_or_create_trial_start();

        let trial_end = trial_start + Duration::days(TRIAL_DAYS);

        if today <= trial_end {
            debug!("trial active until {trial_end}");
            LicenseState {
                license_type: LicenseType::Trial,
                issued_to: "Trial User".into(),
                expiry: trial_end,
                features: vec!["editor".into(), "forms".into()],
                seats: 1,
            }
        } else {
            debug!("trial expired; falling back to Personal");
            LicenseState {
                license_type: LicenseType::Personal,
                issued_to: String::new(),
                // Personal has no hard expiry – set far future.
                expiry: NaiveDate::from_ymd_opt(9999, 12, 31).unwrap(),
                features: vec!["editor".into()],
                seats: 1,
            }
        }
    }

    /// Loads the trial start timestamp from disk, or creates it (first launch).
    fn load_or_create_trial_start() -> NaiveDate {
        let today = Utc::now().date_naive();

        let path = match trial_start_file_path() {
            Some(p) => p,
            None => return today,
        };

        if let Ok(content) = std::fs::read_to_string(&path) {
            if let Ok(date) = NaiveDate::parse_from_str(content.trim(), "%Y-%m-%d") {
                return date;
            }
        }

        // First launch: persist the trial start date.
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let _ = std::fs::write(&path, today.format("%Y-%m-%d").to_string());
        today
    }
}

impl Default for LicenseManager {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Helpers (no unsafe code)
// ---------------------------------------------------------------------------

/// Builds the canonical JSON payload that was signed.
///
/// The payload is the full license object serialised to a `serde_json::Value`,
/// with the `signature` key removed, then re-serialised to a compact JSON
/// string with keys in sorted order.
fn build_payload(license: &LicenseFile) -> Result<String, LicenseError> {
    let mut val: Value = serde_json::to_value(license)?;
    if let Some(obj) = val.as_object_mut() {
        obj.remove("signature");
    }
    // Re-serialise with sorted keys for determinism.
    serialise_sorted(&val)
}

/// Recursively serialises a `serde_json::Value` with object keys sorted.
fn serialise_sorted(val: &Value) -> Result<String, LicenseError> {
    match val {
        Value::Object(map) => {
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            let mut parts = Vec::with_capacity(keys.len());
            for k in keys {
                let v = serialise_sorted(&map[k])?;
                parts.push(format!("\"{}\":{}", k, v));
            }
            Ok(format!("{{{}}}", parts.join(",")))
        }
        Value::Array(arr) => {
            let parts: Result<Vec<_>, _> = arr.iter().map(serialise_sorted).collect();
            Ok(format!("[{}]", parts?.join(",")))
        }
        other => Ok(other.to_string()),
    }
}

fn decode_hex(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err("odd length hex string".into());
    }
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string()))
        .collect()
}

fn decode_base64(s: &str) -> Result<Vec<u8>, String> {
    // Use a simple Base64 decoder without external dependencies.
    // Standard alphabet (A-Z, a-z, 0-9, +, /)
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut table = [0xffu8; 256];
    for (i, &c) in ALPHABET.iter().enumerate() {
        table[c as usize] = i as u8;
    }

    let s = s.trim_end_matches('=');
    let mut out = Vec::with_capacity(s.len() * 3 / 4);
    let bytes = s.as_bytes();
    let chunks = bytes.chunks(4);
    for chunk in chunks {
        let b: Vec<u8> = chunk
            .iter()
            .map(|&c| {
                let v = table[c as usize];
                if v == 0xff {
                    Err(format!("invalid base64 character: {}", c as char))
                } else {
                    Ok(v)
                }
            })
            .collect::<Result<_, _>>()?;

        let combined = match b.len() {
            4 => (b[0] as u32) << 18 | (b[1] as u32) << 12 | (b[2] as u32) << 6 | b[3] as u32,
            3 => (b[0] as u32) << 18 | (b[1] as u32) << 12 | (b[2] as u32) << 6,
            2 => (b[0] as u32) << 18 | (b[1] as u32) << 12,
            _ => return Err("invalid base64 input".into()),
        };

        out.push((combined >> 16) as u8);
        if b.len() >= 3 {
            out.push((combined >> 8) as u8);
        }
        if b.len() == 4 {
            out.push(combined as u8);
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::LicenseType;

    fn make_trial_state(days_ago: i64) -> LicenseState {
        let today = Utc::now().date_naive();
        let trial_end = today + Duration::days(TRIAL_DAYS) - Duration::days(days_ago);
        LicenseState {
            license_type: LicenseType::Trial,
            issued_to: "Trial User".into(),
            expiry: trial_end,
            features: vec!["editor".into(), "forms".into()],
            seats: 1,
        }
    }

    #[test]
    fn trial_is_valid_within_period() {
        let state = make_trial_state(0);
        assert!(state.is_valid());
    }

    #[test]
    fn trial_expired() {
        let state = LicenseState {
            license_type: LicenseType::Trial,
            issued_to: "Trial User".into(),
            expiry: NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
            features: vec!["editor".into()],
            seats: 1,
        };
        assert!(!state.is_valid());
    }

    #[test]
    fn personal_is_not_commercial() {
        let state = LicenseState {
            license_type: LicenseType::Personal,
            issued_to: String::new(),
            expiry: NaiveDate::from_ymd_opt(9999, 12, 31).unwrap(),
            features: vec!["editor".into()],
            seats: 1,
        };
        assert!(!state.is_commercial_allowed());
    }

    #[test]
    fn commercial_allows_commercial() {
        let state = LicenseState {
            license_type: LicenseType::Commercial,
            issued_to: "ACME Corp".into(),
            expiry: NaiveDate::from_ymd_opt(9999, 12, 31).unwrap(),
            features: vec!["editor".into(), "ocr".into(), "forms".into()],
            seats: 5,
        };
        assert!(state.is_commercial_allowed());
        assert!(state.feature_enabled("ocr"));
        assert!(!state.feature_enabled("batch"));
    }

    #[test]
    fn enterprise_allows_commercial() {
        let state = LicenseState {
            license_type: LicenseType::Enterprise,
            issued_to: "BigCorp".into(),
            expiry: NaiveDate::from_ymd_opt(9999, 12, 31).unwrap(),
            features: vec![
                "editor".into(),
                "ocr".into(),
                "forms".into(),
                "batch".into(),
            ],
            seats: 100,
        };
        assert!(state.is_commercial_allowed());
        assert!(state.feature_enabled("batch"));
    }

    #[test]
    fn expired_commercial_not_allowed() {
        let state = LicenseState {
            license_type: LicenseType::Commercial,
            issued_to: "OldCorp".into(),
            expiry: NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
            features: vec!["editor".into(), "ocr".into()],
            seats: 1,
        };
        assert!(!state.is_commercial_allowed());
        assert!(!state.feature_enabled("ocr"));
    }

    #[test]
    fn decode_hex_roundtrip() {
        let bytes = vec![0x00u8, 0xffu8, 0xabu8, 0xcd_u8];
        let hex: String = bytes.iter().map(|b| format!("{:02x}", b)).collect();
        assert_eq!(decode_hex(&hex).unwrap(), bytes);
    }

    #[test]
    fn decode_base64_hello() {
        // "Hello" in base64 is "SGVsbG8="
        let decoded = decode_base64("SGVsbG8=").unwrap();
        assert_eq!(decoded, b"Hello");
    }

    #[test]
    fn invalid_signature_rejected() {
        let license = LicenseFile {
            license_id: "test-id".into(),
            license_type: LicenseType::Commercial,
            issued_to: "Test Corp".into(),
            seats: 1,
            expiry: "2099-01-01".into(),
            features: vec!["editor".into()],
            product: "PdfEditor".into(),
            signature: "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".into(),
        };
        assert!(matches!(
            LicenseManager::verify_signature(&license),
            Err(LicenseError::InvalidSignature)
        ));
    }

    #[test]
    fn signed_license_verifies() {
        // Generate a key pair and sign a real license for testing.
        use ed25519_dalek::{Signer, SigningKey};
        use rand::rngs::OsRng;

        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();

        let mut license = LicenseFile {
            license_id: "550e8400-e29b-41d4-a716-446655440000".into(),
            license_type: LicenseType::Commercial,
            issued_to: "Test Company".into(),
            seats: 3,
            expiry: "2099-01-01".into(),
            features: vec!["editor".into(), "ocr".into(), "forms".into()],
            product: "PdfEditor".into(),
            signature: String::new(),
        };

        // Build payload and sign it.
        let payload = build_payload(&license).unwrap();
        let signature = signing_key.sign(payload.as_bytes());

        // Encode signature to base64.
        license.signature = encode_base64(signature.to_bytes().as_ref());

        // Temporarily swap the public key constant for verification.
        // We verify directly using the verifying key here.
        let sig_bytes = decode_base64(&license.signature).unwrap();
        let sig_arr: [u8; 64] = sig_bytes.try_into().unwrap();
        let sig = Signature::from_bytes(&sig_arr);
        let payload2 = build_payload(&license).unwrap();
        assert!(verifying_key.verify(payload2.as_bytes(), &sig).is_ok());
    }

    /// Helper: encode bytes to standard base64.
    fn encode_base64(data: &[u8]) -> String {
        const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut out = String::new();
        for chunk in data.chunks(3) {
            let b0 = chunk[0] as u32;
            let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
            let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
            let combined = (b0 << 16) | (b1 << 8) | b2;
            out.push(CHARS[((combined >> 18) & 0x3f) as usize] as char);
            out.push(CHARS[((combined >> 12) & 0x3f) as usize] as char);
            if chunk.len() > 1 {
                out.push(CHARS[((combined >> 6) & 0x3f) as usize] as char);
            } else {
                out.push('=');
            }
            if chunk.len() > 2 {
                out.push(CHARS[(combined & 0x3f) as usize] as char);
            } else {
                out.push('=');
            }
        }
        out
    }

    #[test]
    fn serialise_sorted_is_deterministic() {
        let license = LicenseFile {
            license_id: "abc".into(),
            license_type: LicenseType::Personal,
            issued_to: "User".into(),
            seats: 1,
            expiry: "2099-01-01".into(),
            features: vec!["editor".into()],
            product: "PdfEditor".into(),
            signature: "sig".into(),
        };
        let p1 = build_payload(&license).unwrap();
        let p2 = build_payload(&license).unwrap();
        assert_eq!(p1, p2);
        // Signature must not appear in payload.
        assert!(!p1.contains("signature"));
    }

    #[test]
    fn wrong_product_rejected() {
        // verify that load_and_verify returns WrongProduct.
        // Product validation runs before signature verification, so any
        // placeholder signature suffices here.
        let license = LicenseFile {
            license_id: "wrong-prod".into(),
            license_type: LicenseType::Commercial,
            issued_to: "Evil Corp".into(),
            seats: 1,
            expiry: "2099-01-01".into(),
            features: vec!["editor".into()],
            product: "OtherApp".into(),
            signature: "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".into(),
        };

        let json = serde_json::to_string(&license).unwrap();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("wrong_product.json");
        std::fs::write(&path, json).unwrap();

        assert!(matches!(
            LicenseManager::load_and_verify(&path),
            Err(LicenseError::WrongProduct)
        ));
    }

    /// Test that `activate()` copies the license file to the platform storage
    /// path and immediately updates the manager's state.
    ///
    /// The test signs a license with the fixed test-seed `0x42 * 32` whose
    /// public half (`2152f8d...`) is the compile-time `APP_PUBLIC_KEY` in
    /// debug/test builds (see `build.rs`).
    #[test]
    fn activate_copies_license_and_updates_state() {
        use ed25519_dalek::{Signer, SigningKey};

        // Fixed test seed whose public key is the compile-time APP_PUBLIC_KEY
        // fallback (see build.rs).  This seed is test-only and publicly known.
        let signing_key = SigningKey::from_bytes(&[0x42u8; 32]);

        let mut license = LicenseFile {
            license_id: "LIC-ACTIVATE-TEST".into(),
            license_type: LicenseType::Commercial,
            issued_to: "Test Corp".into(),
            seats: 5,
            expiry: "2099-01-01".into(),
            features: vec!["editor".into(), "ocr".into()],
            product: "PdfEditor".into(),
            signature: String::new(),
        };
        let payload = build_payload(&license).unwrap();
        let sig = signing_key.sign(payload.as_bytes());
        license.signature = encode_base64(sig.to_bytes().as_ref());

        // Write the signed license to a source temp file.
        let source_dir = tempfile::tempdir().unwrap();
        let source_path = source_dir.path().join("test.pdfeditor-license");
        std::fs::write(&source_path, serde_json::to_string(&license).unwrap()).unwrap();

        // Guard that restores (or removes) XDG_CONFIG_HOME even on panic,
        // preventing this test from polluting parallel test environments.
        struct EnvGuard {
            key: &'static str,
            old: Option<String>,
        }
        impl Drop for EnvGuard {
            fn drop(&mut self) {
                match &self.old {
                    Some(v) => std::env::set_var(self.key, v),
                    None => std::env::remove_var(self.key),
                }
            }
        }
        let _guard = EnvGuard {
            key: "XDG_CONFIG_HOME",
            old: std::env::var("XDG_CONFIG_HOME").ok(),
        };

        // Point XDG_CONFIG_HOME to an isolated tempdir so the test doesn't
        // touch the developer's real license directory.
        let storage_dir = tempfile::tempdir().unwrap();
        std::env::set_var("XDG_CONFIG_HOME", storage_dir.path());

        // Create a fresh manager; it should start in trial/personal state
        // because the storage tempdir contains no license yet.
        let mut mgr = LicenseManager::new();
        assert_ne!(
            mgr.current_license().license_type,
            LicenseType::Commercial,
            "should start non-commercial before activation"
        );

        // Activate the license.
        mgr.activate(&source_path).expect("activate should succeed");

        // State must now reflect the commercial license.
        assert_eq!(
            mgr.current_license().license_type,
            LicenseType::Commercial,
            "state should become Commercial after activation"
        );

        // The license file must have been persisted to the storage path.
        let expected_dest = storage_dir.path().join("pdfeditor").join("license.json");
        assert!(
            expected_dest.exists(),
            "license file should be copied to the storage path"
        );
    }
}
