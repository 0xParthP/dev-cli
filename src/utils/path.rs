//! Utilities for displaying filesystem paths.

use std::path::Path;

/// Convert Windows canonical paths (`\\?\C:\...`) into human-readable paths.
///
/// On non-Windows platforms this returns the original path unchanged.
pub fn display_path(path: &Path) -> String {
    let path = path.display().to_string();

    #[cfg(windows)]
    {
        path.strip_prefix(r"\\?\").unwrap_or(&path).to_string()
    }

    #[cfg(not(windows))]
    {
        path
    }
}
