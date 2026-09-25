//! Configuration file management.

use std::{fs, path::PathBuf};

use anyhow::{Context, Result};
use directories::{BaseDirs, ProjectDirs};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::{
    custom_ide::CustomIde,
    ide::{Ide, IdeSelection},
    project::Project,
    recent_project::RecentProject,
};

fn default_max_depth() -> usize {
    4
}

fn default_ignore_patterns() -> Vec<String> {
    vec![
        "node_modules".to_string(),
        "target".to_string(),
        "vendor".to_string(),
        ".venv".to_string(),
        "venv".to_string(),
        "build".to_string(),
        "dist".to_string(),
    ]
}

fn default_theme() -> String {
    "neon".to_string()
}

fn default_recent_limit() -> usize {
    10
}

fn default_auto_refresh() -> bool {
    true
}

/// User configuration for dev-cli.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Directories to search for Git repositories.
    pub projects_root: Vec<PathBuf>,

    /// Default IDE used when opening projects.
    pub default_ide: IdeSelection,

    /// Recently opened projects.
    #[serde(default)]
    pub recent_projects: Vec<RecentProject>,

    /// User-configured custom IDEs.
    #[serde(default)]
    pub custom_ides: Vec<CustomIde>,

    /// Maximum search depth for repository scanner.
    #[serde(default = "default_max_depth")]
    pub max_depth: usize,

    /// Extra directory names to ignore during scanning.
    #[serde(default = "default_ignore_patterns")]
    pub ignore_patterns: Vec<String>,

    /// Active TUI theme palette name ("neon", "cyberpunk", "catppuccin", "monokai", "high-contrast").
    #[serde(default = "default_theme")]
    pub theme: String,

    /// Maximum number of recent projects to retain.
    #[serde(default = "default_recent_limit")]
    pub recent_projects_limit: usize,

    /// Automatically refresh project list when starting the TUI.
    #[serde(default = "default_auto_refresh")]
    pub auto_refresh_on_launch: bool,
}

impl Default for Config {
    /// Creates configuration with sensible defaults.
    fn default() -> Self {
        let home = BaseDirs::new()
            .map(|b| b.home_dir().to_path_buf())
            .unwrap_or_else(|| PathBuf::from("."));

        Self {
            projects_root: vec![home.join("Projects")],
            default_ide: IdeSelection::BuiltIn(Ide::Vscode),
            recent_projects: Vec::new(),
            custom_ides: Vec::new(),
            max_depth: default_max_depth(),
            ignore_patterns: default_ignore_patterns(),
            theme: default_theme(),
            recent_projects_limit: default_recent_limit(),
            auto_refresh_on_launch: default_auto_refresh(),
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
        self.add_recent_project_with_ide(project, self.default_ide.clone());
    }

    pub fn add_recent_project_with_ide(&mut self, project: &Project, ide: IdeSelection) {
        self.recent_projects.retain(|p| p.path != project.path);

        self.recent_projects.insert(
            0,
            RecentProject {
                name: project.name.clone(),
                path: project.path.clone(),
                last_opened: Self::now_timestamp(),
                ide: Some(ide),
            },
        );

        self.recent_projects.truncate(10);
    }

    /// Add a custom IDE to the configuration.
    pub fn add_custom_ide(&mut self, ide: CustomIde) {
        self.custom_ides.retain(|i| i.id != ide.id);
        self.custom_ides.push(ide);
        self.custom_ides.sort_by_key(|a| a.display_name.to_lowercase());
    }

    /// Remove a custom IDE by its identifier.
    pub fn remove_custom_ide(&mut self, id: &str) {
        self.custom_ides.retain(|i| i.id != id);
    }

    /// Look up a custom IDE by its identifier.
    pub fn get_custom_ide(&self, id: &str) -> Option<&CustomIde> {
        self.custom_ides.iter().find(|i| i.id == id)
    }

    /// Add a project root directory to the configuration.
    pub fn add_project_root(&mut self, root: PathBuf) {
        if !self.projects_root.contains(&root) {
            self.projects_root.push(root);
        }
    }

    /// Remove a project root directory by index.
    pub fn remove_project_root(&mut self, index: usize) {
        if index < self.projects_root.len() {
            self.projects_root.remove(index);
        }
    }
}
