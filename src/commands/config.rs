use anyhow::Result;
use owo_colors::OwoColorize;

use crate::{
    cli::{ConfigCommand, ConfigSubcommand},
    config::Config,
};

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

fn show() -> Result<()> {
    let config = Config::load()?;

    println!("{:#?}", config);

    Ok(())
}