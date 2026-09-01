//! Global installation logic.
//!
//! Handles installing dev-cli to ~/.local/bin for global access.

use std::{
    env, fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use directories::BaseDirs;

use crate::config::Config;

/// Returns the directory where dev-cli installs itself.
///
/// Resolution order:
/// 1. `DEVCLI_INSTALL_DIR` environment variable (tests/CI)
/// 2. `~/.local/bin`
pub fn binary_install_dir() -> Result<PathBuf> {
    if let Some(dir) = env::var_os("DEVCLI_INSTALL_DIR") {
        return Ok(PathBuf::from(dir));
    }

    let base = BaseDirs::new().ok_or_else(|| anyhow::anyhow!("Could not locate home directory"))?;

    Ok(base.home_dir().join(".local/bin"))
}

/// Returns the destination executable path.
fn destination_path(bin_dir: &Path) -> PathBuf {
    if cfg!(windows) { bin_dir.join("dev.exe") } else { bin_dir.join("dev") }
}

/// Install dev-cli globally.
///
/// Copies the current executable into the install directory and ensures the
/// configuration file exists.
pub fn install() -> Result<()> {
    let exe = env::current_exe().context("Couldn't locate current executable")?;

    let bin = binary_install_dir()?;
    fs::create_dir_all(&bin).context("Couldn't create install directory")?;

    let destination = destination_path(&bin);

    // Replace any existing installation.
    if destination.exists() {
        fs::remove_file(&destination).context("Couldn't remove existing installation")?;
    }

    fs::copy(&exe, &destination).context("Couldn't copy executable")?;

    // Ensure config exists.
    Config::load()?;

    println!("✓ Installed to {}", destination.display());
    println!();
    println!("Add this directory to PATH if it isn't already:");
    println!("{}", bin.display());

    Ok(())
}
