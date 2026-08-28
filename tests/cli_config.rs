use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn config_show_runs() {
    let mut cmd = Command::cargo_bin("dev").unwrap();

    cmd.arg("config")
        .arg("show")
        .assert()
        .success()
        .stdout(predicate::str::contains("default_ide"));
}
