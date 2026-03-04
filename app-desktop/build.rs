use std::env;
use std::fs;
use std::path::Path;

fn main() {
    slint_build::compile("ui/app.slint").unwrap();

    // ── Version injection from release/release.json ───────────────────────────
    // CI sets APP_VERSION, APP_CHANNEL, APP_BUILD_NUMBER as environment
    // variables before invoking cargo build.  Local dev falls back to values in
    // release/release.json, then to sensible defaults.
    let (version, channel, build_number) = read_release_metadata();

    println!("cargo:rustc-env=APP_VERSION={version}");
    println!("cargo:rustc-env=APP_CHANNEL={channel}");
    println!("cargo:rustc-env=APP_BUILD_NUMBER={build_number}");

    // Disable self-update for store builds (set STORE_BUILD=1 in CI for store targets).
    let store_build = env::var("STORE_BUILD").unwrap_or_else(|_| "0".to_string());
    println!("cargo:rustc-env=STORE_BUILD={store_build}");

    // Backend endpoints (empty string = feature disabled).
    let crash_endpoint = env::var("CRASH_ENDPOINT").unwrap_or_default();
    println!("cargo:rustc-env=CRASH_ENDPOINT={crash_endpoint}");

    let telemetry_endpoint = env::var("TELEMETRY_ENDPOINT").unwrap_or_default();
    println!("cargo:rustc-env=TELEMETRY_ENDPOINT={telemetry_endpoint}");

    let update_server_url = env::var("UPDATE_SERVER_URL").unwrap_or_default();
    println!("cargo:rustc-env=UPDATE_SERVER_URL={update_server_url}");

    // Re-run only when these files change.
    println!("cargo:rerun-if-changed=../release/release.json");
    println!("cargo:rerun-if-env-changed=APP_VERSION");
    println!("cargo:rerun-if-env-changed=APP_CHANNEL");
    println!("cargo:rerun-if-env-changed=APP_BUILD_NUMBER");
    println!("cargo:rerun-if-env-changed=STORE_BUILD");
    println!("cargo:rerun-if-env-changed=CRASH_ENDPOINT");
    println!("cargo:rerun-if-env-changed=TELEMETRY_ENDPOINT");
    println!("cargo:rerun-if-env-changed=UPDATE_SERVER_URL");
}

fn read_release_metadata() -> (String, String, String) {
    // CI-provided values take highest priority.
    if let (Ok(v), Ok(c), Ok(b)) = (
        env::var("APP_VERSION"),
        env::var("APP_CHANNEL"),
        env::var("APP_BUILD_NUMBER"),
    ) {
        return (v, c, b);
    }

    // Fall back to release/release.json (relative to workspace root).
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
    let json_path = Path::new(&manifest_dir)
        .parent()
        .unwrap_or(Path::new("."))
        .join("release")
        .join("release.json");

    if let Ok(raw) = fs::read_to_string(&json_path) {
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(&raw) {
            let version = val["version"].as_str().unwrap_or("0.0.0").to_string();
            let channel = val["channel"].as_str().unwrap_or("stable").to_string();
            let build_number = val["build_number"].as_u64().unwrap_or(0).to_string();
            return (version, channel, build_number);
        }
    }

    ("0.0.0".to_string(), "stable".to_string(), "0".to_string())
}
