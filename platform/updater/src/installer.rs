//! Package downloader and installer for self-update.

use super::checker::UpdateInfo;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use tracing::info;

/// Download `info.download_url`, verify SHA-256, and launch the installer.
///
/// On success this function does **not** return — the current process exits
/// after launching the installer so the user sees a single window.
pub fn download_and_install(info: &UpdateInfo) -> Result<(), Box<dyn std::error::Error>> {
    let tmp = std::env::temp_dir().join(format!("fpe_update_{}", &info.new_version));
    std::fs::create_dir_all(&tmp)?;

    let filename = info
        .download_url
        .split('/')
        .last()
        .unwrap_or("update_package");
    let dest: PathBuf = tmp.join(filename);

    info!(
        "Downloading update {} to {}",
        info.new_version,
        dest.display()
    );
    download_file(&info.download_url, &dest)?;

    verify_sha256(&dest, &info.sha256)?;
    info!("SHA-256 verified for update package");

    launch_installer(&dest)?;
    std::process::exit(0);
}

fn download_file(url: &str, dest: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let resp = ureq::get(url).call()?;
    let mut reader = resp.into_reader();
    let mut file = std::fs::File::create(dest)?;
    std::io::copy(&mut reader, &mut file)?;
    Ok(())
}

fn verify_sha256(path: &Path, expected: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Read;

    let mut file = std::fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 65536];
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    let digest = hex::encode(hasher.finalize());
    if digest.eq_ignore_ascii_case(expected) {
        Ok(())
    } else {
        Err(format!("SHA-256 mismatch: expected {expected}, got {digest}").into())
    }
}

fn launch_installer(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("msiexec")
            .arg("/i")
            .arg(path)
            .spawn()?;
        return Ok(());
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open").arg(path).spawn()?;
        return Ok(());
    }
    // On other platforms (e.g. Linux) the self-updater is not supported.
    // Suppress unused-variable warning for `path` in this branch.
    let _ = path;
    Err("self-update installer launch not supported on this platform".into())
}
