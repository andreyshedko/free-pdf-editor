//! Build script for the `licensing` crate.
//!
//! Reads the public key from the `APP_PUBLIC_KEY` environment variable at
//! compile time and passes it to the crate via `cargo:rustc-env`.
//!
//! If the variable is not set, a well-known test key is used so that
//! `cargo test` works out-of-the-box without any environment configuration.
//! This test key has no corresponding private key shipped in the repository.

fn main() {
    // Re-run this build script whenever APP_PUBLIC_KEY changes.
    println!("cargo:rerun-if-env-changed=APP_PUBLIC_KEY");

    let key = std::env::var("APP_PUBLIC_KEY").unwrap_or_else(|_| {
        // Deterministic test key (public half only; private key is not stored).
        // Replace with a real key for production builds via APP_PUBLIC_KEY.
        "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a".to_string()
    });

    println!("cargo:rustc-env=APP_PUBLIC_KEY={key}");
}
