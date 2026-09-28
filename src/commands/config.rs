//! Configuration command implementation.

use anyhow::Result;
use owo_colors::OwoColorize;

use crate::{
    cli::{ConfigCommand, ConfigSubcommand},
    config::Config,
};

use crate::models::ide::IdeSelection;

/// Execute a configuration command.
pub fn execute(cmd: ConfigCommand) -> Result<()> {
    match cmd.command {
        ConfigSubcommand::Init => init(),
        ConfigSubcommand::Show => show(),
        ConfigSubcommand::SetDefaultIde { ide } => {
            let mut config = Config::load()?;
            match &ide {
                IdeSelection::BuiltIn(b) => {
                    let detected = crate::ide::detect::detect_ides();
                    if !detected.iter().any(|i| i.ide == *b) {
                        anyhow::bail!("IDE '{:?}' is not installed on your system.", b);
                    }
                }
                IdeSelection::Custom(id) => {
                    if !config.custom_ides.iter().any(|c| c.id == *id) {
                        anyhow::bail!("Custom IDE '{}' is not configured.", id);
                    }
                }
            }

            config.default_ide = ide;
            config.save()?;

            println!("{}", "✔ Default IDE updated".green());

            Ok(())
        }
    }
}

/// Initialize configuration file with defaults.
fn init() -> Result<()> {
    let config = Config::default();
    config.save()?;

    println!("{} {}", "✔ Config created at".green(), Config::path()?.display());

    Ok(())
}

/// Display current configuration.
fn show() -> Result<()> {
    let config = Config::load()?;

    println!("{:#?}", config);

    Ok(())
}
