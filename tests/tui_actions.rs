use anyhow::{Result, anyhow};
use temp_env::with_var;
use tempfile::TempDir;

use dev_cli::{
    config::Config,
    models::{ide::Ide, project::Project},
    tui::actions::open_project_with,
};

fn with_temp_config<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let dir = TempDir::new().unwrap();
    with_var("DEVCLI_CONFIG_DIR", Some(dir.path()), f)
}

fn project(name: &str) -> Project {
    let root = std::env::temp_dir();
    let path = root.join(name);

    std::fs::create_dir_all(path.join(".git")).unwrap();

    Project { name: name.into(), path: path.clone(), root, git_dir: path.join(".git") }
}

#[test]
fn open_project_uses_default_ide() -> Result<()> {
    with_temp_config(|| -> Result<()> {
        Config { projects_root: vec![], default_ide: Ide::Vscode }.save()?;

        let project = project("demo");

        let mut called = false;

        open_project_with(&project, |ide, path| {
            called = true;
            assert_eq!(ide, Ide::Vscode);
            assert_eq!(path, &project.path);
            Ok(())
        })?;

        assert!(called);
        Ok(())
    })
}

#[test]
fn open_project_propagates_launcher_error() -> Result<()> {
    with_temp_config(|| -> Result<()> {
        Config { projects_root: vec![], default_ide: Ide::Vscode }.save()?;

        let project = project("demo");

        let result = open_project_with(&project, |_, _| Err(anyhow!("launch failed")));

        assert!(result.is_err());
        Ok(())
    })
}
