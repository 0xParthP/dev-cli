//! Global installation command implementation.
//!
//! Implements `dev install` for installing dev-cli globally in user's system.

use anyhow::Result;

use crate::installer;

/// Execute the install command.
///
/// Copies the dev-cli executable to ~/.local/bin and initializes configuration.
/// After installation, `dev` command will be available in PATH.
///
/// # Errors
///
/// Returns error if installation fails (e.g., cannot write to target location).
pub fn execute() -> Result<()> {
    installer::install()
}
