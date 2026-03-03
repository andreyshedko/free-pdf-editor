//! Platform-specific integration: file dialogs, clipboard, OS services.
//!
//! # Thread ownership
//! Functions in this crate may only be called from threads that are allowed to
//! interact with the OS UI (typically the main/UI thread).
//!
//! # Safety
//! No unsafe code in this crate.

use std::path::PathBuf;

/// Show an "Open File" dialog and return the selected path, if any.
///
/// Returns `None` if the user cancelled.
pub fn pick_open_file() -> Option<PathBuf> {
    // Production: integrate with native file-dialog crate per platform.
    // Stub: return None (user must supply a path via command line or drag-drop).
    tracing::info!("pick_open_file called (stub — returns None)");
    None
}

/// Show a "Save File" dialog and return the chosen path, if any.
pub fn pick_save_file() -> Option<PathBuf> {
    tracing::info!("pick_save_file called (stub — returns None)");
    None
}

/// Write `text` to the system clipboard.
pub fn set_clipboard_text(_text: &str) {
    tracing::debug!("set_clipboard_text (stub)");
}

/// Read text from the system clipboard.
pub fn get_clipboard_text() -> Option<String> {
    tracing::debug!("get_clipboard_text (stub)");
    None
}
