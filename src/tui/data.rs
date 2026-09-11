//! Loads dashboard data from config + scanner.

use anyhow::Result;

use crate::{config::Config, models::project::Project, scanner};

/// Discover and load all projects based on configuration.
pub fn load_projects() -> Result<Vec<Project>> {
    let config = Config::load()?;

    // Scan the configured projects root.
    let mut projects: Vec<Project> = scanner::discover_projects(&config.projects_root)?;
    projects.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(projects)
}
