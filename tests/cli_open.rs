use assert_cmd::Command;
use predicates::prelude::*;

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
