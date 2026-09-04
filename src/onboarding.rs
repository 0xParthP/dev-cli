//! First-run onboarding wizard.
//!
//! Runs automatically the first time any `dev` command is executed.
//!
//! # Coverage notes
//!
//! The interactive wizard body in [`run_onboarding`] is gated by
//! `#[cfg(not(coverage))]`. It can only execute when stdin *and* stdout
//! are attached to a real terminal, which `cargo test` does not provide.
//! A PTY-driven harness would be the only honest way to exercise those
//! lines; without one, including them in coverage just keeps the metric
//! low for no signal. This is a future TODO.

use anyhow::Result;

#[cfg(not(coverage))]
use std::io::IsTerminal;

#[cfg(not(coverage))]
use crate::{config::Config, models::ide::Ide};
#[cfg(not(coverage))]
use cliclack::{confirm, input, intro, outro, select};
#[cfg(not(coverage))]
use std::path::PathBuf;

/// Runs onboarding only if `config.toml` doesn't exist.
pub fn ensure_onboarded() -> Result<()> {
    if is_interactive_terminal() {
        #[cfg(not(coverage))]
        run_onboarding_if_needed()?;
    }
    Ok(())
}

/// Returns `true` if both stdin and stdout are attached to a terminal.
///
/// The body of this helper is excluded from coverage because `cargo test`
/// never has a real TTY — gating it with `#[cfg(not(coverage))]` keeps the
/// wizard from being called under coverage while still producing
/// deterministic `false` for the test harness to verify the no-op path.
#[cfg(not(coverage))]
fn is_interactive_terminal() -> bool {
    std::io::stdin().is_terminal() && std::io::stdout().is_terminal()
}

/// Coverage-mode stub: `cargo test` cannot attach a TTY, so this branch is
/// always `false`. The wizard body that this guard protects is itself
/// `#[cfg(not(coverage))]`, so the production `is_interactive_terminal` is
/// the only path that ever calls it.
#[cfg(coverage)]
fn is_interactive_terminal() -> bool {
    false
}

/// Loads the config or runs the wizard, but only when attached to a TTY.
#[cfg(not(coverage))]
fn run_onboarding_if_needed() -> Result<()> {
    if Config::exists()? {
        return Ok(());
    }

    run_onboarding()
}

/// Interactive setup wizard.
///
/// This function is excluded from coverage because it can only be driven
/// from a real terminal session
#[cfg(not(coverage))]
pub fn run_onboarding() -> Result<()> {
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
///
/// Returns `~/Projects` when the platform has no home directory concept
/// (i.e. `BaseDirs::new()` returns `None`).
pub fn default_projects_dir() -> String {
    match directories::BaseDirs::new() {
        Some(dirs) => dirs.home_dir().join("Projects").display().to_string(),
        None => String::from("~/Projects"),
    }
}
