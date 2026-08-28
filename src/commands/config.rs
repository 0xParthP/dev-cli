//! Configuration command implementation.
//!
//! Implements `dev config` subcommands for managing user configuration.

use anyhow::Result;
use owo_colors::OwoColorize;

use crate::{
    cli::{ConfigCommand, ConfigSubcommand},
    config::Config,
};

/// Execute a configuration command.
///
/// Dispatches to appropriate subcommand handler:
/// - `init` — Initialize configuration
/// - `show` — Display configuration
/// - `set-default-ide` — Set default IDE
///
/// # Errors
///
/// Returns error if any command operation fails.
pub fn execute(cmd: ConfigCommand) -> Result<()> {
    match cmd.command {
        ConfigSubcommand::Init => init(),
        ConfigSubcommand::Show => show(),
        ConfigSubcommand::SetDefaultIde { ide } => {
            let mut config = Config::load()?;
            config.default_ide = ide;
            config.save()?;

            println!("{}", "✔ Default IDE updated".green());

            Ok(())
        }
    }
}

/// Initialize configuration file with defaults.
///
/// Creates config file at platform-specific location with sensible defaults.
/// If file already exists, overwrites it.
///
/// # Errors
///
/// Returns error if config cannot be saved.
fn init() -> Result<()> {
    let config = Config::default();
    config.save()?;

    println!(
        "{} {}",
        "✔ Config created at".green(),
        Config::path()?.display()
    );

    Ok(())
}

/// Display current configuration.
///
/// Loads and prints configuration using debug formatting.
///
/// # Errors
///
/// Returns error if config cannot be loaded.
fn show() -> Result<()> {
    let config = Config::load()?;

    println!("{:#?}", config);

    Ok(())
}
