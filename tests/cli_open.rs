mod common;

use assert_cmd::Command;
use predicates::prelude::*;
use serial_test::serial;
use tempfile::TempDir;

use common::temp_project::TempProject;

fn dev_cmd_isolated() -> (Command, TempDir) {
    let tmp = TempDir::new().expect("create temp dir");

    let mut cmd = Command::cargo_bin("dev").unwrap();
    cmd.env("DEVCLI_CONFIG_DIR", tmp.path());

    (cmd, tmp)
}

#[test]
fn unknown_project_returns_error() {
    let (mut cmd, _tmp) = dev_cmd_isolated();

    cmd.args(["open", "DoesNotExist"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Project"));
}

#[test]
fn help_for_open_command_works() {
    let (mut cmd, _tmp) = dev_cmd_isolated();

    cmd.args(["open", "--help"]).assert().success().stdout(predicate::str::contains("Usage"));
}

#[test]
fn open_with_specific_ide_parses() {
    let (mut cmd, _tmp) = dev_cmd_isolated();

    cmd.args(["open", "FakeProject", "--ide", "vscode"]).assert().failure();
}

#[test]
#[serial]
fn project_list_runs() {
    let (mut cmd, _tmp) = dev_cmd_isolated();

    cmd.args(["project", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Configured Project Roots"));
}

/// Cross-platform test that creates a fake project and a fake executable to test the `dev open` command.
#[test]
#[serial]
fn open_existing_project_with_test_executable() {
    let temp = TempProject::new("cli-open");
    temp.create_git_repo("MyProject");

    // Create a temporary config that points at the temp project root.
    let config_dir = temp.root().join("dev-cli");
    std::fs::create_dir_all(&config_dir).unwrap();

    let config = format!(
        r#"
default_ide = "Cursor"

projects_root = ["{}"]
"#,
        temp.root().display().to_string().replace('\\', "\\\\")
    );

    std::fs::write(config_dir.join("config.toml"), config).unwrap();

    let bin_dir = temp.root().join("bin");
    std::fs::create_dir_all(&bin_dir).unwrap();

    #[cfg(windows)]
    let exe_name = "cursor.exe";
    #[cfg(not(windows))]
    let exe_name = "cursor";

    let exe_path = bin_dir.join(exe_name);
    if cfg!(windows) {
        let comspec =
            std::env::var("COMSPEC").unwrap_or_else(|_| r"C:\Windows\System32\cmd.exe".to_string());
        std::fs::copy(comspec, &exe_path).unwrap();
    } else {
        std::fs::write(&exe_path, "#!/bin/sh\nexit 0\n").unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&exe_path).unwrap().permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&exe_path, perms).unwrap();
        }
    }

    let path_sep = if cfg!(windows) { ";" } else { ":" };
    let path_var = match std::env::var("PATH") {
        Ok(p) => format!("{}{}{}", bin_dir.display(), path_sep, p),
        Err(_) => bin_dir.display().to_string(),
    };

    Command::cargo_bin("dev")
        .unwrap()
        .env("DEVCLI_SKIP_ONBOARDING", "1")
        .env("PATH", path_var)
        .env("DEVCLI_CONFIG_DIR", &config_dir)
        .args(["open", "MyProject"])
        .assert()
        .success();
}
