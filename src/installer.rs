use std::{env, fs};

use anyhow::{Context, Result};
use directories::BaseDirs;

use crate::config::Config;

pub fn install() -> Result<()> {
    let exe = env::current_exe()?;

    let home = BaseDirs::new().unwrap().home_dir().to_path_buf();

    let bin = home.join(".local/bin");

    fs::create_dir_all(&bin)?;

    let destination = bin.join("dev.exe");

    fs::copy(&exe, &destination).context("Couldn't copy executable")?;

    Config::load()?;

    println!("✓ Installed to {}", destination.display());

    println!();
    println!("Add this directory to PATH if it isn't already:");
    println!("{}", bin.display());

    Ok(())
}
