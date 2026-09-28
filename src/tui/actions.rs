//! Actions performed from the TUI.

use anyhow::Result;
use std::path::Path;

use crate::{
    config::Config,
    ide::launcher,
    models::{ide::IdeSelection, project::Project},
};

pub fn open_path(path: &Path) -> anyhow::Result<()> {
    let project = Project::new(path.to_path_buf(), path.to_path_buf());
    open_project(&project)
}

/// Open a project using the real launcher and default IDE.
pub fn open_project(project: &Project) -> Result<()> {
    let config = Config::load()?;
    open_project_with_ide(project, config.default_ide.clone(), |selection, path| {
        launcher::launch_selection(&selection, path, &config.custom_ides)
    })
}

/// Open a project with a specific target IDE using testable injected launcher.
pub fn open_project_with_ide<F>(project: &Project, ide: IdeSelection, mut launch: F) -> Result<()>
where
    F: FnMut(IdeSelection, &Path) -> Result<()>,
{
    let mut config = Config::load()?;
    launch(ide.clone(), &project.path)?;
    config.add_recent_project_with_ide(project, ide);
    config.save()?;
    Ok(())
}

/// Testable version that accepts an injected launcher.
pub fn open_project_with<F>(project: &Project, launch: F) -> Result<()>
where
    F: FnMut(IdeSelection, &Path) -> Result<()>,
{
    let config = Config::load()?;
    open_project_with_ide(project, config.default_ide, launch)
}
