//! IDE launching and process spawning.
//!
//! Spawns external IDE processes to open projects.
//!
//! # How It Works
//!
//! 1. Detects the IDE executable path
//! 2. Spawns external process with project path
//! 3. Returns immediately (doesn't wait for IDE to close)
//!
//! # IDE-Specific Behavior
//!
//! Different IDEs have different CLI interfaces:
//! - **VS Code & Cursor:** `code /path/to/project`
//! - **Claude Code:** Runs in project directory
//! - **Windows Terminal:** `wt -d /path/to/project`

use std::{path::Path, process::Command};

use anyhow::{Context, Result, bail};

use crate::{ide::detect::detect_ides, models::ide::Ide};

/// Launch an IDE to open a project.
///
/// Finds the IDE executable and spawns it with the project path.
/// Does not wait for the IDE to close; returns immediately after spawn.
///
/// # Arguments
///
/// * `ide` — Which IDE to launch
/// * `project` — Path to project directory
///
/// # Errors
///
/// Returns error if:
/// - IDE is not installed/detected
/// - Process cannot be spawned
/// - Process spawn fails
///
/// # Example
///
/// ```no_run
/// # use anyhow::Result;
/// # fn example() -> Result<()> {
/// use dev_cli::ide::launcher;
/// use dev_cli::models::ide::Ide;
/// use std::path::Path;
///
/// launcher::launch(Ide::Cursor, Path::new("./my-project"))?;
/// # Ok(())
/// # }
/// ```
pub fn launch(ide: Ide, project: &Path) -> Result<()> {
    let installed = detect_ides();

    let launcher = installed.iter().find(|i| i.ide == ide);

    let Some(launcher) = launcher else {
        bail!("{:?} is not installed.", ide);
    };

    match ide {
        Ide::Claude => {
            Command::new(&launcher.executable)
                .current_dir(project)
                .spawn()
                .context("Couldn't start Claude Code")?;
        }

        Ide::Terminal => {
            Command::new(&launcher.executable)
                .arg("-d")
                .arg(project)
                .spawn()
                .context("Couldn't open Windows Terminal")?;
        }

        _ => {
            Command::new(&launcher.executable)
                .arg(project)
                .spawn()
                .with_context(|| format!("Couldn't launch {}", launcher.display_name))?;
        }
    }

    Ok(())
}
