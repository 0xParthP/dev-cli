//! Binary entrypoint for `dev-cli`.
//!
//! This binary is intentionally thin.
//! All application logic lives in the `dev_cli` library crate.
//!
//! ## Responsibilities
//!
//! - Initialise logging.
//! - Parse CLI arguments using `clap`.
//! - Dispatch commands to the library crate.

use anyhow::Result;
use clap::Parser;

use dev_cli::{
    cli::{Cli, Commands},
    commands, onboarding, tui,
};

fn main() -> Result<()> {
    tracing_subscriber::fmt().with_target(false).without_time().init();

    let cli = Cli::parse();

    onboarding::ensure_onboarded()?;

    match cli.command {
        Some(Commands::Project(cmd)) => commands::project::execute(cmd)?,
        Some(Commands::Config(cmd)) => commands::config::execute(cmd)?,
        Some(Commands::Ide(cmd)) => commands::ide::execute(cmd)?,
        Some(Commands::Open(args)) => commands::project::open_shortcut(args)?,

        None => tui::run()?,
    }

    Ok(())
}
