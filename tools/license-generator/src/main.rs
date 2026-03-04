//! License Generator CLI for PDF Editor.
//!
//! # Usage
//!
//! ```text
//! export LICENSE_PRIVATE_KEY=<hex-encoded 32-byte ed25519 private key seed>
//!
//! license-generator generate \
//!     --holder "ACME Inc" \
//!     --email admin@acme.com \
//!     --type commercial \
//!     --seats 10
//!
//! license-generator inspect path/to/license.pdfeditor-license
//! ```
//!
//! The private key must be provided via the `LICENSE_PRIVATE_KEY` environment
//! variable and is **never** stored in the repository.

use chrono::Utc;
use ed25519_dalek::{Signer, SigningKey};
use serde::{Deserialize, Serialize};
use std::{env, path::PathBuf, process};

// ---------------------------------------------------------------------------
// License data model (must stay in sync with services/licensing/src/types.rs)
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
struct LicenseFile {
    license_id: String,
    license_type: String,
    issued_to: String,
    seats: u32,
    expiry: String,
    features: Vec<String>,
    product: String,
    signature: String,
}

// ---------------------------------------------------------------------------
// CLI entry point
// ---------------------------------------------------------------------------

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage(&args[0]);
        process::exit(1);
    }

    match args[1].as_str() {
        "generate" => cmd_generate(&args[2..]),
        "inspect" => cmd_inspect(&args[2..]),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            usage(&args[0]);
            process::exit(1);
        }
    }
}

fn usage(program: &str) {
    eprintln!(
        "Usage:\n  \
         {program} generate --holder <name> --email <email> --type <type> --seats <n> [--expiry YYYY-MM-DD]\n  \
         {program} inspect <path>\n\n\
         License types: personal, commercial, trial, enterprise\n\n\
         Environment variables:\n  \
         LICENSE_PRIVATE_KEY  hex-encoded 32-byte ed25519 signing key seed (required for generate)"
    );
}

// ---------------------------------------------------------------------------
// generate sub-command
// ---------------------------------------------------------------------------

fn cmd_generate(args: &[String]) {
    let mut holder = String::new();
    let mut email = String::new();
    let mut license_type = String::new();
    let mut seats: u32 = 1;
    let mut expiry = "9999-12-31".to_string();

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--holder" => {
                i += 1;
                holder = require_arg(args, i, "--holder");
            }
            "--email" => {
                i += 1;
                email = require_arg(args, i, "--email");
            }
            "--type" => {
                i += 1;
                license_type = require_arg(args, i, "--type");
            }
            "--seats" => {
                i += 1;
                let s = require_arg(args, i, "--seats");
                seats = s.parse().unwrap_or_else(|_| {
                    eprintln!("--seats must be a positive integer");
                    process::exit(1);
                });
            }
            "--expiry" => {
                i += 1;
                let raw = require_arg(args, i, "--expiry");
                // Validate the format matches what the verifier expects.
                if chrono::NaiveDate::parse_from_str(&raw, "%Y-%m-%d").is_err() {
                    eprintln!("--expiry must be in YYYY-MM-DD format (e.g. 2028-12-31)");
                    process::exit(1);
                }
                expiry = raw;
            }
            other => {
                eprintln!("Unknown argument: {other}");
                process::exit(1);
            }
        }
        i += 1;
    }

    if holder.is_empty() || email.is_empty() || license_type.is_empty() {
        eprintln!("--holder, --email, and --type are required");
        process::exit(1);
    }

    // Load signing key from environment.
    let key_hex = env::var("LICENSE_PRIVATE_KEY").unwrap_or_else(|_| {
        eprintln!("LICENSE_PRIVATE_KEY environment variable is not set");
        process::exit(1);
    });
    let signing_key = load_signing_key(&key_hex);

    // Derive feature list from license type.
    let features: Vec<String> = match license_type.as_str() {
        "personal" => vec!["editor".into()],
        "trial" => vec!["editor".into(), "forms".into()],
        "commercial" => vec!["editor".into(), "ocr".into(), "forms".into()],
        "enterprise" => vec![
            "editor".into(),
            "ocr".into(),
            "forms".into(),
            "batch".into(),
        ],
        other => {
            eprintln!(
                "Unknown license type: {other}. Use: personal, trial, commercial, enterprise"
            );
            process::exit(1);
        }
    };

    // Build license ID from current timestamp + holder initials + random suffix.
    use rand::Rng;
    let issued_at = Utc::now().format("%Y%m%d%H%M%S").to_string();
    let initials: String = holder
        .split_whitespace()
        .filter_map(|w| w.chars().next())
        .collect::<String>()
        .to_uppercase();
    let suffix: u32 = rand::thread_rng().gen_range(1000..9999);
    let license_id = format!("LIC-{issued_at}-{initials}-{suffix}");

    let mut license = LicenseFile {
        license_id,
        license_type: license_type.clone(),
        issued_to: format!("{holder} <{email}>"),
        seats,
        expiry,
        features,
        product: "PdfEditor".into(),
        signature: String::new(),
    };

    // Build canonical payload and sign.
    let payload = build_payload(&license).unwrap_or_else(|e| {
        eprintln!("Failed to build payload: {e}");
        process::exit(1);
    });
    let sig = signing_key.sign(payload.as_bytes());
    license.signature = encode_base64(&sig.to_bytes());

    // Serialise final license to JSON.
    let json = serde_json::to_string_pretty(&license).unwrap_or_else(|e| {
        eprintln!("Failed to serialise license: {e}");
        process::exit(1);
    });

    // Write to file.
    // Sanitize holder for use in the filename: allow only [A-Za-z0-9_-], replace others with '_'.
    let mut safe_holder: String = holder
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect();
    if safe_holder.is_empty() {
        safe_holder = "license".to_string();
    }
    let filename = PathBuf::from(format!(
        "{}-{}.pdfeditor-license",
        safe_holder.to_lowercase(),
        license_type
    ));
    std::fs::write(&filename, &json).unwrap_or_else(|e| {
        eprintln!("Failed to write license file: {e}");
        process::exit(1);
    });

    println!("License generated: {}", filename.display());
    println!("{json}");
}

