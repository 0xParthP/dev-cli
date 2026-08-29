use dev_cli::config::Config;

mod common;
use common::temp_config::test_config;

#[test]
fn default_config_has_project_root() {
    let config = Config::default();
    assert!(!config.projects_root.is_empty());
}

#[test]
fn config_round_trip_serialization() {
    let config = test_config();

    let toml = toml::to_string(&config).unwrap();
    let decoded: Config = toml::from_str(&toml).unwrap();

    assert_eq!(config.default_ide, decoded.default_ide);
    assert_eq!(config.projects_root, decoded.projects_root);
}

#[test]
fn invalid_toml_returns_error() {
    let bad = r#"
default_ide = [
"#;

    assert!(toml::from_str::<Config>(bad).is_err());
}
