use dev_cli::models::recent_project::RecentProject;
use dev_cli::{config::Config, models::ide::Ide};
use serial_test::serial;
use std::{
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};
use temp_env::with_var;
use tempfile::TempDir;

mod common;
use crate::common::temp_project::TempProject;
use anyhow::Result;
use common::factories::fake_project as project;
use common::temp_config::test_config;

fn with_temp_config<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let dir = TempDir::new().unwrap();
    with_var("DEVCLI_CONFIG_DIR", Some(dir.path()), f)
}

#[test]
fn default_config_has_project_root() {
    let config = Config::default();

    assert!(!config.projects_root.is_empty());
}

#[test]
fn config_round_trip_serialization() {
    let config = Config {
        default_ide: Ide::Vscode,
        projects_root: vec![PathBuf::from("C:/Projects")],
        recent_projects: Vec::new(),
    };

    let toml = toml::to_string(&config).unwrap();
    let decoded: Config = toml::from_str(&toml).unwrap();

    assert_eq!(decoded.default_ide, Ide::Vscode);
    assert_eq!(decoded.projects_root.len(), 1);
    assert_eq!(decoded.projects_root[0], PathBuf::from("C:/Projects"));
}

#[test]
fn config_multiple_roots_round_trip() {
    let config = Config {
        default_ide: Ide::Vscode,
        projects_root: vec![
            PathBuf::from("C:/Projects"),
            PathBuf::from("D:/Work"),
            PathBuf::from("/tmp/dev"),
        ],
        recent_projects: Vec::new(),
    };

    let toml = toml::to_string(&config).unwrap();
    let decoded: Config = toml::from_str(&toml).unwrap();

    assert_eq!(decoded.projects_root.len(), 3);
    assert_eq!(decoded.projects_root[1], PathBuf::from("D:/Work"));
}

#[test]
fn invalid_toml_returns_error() {
    let bad = "default_ide = 'banana'";

    let parsed = toml::from_str::<Config>(bad);

    assert!(parsed.is_err());
}

#[test]
fn empty_project_roots_deserialise() {
    let toml = r#"
default_ide = "Cursor"
projects_root = []
"#;

    let config: Config = toml::from_str(toml).unwrap();

    assert!(config.projects_root.is_empty());
}

#[test]
#[serial]
fn load_creates_defaults_when_file_missing() {
    with_temp_config(|| {
        // Make sure the config file does not exist.
        let path = Config::path().expect("config path");
        if path.exists() {
            std::fs::remove_file(&path).expect("remove existing config");
        }

        let config = Config::load().expect("load should succeed");
        assert!(!config.projects_root.is_empty());
        assert_eq!(config.default_ide, Ide::Vscode);

        // Load should have persisted the defaults.
        assert!(path.exists(), "load() should create the config file when missing");
    });
}

#[test]
#[serial]
fn save_creates_parent_directory_when_missing() {
    with_temp_config(|| {
        // Make sure the parent directory does not exist before saving.
        let path = Config::path().expect("config path");
        if let Some(parent) = path.parent()
            && parent.exists()
        {
            std::fs::remove_dir_all(parent).expect("remove existing config dir");
        }
        assert!(!path.exists());

        let config = test_config();
        config.save().expect("save should succeed");

        assert!(path.exists(), "save() should create the config file");
    });
}

#[test]
#[serial]
fn corrupted_config_is_recreated_with_defaults() {
    let temp = TempProject::new("corrupt-config");
    let config_dir = temp.root().join("dev-cli");

    with_var("DEVCLI_CONFIG_DIR", Some(config_dir.as_path()), || {
        let config_path = Config::path().unwrap();
        std::fs::create_dir_all(config_path.parent().unwrap()).unwrap();

        std::fs::write(&config_path, "this isn't valid toml").unwrap();

        let config = Config::load().unwrap();

        assert_eq!(config.default_ide, Ide::Vscode);
        assert_eq!(config.projects_root.len(), 1);

        // Ensure the file was rewritten with valid TOML.
        let rewritten = std::fs::read_to_string(config_path).unwrap();
        assert!(rewritten.contains("projects_root"));
        assert!(rewritten.contains("default_ide"));
    });
}

#[test]
#[serial]
fn add_recent_project_adds_project() -> Result<()> {
    with_temp_config(|| -> Result<()> {
        let mut config = Config::default();

        let weather = project("weather-app");
        config.add_recent_project(&weather);

        assert_eq!(config.recent_projects.len(), 1);
        assert_eq!(config.recent_projects[0].name, "weather-app");
        assert_eq!(config.recent_projects[0].path, PathBuf::from("/projects/weather-app"));

        Ok(())
    })
}

#[test]
#[serial]
fn add_recent_project_moves_duplicate_to_front() -> Result<()> {
    with_temp_config(|| -> Result<()> {
        let mut config = Config::default();

        let alpha = project("alpha");
        let beta = project("beta");

        config.add_recent_project(&alpha);
        config.add_recent_project(&beta);
        config.add_recent_project(&alpha);

        assert_eq!(config.recent_projects.len(), 2);
        assert_eq!(config.recent_projects[0].name, "alpha");
        assert_eq!(config.recent_projects[1].name, "beta");

        Ok(())
    })
}

#[test]
#[serial]
fn add_recent_project_truncates_to_ten() -> Result<()> {
    with_temp_config(|| -> Result<()> {
        let mut config = Config::default();

        for i in 0..12 {
            let project = project(&format!("project-{i}"));
            config.add_recent_project(&project);
        }

        assert_eq!(config.recent_projects.len(), 10);

        assert_eq!(config.recent_projects[0].name, "project-11");
        assert_eq!(config.recent_projects[9].name, "project-2");

        Ok(())
    })
}

#[test]
#[serial]
fn recent_projects_round_trip_serialization() -> Result<()> {
    with_temp_config(|| -> Result<()> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        let config = Config {
            projects_root: vec![PathBuf::from("/projects")],
            default_ide: Ide::Cursor,
            recent_projects: vec![RecentProject {
                name: "weather-app".into(),
                path: PathBuf::from("/projects/weather-app"),
                last_opened: now,
            }],
        };

        config.save()?;

        let loaded = Config::load()?;

        assert_eq!(loaded.recent_projects.len(), 1);
        assert_eq!(loaded.recent_projects[0].name, "weather-app");
        assert_eq!(loaded.recent_projects[0].path, PathBuf::from("/projects/weather-app"));
        assert_eq!(loaded.recent_projects[0].last_opened, now);

        Ok(())
    })
}
