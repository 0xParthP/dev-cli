use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;
use std::fs;

#[test]
fn opening_unknown_project_returns_error() {
    let temp = TempDir::new().unwrap();
    let projects = temp.path().join("Projects");
    fs::create_dir(&projects).unwrap();

    let mut cmd = Command::cargo_bin("dev").unwrap();

    cmd.arg("open")
        .arg("DoesNotExist")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Project 'DoesNotExist' not found"));
}