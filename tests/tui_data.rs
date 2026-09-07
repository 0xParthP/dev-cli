use anyhow::Result;
use dev_cli::{config::Config, models::ide::Ide, tui::data::load_projects};
use serial_test::serial;
use temp_env::with_var;
use tempfile::TempDir;

fn with_temp_config_dir<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let dir = TempDir::new().unwrap();

    with_var("DEVCLI_CONFIG_DIR", Some(dir.path()), f)
}

#[test]
#[serial]
fn load_projects_discovers_git_repository() -> Result<()> {
    with_temp_config_dir(|| {
        let root = TempDir::new()?;
        let repo = root.path().join("my-project");

        std::fs::create_dir_all(repo.join(".git"))?;

        let config = Config {
            projects_root: vec![root.path().to_path_buf()],
            default_ide: Ide::Vscode,
            recent_projects: Vec::new(),
        };

        config.save()?;

        let projects = load_projects()?;

        assert_eq!(projects.len(), 1);
        assert_eq!(projects[0].name, "my-project");

        Ok(())
    })
}

#[test]
#[serial]
fn load_projects_returns_empty_when_root_has_no_git_repos() -> Result<()> {
    with_temp_config_dir(|| {
        let root = TempDir::new()?;

        let config = Config {
            projects_root: vec![root.path().to_path_buf()],
            default_ide: Ide::Vscode,
            recent_projects: Vec::new(),
        };

        config.save()?;

        let projects = load_projects()?;

        assert!(projects.is_empty());

        Ok(())
    })
}
