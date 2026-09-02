mod common;

use std::sync::Mutex;

use assert_cmd::Command;
use common::assertions::contains_usage;
use predicates::prelude::*;
use tempfile::TempDir;

/// Serialise tests that override the platform config directory
/// (`APPDATA` / `XDG_CONFIG_HOME` / `HOME`) so concurrent `set_var` /
/// `remove_var` calls do not race.
static CFG_ENV_MTX: Mutex<()> = Mutex::new(());

/// Build a Command that points the platform config directory at an
/// isolated temp dir, then return both the `Command` and the `TempDir`
/// (the latter so the caller can keep it alive for the test).
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
fn config_set_default_ide_persists() {
    let _guard = CFG_ENV_MTX.lock().unwrap();
    let (mut cmd, _tmp) = dev_cmd_isolated();

    cmd.args(["config", "init"]).assert().success();

    let (mut cmd, _tmp) = dev_cmd_isolated();
    cmd.args(["config", "set-default-ide", "cursor"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Default IDE updated"));

    let (mut cmd, _tmp) = dev_cmd_isolated();
    cmd.args(["config", "show"]).assert().success().stdout(predicate::str::contains("Cursor"));
}
