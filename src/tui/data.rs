//! Loads dashboard data from config + scanner.

use anyhow::Result;

use crate::{config::Config, models::project::Project, scanner};

pub fn load_projects() -> Result<Vec<Project>> {
    let config = Config::load()?;

    // Scan the configured projects root.
    let projects: Vec<Project> = scanner::discover_projects(&config.projects_root)?;

    Ok(projects)
}
