//! Project management command implementation.

use anyhow::{Result, bail};
use owo_colors::OwoColorize;

use crate::{
    cli::{OpenArgs, ProjectCommand, ProjectSubcommand},
    config::Config,
    ide::launcher,
    scanner,
};

/// Execute a project command.
pub fn execute(cmd: ProjectCommand) -> Result<()> {
    match cmd.command {
        ProjectSubcommand::List => list_projects(),
        ProjectSubcommand::Open(args) => open(args),
    }
}

/// `dev open <project>` shortcut.
pub fn open_shortcut(args: OpenArgs) -> Result<()> {
    open(args)
}

/// List configured roots and discovered Git repositories.
fn list_projects() -> Result<()> {
    let config = Config::load()?;

    println!("{}", "Configured Project Roots".bold());

    for root in &config.projects_root {
        println!("📁 {}", root.display());
    }

    println!();
    println!("{}", "Discovered Git Repositories".bold());

    let projects = scanner::discover_projects(&config.projects_root)?;

    if projects.is_empty() {
        println!("No repositories found.");
    } else {
        for project in projects {
            println!("• {} ({})", project.name, project.path.display());
        }
    }

    Ok(())
}

/// Open a project in an IDE.
fn open(args: OpenArgs) -> Result<()> {
    let config = Config::load()?;

    let projects = scanner::discover_projects(&config.projects_root)?;

    let Some(project) = projects.into_iter().find(|p| p.name == args.project) else {
        bail!("Project '{}' not found.", args.project);
    };

    let ide = args.ide.unwrap_or(config.default_ide);

    launcher::launch(ide, &project.path)?;

    println!("{} {}", "Opened".green(), project.path.display());

    Ok(())
}
