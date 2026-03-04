use std::path::PathBuf;

/// Returns the platform-specific path where the license file is stored.
///
/// | Platform | Path                                                 |
/// |----------|------------------------------------------------------|
/// | Windows  | `%APPDATA%\PdfEditor\license.json`                  |
/// | macOS    | `~/Library/Application Support/PdfEditor/license.json` |
/// | Linux    | `~/.config/pdfeditor/license.json`                  |
pub fn license_file_path() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        let base = std::env::var("APPDATA").ok()?;
        Some(PathBuf::from(base).join("PdfEditor").join("license.json"))
    }
    #[cfg(target_os = "macos")]
    {
        let home = std::env::var("HOME").ok()?;
        Some(
            PathBuf::from(home)
                .join("Library")
                .join("Application Support")
                .join("PdfEditor")
                .join("license.json"),
        )
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        // Linux / other Unix
        if let Ok(config_dir) = std::env::var("XDG_CONFIG_HOME") {
            Some(
                PathBuf::from(config_dir)
                    .join("pdfeditor")
                    .join("license.json"),
            )
        } else {
            let home = std::env::var("HOME").ok()?;
            Some(
                PathBuf::from(home)
                    .join(".config")
                    .join("pdfeditor")
                    .join("license.json"),
            )
        }
    }
}

/// Returns the platform-specific path for the trial start timestamp file.
pub fn trial_start_file_path() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        let base = std::env::var("APPDATA").ok()?;
        Some(
            PathBuf::from(base)
                .join("PdfEditor")
                .join("trial_start.json"),
        )
    }
    #[cfg(target_os = "macos")]
    {
        let home = std::env::var("HOME").ok()?;
        Some(
            PathBuf::from(home)
                .join("Library")
                .join("Application Support")
                .join("PdfEditor")
                .join("trial_start.json"),
        )
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        if let Ok(config_dir) = std::env::var("XDG_CONFIG_HOME") {
            Some(
                PathBuf::from(config_dir)
                    .join("pdfeditor")
                    .join("trial_start.json"),
            )
        } else {
            let home = std::env::var("HOME").ok()?;
            Some(
                PathBuf::from(home)
                    .join(".config")
                    .join("pdfeditor")
                    .join("trial_start.json"),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn license_path_is_some() {
        // The function should return Some on any supported platform.
        assert!(license_file_path().is_some());
    }

    #[test]
    fn trial_path_is_some() {
        assert!(trial_start_file_path().is_some());
    }
}
