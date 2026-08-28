use std::{fs, path::PathBuf};

use anyhow::{Context, Result};
use directories::{BaseDirs, ProjectDirs};
use serde::{Deserialize, Serialize};

use crate::models::ide::Ide;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub projects_root: Vec<PathBuf>,
    pub default_ide: Ide,
}

impl Default for Config {
    fn default() -> Self {
        let home = BaseDirs::new().expect("Couldn't find home directory").home_dir().to_path_buf();

        Self { projects_root: vec![home.join("Projects")], default_ide: Ide::Vscode }
    }
}

impl Config {
    pub fn path() -> Result<PathBuf> {
        let proj =
            ProjectDirs::from("", "", "dev-cli").context("Couldn't locate config directory")?;

        Ok(proj.config_dir().join("config.toml"))
    }

    pub fn load() -> Result<Self> {
        let path = Self::path()?;

        if !path.exists() {
            let config = Self::default();
            config.save()?;
            return Ok(config);
        }

        let text = fs::read_to_string(path)?;
        Ok(toml::from_str(&text)?)
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::path()?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(path, toml::to_string_pretty(self)?)?;

        Ok(())
    }
}
