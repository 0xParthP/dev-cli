use dev_cli::{config::Config, models::ide::Ide};
use serial_test::serial;
use std::path::PathBuf;

mod common;
use common::temp_config::test_config;
use crate::common::temp_project::TempProject;

/// Helper to point the config directory at an isolated temp dir
fn isolate_config_dir() -> (tempfile::TempDir, std::path::PathBuf) {
    let tmp = tempfile::TempDir::new().expect("create temp dir");
    let dir = tmp.path().to_path_buf();
    let dir_str = dir.to_string_lossy().into_owned();
    unsafe {
        std::env::set_var("DEVCLI_CONFIG_DIR", &dir_str);
    }
    (tmp, dir)
}

fn reset_config_dir_env() {
    unsafe {
        std::env::remove_var("DEVCLI_CONFIG_DIR");
    }
}

#[test]
fn default_config_has_project_root() {
    let config = Config::default();

    assert!(!config.projects_root.is_empty());
}

#[test]
fn config_round_trip_serialization() {
    let config =
        Config { default_ide: Ide::Cursor, projects_root: vec![PathBuf::from("C:/Projects")] };

    let toml = toml::to_string(&config).unwrap();
    let decoded: Config = toml::from_str(&toml).unwrap();

    assert_eq!(decoded.default_ide, Ide::Cursor);
    assert_eq!(decoded.projects_root.len(), 1);
    assert_eq!(decoded.projects_root[0], PathBuf::from("C:/Projects"));
}

#[test]
fn config_multiple_roots_round_trip() {
    let config = Config {
        default_ide: Ide::Cursor,
        projects_root: vec![
            PathBuf::from("C:/Projects"),
            PathBuf::from("D:/Work"),
            PathBuf::from("/tmp/dev"),
        ],
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
    let (_tmp, _dir) = isolate_config_dir();

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

    reset_config_dir_env();
}

#[test]
#[serial]
fn save_creates_parent_directory_when_missing() {
    let (_tmp, _dir) = isolate_config_dir();

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

    reset_config_dir_env();
}

#[test]
#[serial]
fn corrupted_config_is_recreated_with_defaults() {
    let temp = TempProject::new("corrupt-config");

    unsafe {
        std::env::set_var("DEVCLI_CONFIG_DIR", temp.root().join("dev-cli"));
    }

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

    unsafe {
        std::env::remove_var("DEVCLI_CONFIG_DIR");
    }
}