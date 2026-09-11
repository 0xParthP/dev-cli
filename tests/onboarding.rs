mod common;
use dev_cli::config::Config;
use dev_cli::models::ide::Ide;
use dev_cli::onboarding::{default_projects_dir, ensure_onboarded};
use serial_test::serial;
use std::io::IsTerminal;
use std::path::PathBuf;
use temp_env::with_var;
use tempfile::TempDir;

/// Run a test with DEVCLI_CONFIG_DIR pointing at an isolated temp directory.
/// The environment variable is automatically restored afterwards.
fn with_temp_config<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let dir = TempDir::new().unwrap();
    with_var("DEVCLI_CONFIG_DIR", Some(dir.path()), f)
}

#[test]
#[serial]
fn default_projects_dir_returns_projects_subdir() {
    let dir = default_projects_dir();
    assert!(dir.contains("Projects"), "default dir should contain 'Projects', got: {dir}");
}

#[test]
#[serial]
fn default_projects_dir_uses_home_when_available() {
    if let Some(home) = directories::BaseDirs::new().map(|d| d.home_dir().to_path_buf()) {
        let dir = default_projects_dir();
        let expected = home.join("Projects");
        assert_eq!(PathBuf::from(&dir), expected);
    }
}

#[test]
#[serial]
fn ensure_onboarded_returns_ok_when_config_exists() {
    with_temp_config(|| {
        Config::default().save().unwrap();

        assert!(Config::exists().unwrap());
        assert!(ensure_onboarded().is_ok());
    });
}

#[test]
#[serial]
fn ensure_onboarded_returns_ok_when_config_missing_but_no_terminal() {
    with_temp_config(|| {
        let path = Config::path().unwrap();

        if path.exists() {
            std::fs::remove_file(&path).unwrap();
        }

        assert!(!Config::exists().unwrap());

        if !std::io::stdin().is_terminal() || !std::io::stdout().is_terminal() {
            assert!(ensure_onboarded().is_ok());

            // Wizard must not create a config when not attached to a TTY.
            assert!(!Config::exists().unwrap(), "non-terminal run must not create config");
        }
    });
}

#[test]
#[serial]
fn ensure_onboarded_does_not_overwrite_existing_config() {
    with_temp_config(|| {
        let original = Config {
            default_ide: Ide::Vscode,
            projects_root: vec![PathBuf::from("C:/OriginalProjects")],
            recent_projects: Vec::new(),
        };

        original.save().unwrap();

        let path = Config::path().unwrap();
        let before = std::fs::read_to_string(&path).unwrap();

        ensure_onboarded().unwrap();

        let after = std::fs::read_to_string(&path).unwrap();
        assert_eq!(before, after);

        let loaded = Config::load().unwrap();
        assert_eq!(loaded.default_ide, Ide::Vscode);
        assert_eq!(loaded.projects_root, vec![PathBuf::from("C:/OriginalProjects")]);
    });
}

#[test]
#[serial]
fn onboarding_module_exposes_expected_public_api() {
    use dev_cli::onboarding;

    let _: fn() -> anyhow::Result<()> = onboarding::ensure_onboarded;
    let _: fn() -> String = onboarding::default_projects_dir;
}

#[test]
#[serial]
fn config_exists_returns_false_when_missing() {
    with_temp_config(|| {
        assert!(!Config::exists().unwrap());
    });
}

#[test]
#[serial]
fn config_create_creates_config_file() {
    with_temp_config(|| {
        Config::create(Config::default()).unwrap();
        assert!(Config::exists().unwrap());
    });
}

#[test]
#[serial]
fn existing_config_skips_onboarding() {
    with_temp_config(|| {
        with_var("DEVCLI_SKIP_ONBOARDING", Some("1"), || {
            Config::default().save().unwrap();

            ensure_onboarded().unwrap();

            let loaded = Config::load().unwrap();
            assert_eq!(loaded.default_ide, Config::default().default_ide);
        });
    });
}

#[test]
#[serial]
fn config_exists_after_save() {
    with_temp_config(|| {
        assert!(!Config::exists().unwrap());

        Config::default().save().unwrap();

        assert!(Config::exists().unwrap());
    });
}
