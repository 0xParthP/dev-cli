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

/// Open a project using the real launcher and default IDE.
pub fn open_project(project: &Project) -> Result<()> {
    open_project_with(project, launcher::launch)
}

/// Open a project with a specific target IDE using testable injected launcher.
pub fn open_project_with_ide<F>(project: &Project, ide: Ide, mut launch: F) -> Result<()>
where
    F: FnMut(Ide, &Path) -> Result<()>,
{
    let mut config = Config::load()?;
    launch(ide, &project.path)?;
    config.add_recent_project(project);
    config.save()?;
    Ok(())
}

/// Testable version that accepts an injected launcher.
pub fn open_project_with<F>(project: &Project, launch: F) -> Result<()>
where
    F: FnMut(Ide, &Path) -> Result<()>,
{
    let config = Config::load()?;
    open_project_with_ide(project, config.default_ide, launch)
}
