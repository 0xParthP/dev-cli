use dev_cli::config::Config;

mod common;
use common::temp_project::TempProject;

#[test]
fn config_exists_returns_false_when_missing() {
    let temp = TempProject::new("missing-config");

    unsafe {
        std::env::set_var("DEVCLI_CONFIG_DIR", temp.root().join("dev-cli"));
    }

    assert!(!Config::exists().unwrap());

    unsafe {
        std::env::remove_var("DEVCLI_CONFIG_DIR");
    }
}

#[test]
fn config_create_creates_config_file() {
    let temp = TempProject::new("create-config");

    unsafe {
        std::env::set_var("DEVCLI_CONFIG_DIR", temp.root().join("dev-cli"));
    }

    let config = Config::default();
    Config::create(config).unwrap();

    assert!(Config::exists().unwrap());

    unsafe {
        std::env::remove_var("DEVCLI_CONFIG_DIR");
    }
}
