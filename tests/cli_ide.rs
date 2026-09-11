use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn ide_list_runs() {
    let tmp = TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("dev").unwrap();

    cmd.env("DEVCLI_CONFIG_DIR", tmp.path())
        .args(["ide", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Installed IDEs"));
}
