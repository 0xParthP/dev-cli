//! Actions performed from the TUI.

use std::path::Path;

use anyhow::Result;

use crate::{
    config::Config,
    ide::launcher,
    models::{ide::Ide, project::Project},
};

/// Open a project using the real launcher.
pub fn open_project(project: &Project) -> Result<()> {
    open_project_with(project, launcher::launch)
}

/// Testable version that accepts an injected launcher.
pub fn open_project_with<F>(project: &Project, mut launch: F) -> Result<()>
where
    F: FnMut(Ide, &Path) -> Result<()>,
{
    let config = Config::load()?;
    launch(config.default_ide, &project.path)
}
