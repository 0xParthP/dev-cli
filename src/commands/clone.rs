//! Clone command implementation.

use anyhow::{Result, anyhow};
use std::path::PathBuf;
use std::process::{Command, Stdio};

use crate::cli::CloneArgs;
use crate::config::Config;

/// Executes the clone command.
pub fn execute(args: CloneArgs) -> Result<()> {
    let cfg = Config::load()?;
    if cfg.projects_root.is_empty() {
        return Err(anyhow!("No project roots configured. Please add a root first."));
    }

    // Determine target root
    let target_root = match args.root {
        Some(ref root_arg) => {
            // Check if it's an index
            if let Ok(idx) = root_arg.parse::<usize>() {
                if idx < cfg.projects_root.len() {
                    cfg.projects_root[idx].clone()
                } else {
                    return Err(anyhow!(
                        "Root index {} is out of bounds. You have {} configured roots.",
                        idx,
                        cfg.projects_root.len()
                    ));
                }
            } else {
                // Otherwise treat as path string
                PathBuf::from(root_arg)
            }
        }
        None => cfg.projects_root[0].clone(),
    };

    if !target_root.exists() {
        std::fs::create_dir_all(&target_root)?;
    }

    // Extract repo name if not provided
    let repo_name = match args.name {
        Some(name) => name,
        None => {
            let path = std::path::Path::new(&args.url);
            let file_name = path.file_name().unwrap_or_default().to_string_lossy();
            let name = if file_name.is_empty() { "repository" } else { file_name.as_ref() };
            name.strip_suffix(".git").unwrap_or(name).to_string()
        }
    };

    let target_path = target_root.join(repo_name);

    println!("Cloning '{}' into '{}'...", args.url, target_path.display());

    let mut cmd = Command::new("git");
    cmd.arg("clone");
    cmd.arg(&args.url);
    cmd.arg(&target_path);

    // Stream output directly to terminal so user can interact if auth is needed
    cmd.stdout(Stdio::inherit());
    cmd.stderr(Stdio::inherit());
    cmd.stdin(Stdio::inherit());

    let status = cmd.status().map_err(|e| anyhow!("Failed to spawn git process: {}", e))?;

    if status.success() {
        println!("Successfully cloned into {}", target_path.display());
        Ok(())
    } else {
        Err(anyhow!("git clone failed with exit code: {}", status))
    }
}
