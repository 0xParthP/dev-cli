use std::env;

use anyhow::Result;
use dev_cli::{config::Config, models::ide::Ide, tui::data::load_projects};
use serial_test::serial;
use tempfile::TempDir;

fn setup_config_dir() -> TempDir {
    let dir = TempDir::new().unwrap();

    #[cfg(windows)]
    unsafe {
        env::set_var("APPDATA", dir.path());
    }

    #[cfg(not(windows))]
    unsafe {
        env::set_var("XDG_CONFIG_HOME", dir.path());
    }

    dir
}

#[test]
#[serial]
fn load_projects_discovers_git_repository() -> Result<()> {
    let _config_dir = setup_config_dir();

    let root = TempDir::new()?;
    let repo = root.path().join("my-project");

    std::fs::create_dir_all(repo.join(".git"))?;

    let config =
        Config { projects_root: vec![root.path().to_path_buf()], default_ide: Ide::Cursor };

    config.save()?;

    let projects = load_projects()?;

    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].name, "my-project");

    Ok(())
}

#[test]
#[serial]
fn load_projects_returns_empty_when_root_has_no_git_repos() -> Result<()> {
    let _config_dir = setup_config_dir();

    let root = TempDir::new()?;

    let config =
        Config { projects_root: vec![root.path().to_path_buf()], default_ide: Ide::Cursor };

    config.save()?;

    let projects = load_projects()?;

    assert!(projects.is_empty());

    Ok(())
}
