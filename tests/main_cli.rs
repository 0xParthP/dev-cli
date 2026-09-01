use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn root_help_runs() {
    Command::cargo_bin("dev")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage"));
}

#[test]
fn version_flag_runs() {
    Command::cargo_bin("dev").unwrap().arg("--version").assert().success();
}

#[test]
fn invalid_command_fails() {
    Command::cargo_bin("dev").unwrap().arg("not-a-command").assert().failure();
}
