//! IDE management command implementation.
//!
//! Implements `dev ide` subcommands for listing and managing IDEs.

use anyhow::Result;
use owo_colors::OwoColorize;

use crate::{
    cli::{IdeCommand, IdeSubcommand},
    ide::detect::detect_ides,
};

/// Execute an IDE command.
///
/// Dispatches to appropriate subcommand handler:
/// - `list` — List detected IDEs
///
/// # Errors
///
/// Returns error if any command operation fails.
pub fn execute(cmd: IdeCommand) -> Result<()> {
    match cmd.command {
        IdeSubcommand::List => list(),
    }
}

/// List all detected IDEs on the system.
///
/// Runs the IDE detection algorithm and displays results.
/// Shows IDE name and executable path for each detected IDE.
///
/// # Errors
///
/// Returns error if IDE detection fails.
fn list() -> Result<()> {
    println!("{}", "Installed IDEs".bold());

    let ides = detect_ides();

    if ides.is_empty() {
        println!("No supported IDEs detected.");
        return Ok(());
    }

    for ide in ides {
        println!("{} {}", "✓".green(), ide.display_name);
        println!("    {}", ide.executable.display());
    }

    Ok(())
}
