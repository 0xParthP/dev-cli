use anyhow::{bail, Result};
use owo_colors::OwoColorize;

use crate::{
    cli::{OpenArgs, ProjectCommand, ProjectSubcommand},
    config::Config,
    ide::launcher,
};

pub fn execute(cmd: ProjectCommand) -> Result<()> {
    match cmd.command {
        ProjectSubcommand::List => list(),
        ProjectSubcommand::Open(args) => open(args),
    }
}

pub fn open_shortcut(args: OpenArgs) -> Result<()> {
    open(args)
}

fn list() -> Result<()> {
    let config = Config::load()?;

    println!("{}", "Configured Project Roots".bold());

    for root in config.projects_root {
        println!("📁 {}", root.display());
    }

    Ok(())
}

fn open(args: OpenArgs) -> Result<()> {
    let config = Config::load()?;

    for root in config.projects_root {
        let candidate = root.join(&args.project);

        if candidate.exists() {
            let ide = args.ide.unwrap_or(config.default_ide);

            launcher::launch(ide, &candidate)?;

            println!(
                "{} {}",
                "Opened".green(),
                candidate.display()
            );

            return Ok(());
        }
    }

    bail!("Project '{}' not found.", args.project)
}