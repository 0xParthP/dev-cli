//! Custom IDE type definitions.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// A user-configured custom IDE.
///
/// Unlike the built-in `Ide` enum variants which are auto-detected,
/// custom IDEs are manually added by the user with an explicit
/// executable path and optional argument template.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CustomIde {
    /// Unique identifier (e.g. "neovim", "fleet").
    pub id: String,

    /// Display name shown in the TUI (e.g. "Neovim", "JetBrains Fleet").
    pub display_name: String,

    /// Absolute path to the executable.
    pub executable: PathBuf,

    /// Optional argument template. Use `{path}` as a placeholder for the
    /// project directory.
    ///
    /// Example: `"--new-window {path}"`
    ///
    /// If `None`, the project path is passed as the sole argument.
    #[serde(default)]
    pub args_template: Option<String>,

    /// Whether this IDE has been verified (executable exists).
    #[serde(default)]
    pub verified: bool,
}
