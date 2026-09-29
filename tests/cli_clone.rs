mod common;

use assert_cmd::Command as AssertCommand;
use predicates::prelude::*;
use serial_test::serial;
use std::process::Command;
use tempfile::TempDir;

fn setup_test_env() -> (AssertCommand, TempDir, TempDir) {
    let config_dir = tempfile::tempdir().unwrap();
    let roots_dir = tempfile::tempdir().unwrap();

    let config = format!(
        r#"
default_ide = "Cursor"
projects_root = ["{}"]
"#,
        roots_dir.path().display().to_string().replace('\\', "\\\\")
    );

    std::fs::write(config_dir.path().join("config.toml"), config).unwrap();

    let mut cmd = AssertCommand::cargo_bin("dev").unwrap();
    cmd.env("DEVCLI_CONFIG_DIR", config_dir.path());
    cmd.env("DEVCLI_SKIP_ONBOARDING", "1");

    (cmd, config_dir, roots_dir)
}

#[test]
fn clone_command_fails_without_url() {
    let (mut cmd, _cfg, _roots) = setup_test_env();

    cmd.args(["clone"]).assert().failure().stderr(predicate::str::contains("Usage"));
}

#[test]
#[serial]
fn clone_command_clones_local_repo() {
    let (mut cmd, _cfg, roots) = setup_test_env();

    // Create a real local git repo to clone from
    let source_dir = tempfile::tempdir().unwrap();
    Command::new("git").arg("init").current_dir(source_dir.path()).status().unwrap();

    let source_url = source_dir.path().to_string_lossy().to_string();

    cmd.args(["clone", &source_url]).assert().success();

    // The cloned repo should exist in roots_dir
    let repo_name = source_dir.path().file_name().unwrap().to_string_lossy();
    let target_path = roots.path().join(repo_name.as_ref());
    assert!(target_path.exists());
    assert!(target_path.join(".git").exists());
}

#[test]
#[serial]
fn clone_command_with_custom_name() {
    let (mut cmd, _cfg, roots) = setup_test_env();

    let source_dir = tempfile::tempdir().unwrap();
    Command::new("git").arg("init").current_dir(source_dir.path()).status().unwrap();

    let source_url = source_dir.path().to_string_lossy().to_string();

    cmd.args(["clone", &source_url, "--name", "CustomRepoName"]).assert().success();

    let target_path = roots.path().join("CustomRepoName");
    assert!(target_path.exists());
    assert!(target_path.join(".git").exists());
}
