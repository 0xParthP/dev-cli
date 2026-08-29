mod common;

use assert_cmd::Command;
use common::assertions::contains_usage;
use predicates::prelude::*;

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
