//! Project management command implementation.
//!
//! Implements `dev project` subcommands for listing and opening projects.

use anyhow::{Result, bail};
use owo_colors::OwoColorize;

use crate::{
    cli::{OpenArgs, ProjectCommand, ProjectSubcommand},
    config::Config,
    ide::launcher,
    utils::path::display_path,
};

/// Execute a project command.
///
/// Dispatches to appropriate subcommand handler:
/// - `list` — List configured project roots.
/// - `open` — Open a project in an IDE.
///
/// # Errors
///
/// Returns an error if any command operation fails.
pub fn execute(cmd: ProjectCommand) -> Result<()> {
    match cmd.command {
        ProjectSubcommand::List => list(),
        ProjectSubcommand::Open(args) => open(args),
    }
}

/// Shorthand for opening a project.
///
/// Used by `dev open <PROJECT>`, which is equivalent to
/// `dev project open <PROJECT>`.
///
/// # Errors
///
/// Returns an error if opening fails.
pub fn open_shortcut(args: OpenArgs) -> Result<()> {
    open(args)
}

/// List all configured project root directories.
///
/// Displays the directories where dev-cli searches for projects.
///
/// **Note:** Repository discovery is implemented in `scanner.rs` during Sprint
/// 2.1, but CLI integration happens in Sprint 2.2.
///
/// # Errors
///
/// Returns an error if configuration cannot be loaded.
fn list() -> Result<()> {
    let config = Config::load()?;

    println!("{}", "Configured Project Roots".bold());

    for root in config.projects_root {
        println!("📁 {}", display_path(&root));
    }

    Ok(())
}

/// Open a project in an IDE.
///
/// Searches for the project in configured roots and launches it in the
/// specified IDE. If no IDE is specified, uses the configured default.
///
/// # Errors
///
/// Returns an error if:
/// - configuration cannot be loaded,
/// - the project is not found in any configured root,
/// - the IDE launcher fails.
fn open(args: OpenArgs) -> Result<()> {
    let config = Config::load()?;

    for root in config.projects_root {
        let candidate = root.join(&args.project);

        if candidate.exists() {
            let ide = args.ide.unwrap_or(config.default_ide);

            launcher::launch(ide, &candidate)?;

            println!("{} {}", "Opened".green(), display_path(&candidate));

            return Ok(());
        }
    }

    bail!("Project '{}' not found.", args.project)
}
