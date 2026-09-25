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

use crate::{
    ide::detect::detect_ides,
    models::custom_ide::CustomIde,
    models::ide::{Ide, IdeSelection},
};

/// Launch an IDE selection (built-in or custom) to open a project.
pub fn launch_selection(
    selection: &IdeSelection,
    project: &Path,
    custom_ides: &[CustomIde],
) -> Result<()> {
    match selection {
        IdeSelection::BuiltIn(ide) => launch(*ide, project),
        IdeSelection::Custom(id) => {
            let custom = custom_ides
                .iter()
                .find(|c| c.id == *id)
                .with_context(|| format!("Custom IDE '{}' not found", id))?;
            launch_custom(custom, project)
        }
    }
}

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
pub fn launch(ide: Ide, project: &Path) -> Result<()> {
    let installed = detect_ides();

    let launcher = installed.iter().find(|i| i.ide == ide);

    let Some(launcher) = launcher else {
        bail!("{:?} is not installed.", ide);
    };

    launch_spawn(ide, project, &launcher.executable)
}

/// Helper used to spawn an IDE process with a specific executable path.
#[doc(hidden)]
pub fn launch_spawn(ide: Ide, project: &Path, executable: &Path) -> Result<()> {
    match ide {
        Ide::Claude | Ide::AntigravityCli => {
            Command::new(executable)
                .current_dir(project)
                .status()
                .context("Couldn't start CLI editor/agent")?;
        }
        Ide::Neovim | Ide::Helix => {
            Command::new(executable)
                .arg(project)
                .current_dir(project)
                .status()
                .context("Couldn't start terminal editor")?;
        }
        Ide::Terminal => {
            if cfg!(windows) {
                let exe_name =
                    executable.file_name().and_then(|n| n.to_str()).unwrap_or("").to_lowercase();
                if exe_name.contains("wt") {
                    Command::new(executable)
                        .arg("-d")
                        .arg(project)
                        .spawn()
                        .context("Couldn't open Windows Terminal")?;
                } else {
                    Command::new(executable)
                        .current_dir(project)
                        .spawn()
                        .context("Couldn't open Terminal")?;
                }
            } else if cfg!(target_os = "macos") {
                if executable.to_string_lossy().contains("open") {
                    Command::new(executable)
                        .arg("-a")
                        .arg("Terminal")
                        .arg(project)
                        .spawn()
                        .context("Couldn't open macOS Terminal")?;
                } else {
                    Command::new(executable)
                        .current_dir(project)
                        .spawn()
                        .context("Couldn't open Terminal shell")?;
                }
            } else {
                let exe_name =
                    executable.file_name().and_then(|n| n.to_str()).unwrap_or("").to_lowercase();
                let mut cmd = Command::new(executable);
                if exe_name.contains("gnome-terminal") {
                    cmd.arg(format!("--working-directory={}", project.display()));
                } else if exe_name.contains("konsole") {
                    cmd.arg("--workdir").arg(project);
                } else if exe_name.contains("alacritty")
                    || exe_name.contains("kitty")
                    || exe_name.contains("wezterm")
                {
                    cmd.arg("--working-directory").arg(project);
                } else if exe_name.contains("xterm") {
                    cmd.arg("-e").arg(format!("cd {}; $SHELL", project.display()));
                } else {
                    cmd.current_dir(project);
                }
                cmd.spawn().context("Couldn't open Linux Terminal")?;
            }
        }
        _ => {
            Command::new(executable).arg(project).spawn().context("Couldn't launch IDE")?;
        }
    }
    Ok(())
}

/// Launch a custom IDE to open a project.
///
/// Supports an optional argument template with `{path}` as a placeholder
/// for the project directory. When no template is provided, the project
/// path is passed as the sole argument.
pub fn launch_custom(custom: &CustomIde, project: &Path) -> Result<()> {
    let project_str = project.to_string_lossy();

    let mut cmd = Command::new(&custom.executable);

    if let Some(ref template) = custom.args_template {
        let args: Vec<String> =
            template.split_whitespace().map(|arg| arg.replace("{path}", &project_str)).collect();

        cmd.args(&args);
    } else {
        cmd.arg(project);
    }

    cmd.spawn().with_context(|| format!("Couldn't launch {}", custom.display_name))?;

    Ok(())
}
