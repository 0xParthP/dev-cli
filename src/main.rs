mod cli;
mod commands;
mod config;
mod installer;
mod scanner;
mod ide;
mod models;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .without_time()
        .init();

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