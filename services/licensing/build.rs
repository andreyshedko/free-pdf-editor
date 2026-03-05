//! Build script for the `licensing` crate.
//!
//! Reads the public key from the `APP_PUBLIC_KEY` environment variable at
//! compile time and passes it to the crate via `cargo:rustc-env`.
//!
//! If the variable is not set, a well-known test key is used so that
//! `cargo test` works out-of-the-box without any environment configuration.
//! This test key has no corresponding private key shipped in the repository.

fn is_valid_hex_64(s: &str) -> bool {
    if s.len() != 64 {
        return false;
    }
    s.chars().all(|c| c.is_ascii_hexdigit())
}

fn main() {
    // Re-run this build script whenever APP_PUBLIC_KEY changes.
    println!("cargo:rerun-if-env-changed=APP_PUBLIC_KEY");

    let key = match std::env::var("APP_PUBLIC_KEY") {
        Ok(k) => k,
        Err(_) => {
            // Deterministic test key (public half only; the corresponding seed
            // is available in test code only — see manager::tests).
            // Replace with a real key for production builds via APP_PUBLIC_KEY.
            let fallback = "2152f8d19b791d24453242e15f2eab6cb7cffa7b6a5ed30097960e069881db12";

            let profile = std::env::var("PROFILE").unwrap_or_default();
            let is_debug = profile == "debug";
            let is_test = std::env::var("CARGO_CFG_TEST").is_ok();

            if !(is_debug || is_test) {
                println!(
                    "cargo:warning=APP_PUBLIC_KEY is not set; using insecure built-in fallback key. \
                     Set APP_PUBLIC_KEY to a 64-character hex-encoded Ed25519 public key for production builds."
                );
            }

            fallback.to_string()
        }
    };

    if !is_valid_hex_64(&key) {
        panic!(
            "APP_PUBLIC_KEY must be a 64-character hexadecimal string (got length {}).",
            key.len()
        );
    }
    println!("cargo:rustc-env=APP_PUBLIC_KEY={key}");
}
