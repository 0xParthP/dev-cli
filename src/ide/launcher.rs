use std::{path::Path, process::Command};

use anyhow::{Context, Result, bail};

use crate::{ide::detect::detect_ides, models::ide::Ide};

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
