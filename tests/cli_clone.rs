mod common;

use assert_cmd::Command as AssertCommand;
use predicates::prelude::*;
use serial_test::serial;
use std::path::PathBuf;
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

fn setup_test_env_with_roots(roots: &[PathBuf]) -> (AssertCommand, TempDir) {
    let config_dir = tempfile::tempdir().unwrap();
    let roots_toml = roots
        .iter()
        .map(|r| format!("\"{}\"", r.display().to_string().replace('\\', "\\\\")))
        .collect::<Vec<_>>()
        .join(", ");

    let config = format!(
        r#"
default_ide = "Cursor"
projects_root = [{roots_toml}]
"#
    );

    std::fs::write(config_dir.path().join("config.toml"), config).unwrap();

    let mut cmd = AssertCommand::cargo_bin("dev").unwrap();
    cmd.env("DEVCLI_CONFIG_DIR", config_dir.path());
    cmd.env("DEVCLI_SKIP_ONBOARDING", "1");

    (cmd, config_dir)
}

#[test]
#[serial]
fn clone_command_fails_without_url() {
    let (mut cmd, _cfg, _roots) = setup_test_env();

    cmd.args(["clone"]).assert().failure().stderr(predicate::str::contains("Usage"));
}

#[test]
#[serial]
fn clone_command_fails_when_no_roots_configured() {
    let (mut cmd, _cfg) = setup_test_env_with_roots(&[]);

    let source_dir = tempfile::tempdir().unwrap();
    Command::new("git").arg("init").current_dir(source_dir.path()).status().unwrap();
    let source_url = source_dir.path().to_string_lossy().to_string();

    cmd.args(["clone", &source_url])
        .assert()
        .failure()
        .stderr(predicate::str::contains("No project roots configured"));
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

#[test]
#[serial]
fn clone_command_strips_git_suffix() {
    let (mut cmd, _cfg, roots) = setup_test_env();

    let source_parent = tempfile::tempdir().unwrap();
    let source_dir = source_parent.path().join("sample-repo.git");
    std::fs::create_dir_all(&source_dir).unwrap();
    Command::new("git").arg("init").current_dir(&source_dir).status().unwrap();

    let source_url = source_dir.to_string_lossy().to_string();

    cmd.args(["clone", &source_url]).assert().success();

    let target_path = roots.path().join("sample-repo");
    assert!(target_path.exists());
    assert!(target_path.join(".git").exists());
}

#[test]
#[serial]
fn clone_command_with_root_index() {
    let root1 = tempfile::tempdir().unwrap();
    let root2 = tempfile::tempdir().unwrap();

    let (mut cmd, _cfg) =
        setup_test_env_with_roots(&[root1.path().to_path_buf(), root2.path().to_path_buf()]);

    let source_dir = tempfile::tempdir().unwrap();
    Command::new("git").arg("init").current_dir(source_dir.path()).status().unwrap();

    let source_url = source_dir.path().to_string_lossy().to_string();

    cmd.args(["clone", &source_url, "--root", "1", "--name", "IndexedRepo"]).assert().success();

    let target_path = root2.path().join("IndexedRepo");
    assert!(target_path.exists());
    assert!(target_path.join(".git").exists());
    assert!(!root1.path().join("IndexedRepo").exists());
}

#[test]
#[serial]
fn clone_command_with_out_of_bounds_root_index() {
    let (mut cmd, _cfg, _roots) = setup_test_env();

    let source_dir = tempfile::tempdir().unwrap();
    Command::new("git").arg("init").current_dir(source_dir.path()).status().unwrap();
    let source_url = source_dir.path().to_string_lossy().to_string();

    cmd.args(["clone", &source_url, "--root", "5"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Root index 5 is out of bounds"));
}

#[test]
#[serial]
fn clone_command_with_custom_root_path_creates_directory() {
    let (mut cmd, _cfg, _roots) = setup_test_env();

    let parent_dir = tempfile::tempdir().unwrap();
    let nonexistent_root = parent_dir.path().join("new_subfolder");

    let source_dir = tempfile::tempdir().unwrap();
    Command::new("git").arg("init").current_dir(source_dir.path()).status().unwrap();
    let source_url = source_dir.path().to_string_lossy().to_string();

    let root_str = nonexistent_root.to_string_lossy().to_string();
    cmd.args(["clone", &source_url, "--root", &root_str, "--name", "PathRepo"]).assert().success();

    let target_path = nonexistent_root.join("PathRepo");
    assert!(target_path.exists());
    assert!(target_path.join(".git").exists());
}

#[test]
#[serial]
fn clone_command_fails_on_git_clone_error() {
    let (mut cmd, _cfg, _roots) = setup_test_env();

    cmd.args(["clone", "file:///nonexistent/invalid/repo.git"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("git clone failed"));
}
