//! First-run onboarding wizard.
//!
//! Runs automatically the first time any `dev` command is executed.

use std::path::PathBuf;

use anyhow::Result;
use cliclack::{confirm, input, intro, outro, select};

use dev_cli::{config::Config, models::ide::Ide};

/// Runs onboarding only if config.toml doesn't exist.
pub fn ensure_onboarded() -> Result<()> {
    // Skip onboarding during automated tests.
    if std::env::var("DEVCLI_SKIP_ONBOARDING").is_ok() {
        return Ok(());
    }

    if Config::exists()? {
        return Ok(());
    }

    run_onboarding()
}

/// Interactive setup wizard.
fn run_onboarding() -> Result<()> {
    intro("🚀 Welcome to dev-cli")?;

    let default_projects = default_projects_dir();

    let projects_root: String =
        input("Where are your Git projects stored?").default_input(&default_projects).interact()?;

    let default_ide = select("Choose your default IDE")
        .item(Ide::Vscode, "VS Code", "Recommended")
        .item(Ide::Cursor, "Cursor", "")
        .item(Ide::Claude, "Claude Code", "")
        .item(Ide::Terminal, "Terminal", "")
        .interact()?;

    let mut roots = vec![PathBuf::from(projects_root)];

    loop {
        let add_another =
            confirm("Add another projects directory root?").initial_value(false).interact()?;

        if !add_another {
            break;
        }

        let next_root: String = input("Projects directory root").interact()?;
        roots.push(PathBuf::from(next_root));
    }

    Config { projects_root: roots, default_ide }.save()?;

    outro("✨ Setup complete! You're ready to use dev-cli.")?;

    Ok(())
}

/// Default `~/Projects` path.
fn default_projects_dir() -> String {
    directories::BaseDirs::new()
        .map(|dirs| dirs.home_dir().join("Projects").display().to_string())
        .unwrap_or_else(|| String::from("~/Projects"))
}
