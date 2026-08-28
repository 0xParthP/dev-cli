//! Global installation logic.
//!
//! Handles installing dev-cli to ~/.local/bin for global access.

use std::{env, fs};

use anyhow::{Context, Result};
use directories::BaseDirs;

use crate::config::Config;

/// Install dev-cli globally to ~/.local/bin.
///
/// Copies the current executable to `~/.local/bin/dev.exe` and ensures
/// configuration file is initialized.
///
/// # Process
///
/// 1. Determine current executable path
/// 2. Create ~/.local/bin directory if needed
/// 3. Copy executable to ~/.local/bin/dev.exe
/// 4. Initialize configuration with defaults
/// 5. Print installation location and PATH instructions
///
/// # Errors
///
/// Returns error if:
/// - Current executable cannot be determined
/// - ~/.local/bin cannot be created
/// - Executable cannot be copied
/// - Configuration cannot be initialized
///
/// # Platform Notes
///
/// Currently Windows-focused. Uses ~/.local/bin to follow standard practice
/// for portable installations.
pub fn install() -> Result<()> {
    let exe = env::current_exe()?;

    let home = BaseDirs::new()
        .unwrap()
        .home_dir()
        .to_path_buf();

    let bin = home.join(".local/bin");

    fs::create_dir_all(&bin)?;

    let destination = bin.join("dev.exe");

    fs::copy(&exe, &destination)
        .context("Couldn't copy executable")?;

    Config::load()?;

    println!("✓ Installed to {}", destination.display());

    println!();
    println!("Add this directory to PATH if it isn't already:");
    println!("{}", bin.display());

    Ok(())
}
