mod common;

use assert_cmd::Command;
use common::assertions::contains_usage;
use predicates::prelude::*;
use serial_test::serial;
use tempfile::TempDir;

/// Create a temporary isolated configuration directory.
fn isolated_temp_dir() -> TempDir {
    TempDir::new().expect("create temp dir")
}

/// Build a Command that points the platform config directory at the
/// given isolated temp dir.
fn isolated_cmd(dir: &TempDir) -> Command {
    let dir_str = dir.path().to_string_lossy().into_owned();

    let mut cmd = Command::cargo_bin("dev").unwrap();
    if cfg!(windows) {
        cmd.env("APPDATA", &dir_str);
        cmd.env("LOCALAPPDATA", &dir_str);
    } else {
        cmd.env("XDG_CONFIG_HOME", &dir_str);
        cmd.env("HOME", &dir_str);
    }

    cmd
}

#[test]
fn config_show_runs() {
    Command::cargo_bin("dev")
        .unwrap()
        .args(["config", "show"])
        .assert()
        .success()
        .stdout(predicate::str::contains("default_ide"));
}

#[test]
fn config_init_runs() {
    Command::cargo_bin("dev")
        .unwrap()
        .args(["config", "init"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Config created"));
}

#[test]
fn config_help_runs() {
    Command::cargo_bin("dev")
        .unwrap()
        .args(["config", "--help"])
        .assert()
        .success()
        .stdout(contains_usage());
}

#[test]
#[serial]
fn config_set_default_ide_persists() {
    let tmp = isolated_temp_dir();

    isolated_cmd(&tmp).args(["config", "init"]).assert().success();

    isolated_cmd(&tmp)
        .args(["config", "set-default-ide", "cursor"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Default IDE updated"));

    isolated_cmd(&tmp)
        .args(["config", "show"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Cursor"));
}