// ---------------------------------------------------------------------------
// inspect sub-command
// ---------------------------------------------------------------------------

fn cmd_inspect(args: &[String]) {
    if args.is_empty() {
        eprintln!("inspect requires a path argument");
        process::exit(1);
    }
    let path = &args[0];
    let content = std::fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("Cannot read {path}: {e}");
        process::exit(1);
    });
    let license: LicenseFile = serde_json::from_str(&content).unwrap_or_else(|e| {
        eprintln!("Invalid license JSON: {e}");
        process::exit(1);
    });

    println!("License ID : {}", license.license_id);
    println!("Type       : {}", license.license_type);
    println!("Issued to  : {}", license.issued_to);
    println!("Product    : {}", license.product);
    println!("Seats      : {}", license.seats);
    println!("Expiry     : {}", license.expiry);
    println!("Features   : {}", license.features.join(", "));
    println!(
        "Signature  : {}…",
        &license.signature[..16.min(license.signature.len())]
    );
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn require_arg(args: &[String], i: usize, name: &str) -> String {
    if i >= args.len() {
        eprintln!("Missing value for {name}");
        process::exit(1);
    }
    args[i].clone()
}

fn load_signing_key(hex: &str) -> SigningKey {
    let bytes = decode_hex(hex).unwrap_or_else(|e| {
        eprintln!("Invalid LICENSE_PRIVATE_KEY: {e}");
        process::exit(1);
    });
    let arr: [u8; 32] = bytes.try_into().unwrap_or_else(|_| {
        eprintln!("LICENSE_PRIVATE_KEY must be exactly 32 bytes (64 hex chars)");
        process::exit(1);
    });
    SigningKey::from_bytes(&arr)
}

fn decode_hex(s: &str) -> Result<Vec<u8>, String> {
    if !s.len().is_multiple_of(2) {
        return Err("odd-length hex string".into());
    }
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string()))
        .collect()
}

fn encode_base64(data: &[u8]) -> String {
    use base64::{engine::general_purpose::STANDARD, Engine};
    STANDARD.encode(data)
}

/// Builds the canonical JSON payload (all fields except `signature`), with
/// object keys sorted alphabetically for determinism.
fn build_payload(license: &LicenseFile) -> Result<String, String> {
    let val = serde_json::to_value(license).map_err(|e| e.to_string())?;
    serialise_sorted(&val)
}

fn serialise_sorted(val: &serde_json::Value) -> Result<String, String> {
    use serde_json::Value;
    match val {
        Value::Object(map) => {
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            let mut parts = Vec::with_capacity(keys.len());
            for k in &keys {
                if *k == "signature" {
                    continue;
                }
                let v = serialise_sorted(&map[*k])?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_base64_roundtrip() {
        let data = b"Hello, World!";
        let encoded = encode_base64(data);
        // Standard base64 of "Hello, World!" is "SGVsbG8sIFdvcmxkIQ=="
        assert_eq!(encoded, "SGVsbG8sIFdvcmxkIQ==");
    }

    #[test]
    fn decode_hex_valid() {
        assert_eq!(
            decode_hex("deadbeef").unwrap(),
            vec![0xde, 0xad, 0xbe, 0xef]
        );
    }

    #[test]
    fn payload_excludes_signature() {
        let license = LicenseFile {
            license_id: "LIC-001".into(),
            license_type: "commercial".into(),
            issued_to: "Test <test@test.com>".into(),
            seats: 1,
            expiry: "2099-01-01".into(),
            features: vec!["editor".into()],
            product: "PdfEditor".into(),
            signature: "should-not-appear".into(),
        };
        let payload = build_payload(&license).unwrap();
        assert!(!payload.contains("should-not-appear"));
        assert!(!payload.contains("signature"));
    }

    #[test]
    fn generate_and_verify_signature() {
        use base64::{engine::general_purpose::STANDARD, Engine};
        use ed25519_dalek::{Signer, SigningKey, Verifier};
        use rand::rngs::OsRng;

        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();

        let mut license = LicenseFile {
            license_id: "LIC-TEST".into(),
            license_type: "commercial".into(),
            issued_to: "ACME Inc <admin@acme.com>".into(),
            seats: 5,
            expiry: "2099-01-01".into(),
            features: vec!["editor".into(), "ocr".into()],
            product: "PdfEditor".into(),
            signature: String::new(),
        };

        let payload = build_payload(&license).unwrap();
        let sig = signing_key.sign(payload.as_bytes());
        license.signature = encode_base64(&sig.to_bytes());

        // Re-derive payload (signature field must be excluded).
        let payload2 = build_payload(&license).unwrap();
        assert_eq!(
            payload, payload2,
            "payload must be stable after adding signature"
        );

        // Decode the base64 signature and verify it against the payload.
        let sig_bytes = STANDARD.decode(&license.signature).unwrap();
        let arr: [u8; 64] = sig_bytes.try_into().unwrap();
        let signature = ed25519_dalek::Signature::from_bytes(&arr);
        assert!(verifying_key
            .verify(payload2.as_bytes(), &signature)
            .is_ok());
    }
}
