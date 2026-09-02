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
    // Test override: bypass IDE detection entirely.
    if let Ok(test_executable) = std::env::var("DEVCLI_TEST_EXECUTABLE") {
        return launch_spawn(ide, project, Path::new(&test_executable));
    }

    // Normal application path.
    let installed = detect_ides();

    let launcher = installed.iter().find(|i| i.ide == ide);

    let Some(launcher) = launcher else {
        bail!("{:?} is not installed.", ide);
    };

    launch_spawn(ide, project, &launcher.executable)
}

/// Helper used to spawn an IDE process with a specific executable path.
///
/// Exists to share the per-IDE CLI semantics (Claude uses `current_dir`,
/// Terminal uses `-d`, others pass the project as a positional arg) between
/// the test override path and the normal application path, so both are
/// covered by the same test suite.
#[doc(hidden)]
pub fn launch_spawn(ide: Ide, project: &Path, executable: &Path) -> Result<()> {
    match ide {
        Ide::Claude => {
            Command::new(executable)
                .current_dir(project)
                .spawn()
                .context("Couldn't start Claude Code")?;
        }
        Ide::Terminal => {
            Command::new(executable)
                .arg("-d")
                .arg(project)
                .spawn()
                .context("Couldn't open Windows Terminal")?;
        }
        _ => {
            Command::new(executable).arg(project).spawn().context("Couldn't launch IDE")?;
        }
    }
    Ok(())
}
