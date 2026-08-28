//! IDE registry and detected IDE information.
//!
//! Types for storing information about detected IDEs.

use std::path::PathBuf;

use crate::models::ide::Ide;

/// Information about a detected IDE on the system.
///
/// Created by IDE detection algorithm when an IDE is found.
/// Contains enough information to display the IDE and launch it.
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
    ///
    /// # Arguments
    ///
    /// * `ide` — IDE enum type
    /// * `name` — Display name
    /// * `exe` — Full path to executable
    ///
    /// # Example
    ///
    /// ```
    /// use dev_cli::ide::registry::InstalledIde;
    /// use dev_cli::models::ide::Ide;
    /// use std::path::PathBuf;
    ///
    /// let ide = InstalledIde::new(
    ///     Ide::Cursor,
    ///     "Cursor",
    ///     PathBuf::from("C:\\Program Files\\Cursor\\Cursor.exe"),
    /// );
    ///
    /// assert_eq!(ide.display_name, "Cursor");
    /// ```
    pub fn new(ide: Ide, name: &str, exe: PathBuf) -> Self {
        Self {
            ide,
            display_name: name.into(),
            executable: exe,
        }
    }
}
