//! Configuration file management.

use std::{fs, path::PathBuf};

use anyhow::{Context, Result};
use directories::{BaseDirs, ProjectDirs};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::{ide::Ide, project::Project, recent_project::RecentProject};

/// User configuration for dev-cli.
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Directories to search for Git repositories.
    pub projects_root: Vec<PathBuf>,

    /// Default IDE used when opening projects.
    pub default_ide: Ide,

    /// Recently opened projects.
    #[serde(default)]
    pub recent_projects: Vec<RecentProject>,
}

impl Default for Config {
    /// Creates configuration with sensible defaults.
    fn default() -> Self {
        let home = BaseDirs::new()
            .map(|b| b.home_dir().to_path_buf())
            .unwrap_or_else(|| PathBuf::from("."));

        Self {
            projects_root: vec![home.join("Projects")],
            default_ide: Ide::Vscode,
            recent_projects: Vec::new(),
        }
    }
}

impl Config {
    /// Get the path to the configuration file.
    pub fn path() -> Result<PathBuf> {
        // Check for test override first.
        // `DEVCLI_CONFIG_DIR` lets integration tests point the config at a
        // temporary directory instead of the real platform config location.
        if let Ok(test_dir) = std::env::var("DEVCLI_CONFIG_DIR") {
            return Ok(PathBuf::from(test_dir).join("config.toml"));
        }

        let proj =
            ProjectDirs::from("", "", "dev-cli").context("Couldn't locate config directory")?;

        Ok(proj.config_dir().join("config.toml"))
    }

    /// Load configuration from file.
    pub fn load() -> Result<Self> {
        let path = Self::path()?;

        // First run: create default config.
        if !path.exists() {
            let config = Self::default();
            config.save()?;
            return Ok(config);
        }

        let text = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config file at {}", path.display()))?;

        match toml::from_str::<Self>(&text) {
            Ok(config) => Ok(config),

            Err(error) => {
                eprintln!(
                    "Config at {} is invalid ({}). Recreating defaults.",
                    path.display(),
                    error
                );

                let config = Self::default();

                // Explicitly overwrite the corrupted file.
                fs::write(&path, toml::to_string_pretty(&config)?)?;

                Ok(config)
            }
        }
    }

    /// Save configuration to file.
    pub fn save(&self) -> Result<()> {
        let path = Self::path()?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(path, toml::to_string_pretty(self)?)?;

        Ok(())
    }

    /// Returns `true` if the configuration file already exists.
    pub fn exists() -> Result<bool> {
        Ok(Self::path()?.exists())
    }

    /// Creates and saves a new configuration.
    pub fn create(config: Self) -> Result<Self> {
        config.save()?;
        Ok(config)
    }

    fn now_timestamp() -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs()
    }

    pub fn add_recent_project(&mut self, project: &Project) {
        self.recent_projects.retain(|p| p.path != project.path);

        self.recent_projects.insert(
            0,
            RecentProject {
                name: project.name.clone(),
                path: project.path.clone(),
                last_opened: Self::now_timestamp(),
            },
        );

        self.recent_projects.truncate(10);
    }
}
