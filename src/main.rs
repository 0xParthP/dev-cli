//! # dev-cli
//!
//! A Windows-first CLI tool for managing Git repositories and launching them in IDEs.
//!
//! ## Quick Start
//!
//! ```bash
//! dev config init
//! dev project list
//! dev open MyProject
//! ```
//!
//! ## Architecture
//!
//! dev-cli follows a strict layered architecture with no upward dependencies:
//!
//! 1. **CLI Layer** — Parse command-line arguments (cli.rs, main.rs)
//! 2. **Commands Layer** — Dispatch to command handlers (commands/*.rs)
//! 3. **Services Layer** — Business logic and I/O (config.rs, ide/*, installer.rs)
//! 4. **Models Layer** — Data structures (models/*.rs)
//!
//! ## Modules
//!
//! - [`cli`] — CLI argument parsing
//! - [`commands`] — Command implementations
//! - [`config`] — Configuration file management
//! - [`ide`] — IDE detection and launching
//! - [`installer`] — Global installation
//! - [`models`] — Data structures
//! - [`scanner`] — Repository discovery (future)

mod cli;
mod commands;
mod config;
mod ide;
mod installer;
mod models;
mod scanner;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

/// Application entry point and command dispatcher.
///
/// Parses CLI arguments and dispatches to appropriate command handler.
///
/// # Errors
///
/// Returns error if any command fails or if tracing initialization fails.
fn main() -> Result<()> {
    tracing_subscriber::fmt().with_target(false).without_time().init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Project(cmd) => commands::project::execute(cmd)?,
        Commands::Config(cmd) => commands::config::execute(cmd)?,
        Commands::Ide(cmd) => commands::ide::execute(cmd)?,
        Commands::Install => commands::install::execute()?,
        Commands::Open(args) => commands::project::open_shortcut(args)?,
    }

    Ok(())
}
