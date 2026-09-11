//! First-run onboarding wizard.

use std::{io::IsTerminal, path::PathBuf};

use anyhow::Result;
use cliclack::{confirm, input, intro, outro, select};

use crate::{config::Config, models::ide::Ide};

/// Runs onboarding only if `config.toml` doesn't exist.
pub fn ensure_onboarded() -> Result<()> {
    if is_interactive_terminal() {
        run_onboarding_if_needed()?;
    }
    Ok(())
}

/// Returns `true` if both stdin and stdout are attached to a terminal.
fn is_interactive_terminal() -> bool {
    if std::env::var("DEVCLI_SKIP_ONBOARDING").is_ok() {
        return false;
    }
    std::io::stdin().is_terminal() && std::io::stdout().is_terminal()
}

/// Loads the config or runs the wizard, but only when attached to a TTY.
fn run_onboarding_if_needed() -> Result<()> {
    if Config::exists()? {
        return Ok(());
    }

    run_onboarding()
}

/// Interactive setup wizard.
#[cfg(not(coverage))]
pub fn run_onboarding() -> Result<()> {
    intro("🚀 Welcome to dev-cli")?;

    let default_projects = default_projects_dir();

    let projects_root: String =
        input("Where are your Git projects stored?").default_input(&default_projects).interact()?;

    let detected_ides = crate::ide::detect::detect_ides();

    let mut select_builder = select("Choose your default IDE");

    if detected_ides.is_empty() {
        select_builder = select_builder.item(Ide::Terminal, "Terminal", "System Terminal");
    } else {
        for installed in &detected_ides {
            let note = if installed.ide == Ide::Vscode { "Recommended" } else { "" };
            select_builder = select_builder.item(installed.ide, &installed.display_name, note);
        }
    }

    let default_ide = select_builder
        .initial_value(detected_ides.first().map(|i| i.ide).unwrap_or(Ide::Terminal))
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

    Config { projects_root: roots, default_ide, recent_projects: Vec::new() }.save()?;

    outro("✨ Setup complete! You're ready to use dev-cli.")?;

    Ok(())
}

/// Interactive setup wizard stub for coverage runs.
#[cfg(coverage)]
pub fn run_onboarding() -> Result<()> {
    Ok(())
}

/// Default `~/Projects` path.
pub fn default_projects_dir() -> String {
    match directories::BaseDirs::new() {
        Some(dirs) => dirs.home_dir().join("Projects").display().to_string(),
        None => String::from("~/Projects"),
    }
}
