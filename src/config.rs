//! Configuration file management.
//!
//! Handles loading, saving, and managing the `config.toml` file which stores
//! user preferences like default IDE and project root directories.
//!
//! # File Format
//!
//! Configuration is stored in TOML format at platform-specific locations:
//! - **Windows:** `C:\Users\{user}\AppData\Roaming\dev-cli\config\config.toml`
//! - **macOS:** `~/Library/Application Support/dev-cli/config.toml`
//! - **Linux:** `~/.config/dev-cli/config.toml`
//!
//! # Example Config
//!
//! ```toml
//! projects_root = ["C:\\Users\\user\\Projects", "C:\\Users\\user\\Work"]
//! default_ide = "cursor"
//! ```
//!
//! # Usage
//!
//! ```no_run
//! # use anyhow::Result;
//! # fn example() -> Result<()> {
//! use dev_cli::config::Config;
//!
//! // Load configuration (creates with defaults if missing)
//! let config = Config::load()?;
//!
//! // Modify and save
//! let mut config = config;
//! config.default_ide = dev_cli::models::ide::Ide::Cursor;
//! config.save()?;
//! # Ok(())
//! # }
//! ```

use std::{fs, path::PathBuf};

use anyhow::{Context, Result};
use directories::{BaseDirs, ProjectDirs};
use serde::{Deserialize, Serialize};

use crate::models::ide::Ide;

/// User configuration for dev-cli.
///
/// Stores persistent settings like project roots and default IDE.
/// Automatically serializable to/from TOML format.
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Directories to search for Git repositories.
    ///
    /// Projects are discovered by searching for `.git` directories
    /// within these root paths.
    pub projects_root: Vec<PathBuf>,

    /// Default IDE to use when opening projects.
    ///
    /// Can be overridden on a per-command basis with `--ide` flag.
    pub default_ide: Ide,
}

impl Default for Config {
    /// Creates configuration with sensible defaults.
    ///
    /// Defaults:
    /// - `projects_root` — `~/Projects` directory
    /// - `default_ide` — VS Code
    ///
    /// # Panics
    ///
    /// Panics if home directory cannot be determined.
    fn default() -> Self {
        let home = BaseDirs::new().expect("Couldn't find home directory").home_dir().to_path_buf();

        Self { projects_root: vec![home.join("Projects")], default_ide: Ide::Vscode }
    }
}

impl Config {
    /// Get the path to the configuration file.
    ///
    /// Returns platform-specific path:
    /// - **Windows:** `C:\Users\{user}\AppData\Roaming\dev-cli\config\config.toml`
    /// - **macOS:** `~/Library/Application Support/dev-cli/config.toml`
    /// - **Linux:** `~/.config/dev-cli/config.toml`
    ///
    /// Returns error if platform directories cannot be located.
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
    ///
    /// If the configuration file doesn't exist, creates it with default values
    /// and returns the defaults.
    ///
    /// Returns error if:
    /// - Config directory cannot be located
    /// - File cannot be read (except if missing)
    /// - TOML parsing fails
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
    ///
    /// Creates config directory if it doesn't exist. Overwrites existing file.
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Config directory cannot be located or created
    /// - File cannot be written
    /// - Serialization to TOML fails
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
}
