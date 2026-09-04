mod common;

use assert_cmd::Command;
use predicates::prelude::*;
use serial_test::serial;
use tempfile::TempDir;

#[cfg(unix)]
use common::temp_project::TempProject;
#[cfg(unix)]
use std::{fs, io::Write};

#[cfg(unix)]
fn create_fake_executable() -> std::path::PathBuf {
    let dir = TempDir::new().unwrap();
    let dir_path = dir.keep();

    let path = if cfg!(windows) {
        dir_path.join("fake.cmd")
    } else {
        dir_path.join("fake.sh")
    };

    let mut file = fs::File::create(&path).unwrap();

    if cfg!(windows) {
        writeln!(file, "@echo off").unwrap();
        writeln!(file, "exit /b 0").unwrap();
    } else {
        writeln!(file, "#!/bin/sh").unwrap();
        writeln!(file, "exit 0").unwrap();

        use std::os::unix::fs::PermissionsExt;

        let mut perms = file.metadata().unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&path, perms).unwrap();
    }

    drop(file);

    path
}

/// Build a `dev` command that points the platform config directory at an
/// isolated temp dir.
fn dev_cmd_isolated() -> (Command, TempDir) {
    let tmp = TempDir::new().expect("create temp dir");
    let dir_str = tmp.path().to_string_lossy().into_owned();

    let mut cmd = Command::cargo_bin("dev").unwrap();
    if cfg!(windows) {
        cmd.env("APPDATA", &dir_str);
        cmd.env("LOCALAPPDATA", &dir_str);
    } else {
        cmd.env("XDG_CONFIG_HOME", &dir_str);
        cmd.env("HOME", &dir_str);
    }

    (cmd, tmp)
}

#[test]
fn unknown_project_returns_error() {
    Command::cargo_bin("dev")
        .unwrap()
        .args(["open", "DoesNotExist"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Project"));
}

#[test]
fn help_for_open_command_works() {
    Command::cargo_bin("dev")
        .unwrap()
        .args(["open", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage"));
}

#[test]
fn open_with_specific_ide_parses() {
    Command::cargo_bin("dev")
        .unwrap()
        .args(["open", "FakeProject", "--ide", "vscode"])
        .assert()
        .failure();
}

#[test]
#[serial]
fn project_list_runs() {
    let (mut cmd, _tmp) = dev_cmd_isolated();

    cmd.args(["project", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Configured Project Roots"));
}

/// UNIX only test that creates a fake project and a fake executable to test the `dev open` command.
#[cfg(unix)]
#[test]
#[serial]
fn open_existing_project_with_test_executable() {
    let temp = TempProject::new("cli-open");
    temp.create_git_repo("MyProject");

    // Create a temporary config that points at the temp project root.
    let config_dir = temp.root().join("dev-cli");
    std::fs::create_dir_all(&config_dir).unwrap();

    let config = format!(
        r#"
default_ide = "Cursor"

projects_root = ["{}"]
"#,
        temp.root().display().to_string().replace('\\', "\\\\")
    );

    std::fs::write(config_dir.join("config.toml"), config).unwrap();

    let fake = create_fake_executable();

    Command::cargo_bin("dev")
        .unwrap()
        .env("DEVCLI_SKIP_ONBOARDING", "1")
        .env("DEVCLI_TEST_EXECUTABLE", &fake)
        .env("DEVCLI_CONFIG_DIR", &config_dir)
        .args(["open", "MyProject"])
        .assert()
        .success();
}
