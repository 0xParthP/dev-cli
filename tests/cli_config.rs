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

fn isolated_cmd(dir: &TempDir) -> Command {
    let mut cmd = Command::cargo_bin("dev").unwrap();

    // Use the application's explicit test override.
    cmd.env("DEVCLI_CONFIG_DIR", dir.path());

    cmd
}

#[test]
#[serial]
fn config_show_runs() {
    let tmp = isolated_temp_dir();

    isolated_cmd(&tmp)
        .args(["config", "show"])
        .assert()
        .success()
        .stdout(predicate::str::contains("default_ide"));
}

#[test]
#[serial]
fn config_init_runs() {
    let tmp = isolated_temp_dir();

    isolated_cmd(&tmp)
        .args(["config", "init"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Config created"));
}

#[test]
#[serial]
fn config_help_runs() {
    let tmp = isolated_temp_dir();

    isolated_cmd(&tmp).args(["config", "--help"]).assert().success().stdout(contains_usage());
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
