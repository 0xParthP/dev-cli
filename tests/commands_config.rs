use assert_cmd::Command;
use predicates::prelude::*;

mod common;
use common::temp_project::TempProject;

fn dev_cmd(temp: &TempProject) -> Command {
    let mut cmd = Command::cargo_bin("dev").unwrap();
    cmd.env("DEVCLI_CONFIG_DIR", temp.root());
    cmd
}

#[test]
fn execute_show_returns_ok() {
    let temp = TempProject::new("show");

    dev_cmd(&temp).args(["config", "init"]).assert().success();

    dev_cmd(&temp)
        .args(["config", "show"])
        .assert()
        .success()
        .stdout(predicate::str::contains("projects_root"))
        .stdout(predicate::str::contains("default_ide"));
}

#[test]
fn execute_init_returns_ok() {
    let temp = TempProject::new("init");

    dev_cmd(&temp).args(["config", "init"]).assert().success();
}

#[test]
fn init_twice_is_idempotent() {
    let temp = TempProject::new("idempotent");

    // First init
    dev_cmd(&temp).args(["config", "init"]).assert().success();

    // Second init (new Command instance)
    dev_cmd(&temp).args(["config", "init"]).assert().success();
}

#[test]
fn show_after_init_contains_project_root() {
    let temp = TempProject::new("show-root");

    dev_cmd(&temp).args(["config", "init"]).assert().success();

    dev_cmd(&temp)
        .args(["config", "show"])
        .assert()
        .success()
        .stdout(predicate::str::contains("projects_root"))
        .stdout(predicate::str::contains("default_ide"));
}
