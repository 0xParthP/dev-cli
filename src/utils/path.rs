//! Utilities for displaying filesystem paths.

use std::path::{Path, PathBuf};

/// Convert Windows canonical paths (`\\?\C:\...`) into human-readable paths.
///
/// On non-Windows platforms this returns the original path unchanged.
pub fn display_path(path: &Path) -> String {
    #[cfg(windows)]
    {
        let path = path.display().to_string();

        path.strip_prefix(r"\\?\").unwrap_or(&path).to_string()
    }

    #[cfg(not(windows))]
    {
        path.display().to_string()
    }
}

/// Return a cleaned `PathBuf` without the Windows verbatim prefix.
pub fn normalize_path(path: PathBuf) -> PathBuf {
    #[cfg(windows)]
    {
        PathBuf::from(display_path(&path))
    }

    #[cfg(not(windows))]
    {
        path
    }
}
