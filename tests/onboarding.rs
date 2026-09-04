use dev_cli::config::Config;
use dev_cli::models::ide::Ide;
use dev_cli::onboarding::{default_projects_dir, ensure_onboarded};
use serial_test::serial;
use std::io::IsTerminal;
use std::path::PathBuf;

mod common;
use crate::common::temp_project::TempProject;

/// Helper to point the config directory at an isolated temp dir
fn isolate_config_dir() -> tempfile::TempDir {
    let tmp = tempfile::TempDir::new().expect("create temp dir");
    let dir = tmp.path().to_string_lossy().into_owned();
    unsafe {
        std::env::set_var("DEVCLI_CONFIG_DIR", &dir);
    }
    tmp
}

fn reset_config_dir_env() {
    unsafe {
        std::env::remove_var("DEVCLI_CONFIG_DIR");
    }
}

#[test]
#[serial]
fn default_projects_dir_returns_projects_subdir() {
    // default_projects_dir() should return a path containing "Projects".
    let dir = default_projects_dir();
    assert!(dir.contains("Projects"), "default dir should contain 'Projects', got: {dir}");
}

#[test]
#[serial]
fn default_projects_dir_uses_home_when_available() {
    // On a normal dev machine, BaseDirs resolves to a real home directory,
    // so the default should be an absolute path ending in "Projects".
    // (Skipped in sandboxed environments where home is unknown.)
    if let Some(home) = directories::BaseDirs::new().map(|d| d.home_dir().to_path_buf()) {
        let dir = default_projects_dir();
        let expected = home.join("Projects");
        assert_eq!(PathBuf::from(&dir), expected);
    }
}

#[test]
#[serial]
fn ensure_onboarded_returns_ok_when_config_exists() {
    // If config already exists, ensure_onboarded should short-circuit
    // and never call the interactive wizard.
    let _tmp = isolate_config_dir();

    // Pre-create a config so onboarding has nothing to do.
    let config = Config::default();
    config.save().expect("seed config");
    assert!(Config::exists().unwrap());

    // Should succeed without any terminal interaction.
    let result = ensure_onboarded();
    assert!(result.is_ok());

    reset_config_dir_env();
}

#[test]
#[serial]
fn ensure_onboarded_returns_ok_when_config_missing_but_no_terminal() {
    // When stdin/stdout are not interactive terminals, ensure_onboarded
    // must not run the wizard and must not create a config file.
    let _tmp = isolate_config_dir();

    // Make sure no config exists.
    let path = Config::path().unwrap();
    if path.exists() {
        std::fs::remove_file(&path).unwrap();
    }
    assert!(!Config::exists().unwrap());

    // In a non-interactive context, ensure_onboarded should silently return Ok.
    // This is what protects `dev` from blocking in CI or piped invocations.
    if !std::io::stdin().is_terminal() || !std::io::stdout().is_terminal() {
        let result = ensure_onboarded();
        assert!(result.is_ok());

        // Critically, the wizard must NOT have created a config.
        assert!(!Config::exists().unwrap(), "non-terminal run must not create config");
    }

    reset_config_dir_env();
}

#[test]
#[serial]
fn ensure_onboarded_does_not_overwrite_existing_config() {
    // If config already exists, ensure_onboarded must not touch it,
    // even when running through the short-circuit branch.
    let _tmp = isolate_config_dir();

    // Pre-create a config with distinctive values.
    let original = Config {
        default_ide: Ide::Cursor,
        projects_root: vec![PathBuf::from("C:/OriginalProjects")],
    };
    original.save().expect("seed config");

    let path = Config::path().unwrap();
    let original_contents = std::fs::read_to_string(&path).unwrap();

    // Call ensure_onboarded - should be a no-op since config exists.
    let _ = ensure_onboarded();

    // File contents should be unchanged.
    let after_contents = std::fs::read_to_string(&path).unwrap();
    assert_eq!(original_contents, after_contents, "ensure_onboarded must not rewrite config");

    // And the loaded values should still be our originals.
    let loaded = Config::load().unwrap();
    assert_eq!(loaded.default_ide, Ide::Cursor);
    assert_eq!(loaded.projects_root, vec![PathBuf::from("C:/OriginalProjects")]);

    reset_config_dir_env();
}

#[test]
#[serial]
fn onboarding_module_exposes_expected_public_api() {
    // Compile-time guard: the public API of the onboarding module
    // must include ensure_onboarded and default_projects_dir.
    // If either is removed, this test stops compiling.
    use dev_cli::onboarding;

    let _: fn() -> anyhow::Result<()> = onboarding::ensure_onboarded;
    let _: fn() -> String = onboarding::default_projects_dir;
}

#[test]
#[serial]
fn config_exists_returns_false_when_missing() {
    let temp = TempProject::new("missing-config");

    unsafe {
        std::env::set_var("DEVCLI_CONFIG_DIR", temp.root().join("dev-cli"));
    }

    assert!(!Config::exists().unwrap());

    reset_config_dir_env();
}

#[test]
#[serial]
fn config_create_creates_config_file() {
    let temp = TempProject::new("create-config");

    unsafe {
        std::env::set_var("DEVCLI_CONFIG_DIR", temp.root().join("dev-cli"));
    }

    let config = Config::default();
    Config::create(config).unwrap();

    assert!(Config::exists().unwrap());

    reset_config_dir_env();
}
