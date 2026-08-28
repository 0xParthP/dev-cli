use anyhow::Result;
use owo_colors::OwoColorize;

use crate::{
    cli::{IdeCommand, IdeSubcommand},
    ide::detect::detect_ides,
};

pub fn execute(cmd: IdeCommand) -> Result<()> {
    match cmd.command {
        IdeSubcommand::List => list(),
    }
}

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
