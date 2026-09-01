use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;

mod common;
use common::temp_project::TempProject;

/// Create a config.toml pointing at the temporary project root.
fn write_temp_config(temp: &TempProject) {
    let config_dir = temp.root().join("dev-cli");
    fs::create_dir_all(&config_dir).unwrap();

    let config = format!(
        r#"
default_ide = "Cursor"

projects_root = ["{}"]
"#,
        temp.root().display().to_string().replace('\\', "\\\\")
    );

    fs::write(config_dir.join("config.toml"), config).unwrap();
}

/// Create a CLI command configured for this temporary project.
fn dev_cmd(temp: &TempProject) -> Command {
    let mut cmd = Command::cargo_bin("dev").unwrap();

    cmd.env("DEVCLI_CONFIG_DIR", temp.root().join("dev-cli"));

    #[cfg(windows)]
    cmd.env("DEVCLI_TEST_EXECUTABLE", "cmd");

    #[cfg(unix)]
    cmd.env("DEVCLI_TEST_EXECUTABLE", "true");

    cmd
}

#[test]
fn project_list_runs_with_empty_root() {
    let temp = TempProject::new("empty-root");
    write_temp_config(&temp);

    dev_cmd(&temp)
        .args(["project", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Configured Project Roots"))
        .stdout(predicate::str::contains("Discovered Git Repositories"));
}

#[test]
fn project_list_discovers_repository() {
    let temp = TempProject::new("project-list");
    temp.create_git_repo("demo");
    write_temp_config(&temp);

    dev_cmd(&temp)
        .args(["project", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("demo"));
}

#[test]
fn project_open_missing_project_returns_error() {
    let temp = TempProject::new("missing-project");
    write_temp_config(&temp);

    dev_cmd(&temp)
        .args(["project", "open", "does-not-exist"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}

fn run_open_test(ide: &str) {
    let temp = TempProject::new(&format!("open-{ide}"));
    temp.create_git_repo("demo");
    write_temp_config(&temp);

    dev_cmd(&temp).args(["project", "open", "demo", "--ide", ide]).assert().success();
}

#[test]
fn project_open_cursor_runs() {
    run_open_test("cursor");
}

#[test]
fn project_open_terminal_runs() {
    run_open_test("terminal");
}

#[test]
fn project_open_claude_runs() {
    run_open_test("claude");
}

#[test]
fn open_shortcut_command_runs() {
    let temp = TempProject::new("shortcut");
    temp.create_git_repo("demo");
    write_temp_config(&temp);

    dev_cmd(&temp).args(["open", "demo"]).assert().success();
}
