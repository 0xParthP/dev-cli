//! Actions performed from the TUI.

use anyhow::Result;
use std::path::Path;

use crate::{
    config::Config,
    ide::launcher,
    models::{ide::Ide, project::Project},
};

pub fn open_path(path: &Path) -> anyhow::Result<()> {
    let project = Project::new(path.to_path_buf(), path.to_path_buf());
    open_project(&project)
}

/// Open a project using the real launcher.
pub fn open_project(project: &Project) -> Result<()> {
    open_project_with(project, launcher::launch)
}

/// Testable version that accepts an injected launcher.
pub fn open_project_with<F>(project: &Project, mut launch: F) -> Result<()>
where
    F: FnMut(Ide, &Path) -> Result<()>,
{
    let mut config = Config::load()?;
    launch(config.default_ide, &project.path)?;
    config.add_recent_project(project);
    config.save()?;
    Ok(())
}
