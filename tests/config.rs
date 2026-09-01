use std::path::PathBuf;

use dev_cli::{config::Config, models::ide::Ide};

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
