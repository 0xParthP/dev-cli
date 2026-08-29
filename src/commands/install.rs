//! Install command implementation.

use anyhow::Result;

use crate::{cli::InstallCommand, installer};

/// Executes the `dev install` command.
///
/// The command currently installs the CLI globally (Windows-first).
/// Future versions may support updating and uninstalling.
pub fn execute(_cmd: InstallCommand) -> Result<()> {
    installer::install()
}
