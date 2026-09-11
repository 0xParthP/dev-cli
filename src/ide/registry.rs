//! IDE registry and detected IDE information.

use crate::models::ide::Ide;
use std::path::PathBuf;

/// Information about a detected IDE on the system.
#[derive(Debug, Clone)]
pub struct InstalledIde {
    /// Which IDE type (Vscode, Cursor, etc.)
    pub ide: Ide,

    /// Display name for user output (e.g., "VS Code", "Cursor")
    pub display_name: String,

    /// Full path to IDE executable
    pub executable: PathBuf,
}

impl InstalledIde {
    /// Create a new installed IDE entry.
    pub fn new(ide: Ide, name: &str, exe: PathBuf) -> Self {
        Self { ide, display_name: name.into(), executable: exe }
    }
}
