//! Project management command implementation.
//!
//! Implements `dev project` subcommands for listing and opening projects.

use anyhow::{bail, Result};
use owo_colors::OwoColorize;

use crate::{
    cli::{OpenArgs, ProjectCommand, ProjectSubcommand},
    config::Config,
    ide::launcher,
};

/// Execute a project command.
///
/// Dispatches to appropriate subcommand handler:
/// - `list` — List configured project roots
/// - `open` — Open a project in an IDE
///
/// # Errors
///
/// Returns error if any command operation fails.
pub fn execute(cmd: ProjectCommand) -> Result<()> {
    match cmd.command {
        ProjectSubcommand::List => list(),
        ProjectSubcommand::Open(args) => open(args),
    }
}

/// Shorthand for opening a project.
///
/// Used by `dev open <PROJECT>` which is equivalent to `dev project open <PROJECT>`.
///
/// # Errors
///
/// Returns error if opening fails.
pub fn open_shortcut(args: OpenArgs) -> Result<()> {
    open(args)
}

/// List all configured project root directories.
///
/// Displays the directories where dev-cli searches for projects.
///
/// # Errors
///
/// Returns error if configuration cannot be loaded.
fn list() -> Result<()> {
    let config = Config::load()?;

    println!("{}", "Configured Project Roots".bold());

    for root in config.projects_root {
        println!("📁 {}", root.display());
    }

    Ok(())
}

/// Open a project in an IDE.
///
/// Searches for the project in configured roots and launches in specified IDE.
/// If IDE is not specified, uses the configured default.
///
/// # Arguments
///
/// * `args` — Project name and optional IDE override
///
/// # Errors
///
/// Returns error if:
/// - Configuration cannot be loaded
/// - Project is not found in any configured root
/// - IDE cannot be launched
fn open(args: OpenArgs) -> Result<()> {
    let config = Config::load()?;

    for root in config.projects_root {
        let candidate = root.join(&args.project);

        if candidate.exists() {
            let ide = args.ide.unwrap_or(config.default_ide);

            launcher::launch(ide, &candidate)?;

            println!("{} {}", "Opened".green(), candidate.display());

            return Ok(());
        }
    }

    bail!("Project '{}' not found.", args.project)
}
