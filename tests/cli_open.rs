mod common;

use std::sync::Mutex;

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[cfg(unix)]
use common::temp_project::TempProject;

/// Serialise tests that override the platform config directory
/// (`APPDATA` / `XDG_CONFIG_HOME` / `HOME`) so concurrent `set_var` /
/// `remove_var` calls do not race.
static OPEN_ENV_MTX: Mutex<()> = Mutex::new(());

/// Build a `dev` command that points the platform config directory at an
/// isolated temp dir. Returns the `Command` and a `TempDir` guard that
/// must be kept alive for the test's duration.
fn dev_cmd_isolated() -> (Command, TempDir) {
    let tmp = TempDir::new().expect("create temp dir");
    let dir_str = tmp.path().to_string_lossy().into_owned();

    let mut cmd = Command::cargo_bin("dev").unwrap();
    if cfg!(windows) {
        cmd.env("APPDATA", &dir_str);
        cmd.env("LOCALAPPDATA", &dir_str);
    } else {
        cmd.env("XDG_CONFIG_HOME", &dir_str);
        cmd.env("HOME", &dir_str);
    }

    (cmd, tmp)
}

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

#[test]
fn project_list_runs() {
    let _guard = OPEN_ENV_MTX.lock().unwrap();
    let (mut cmd, _tmp) = dev_cmd_isolated();

    cmd.args(["project", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Configured Project Roots"));
}

/// The `dev open <project>` happy path requires controlling the platform
/// config directory so the `projects_root` matches the test fixture.
/// On Linux this is `$XDG_CONFIG_HOME` / `$HOME`, which we can set from
/// the test runner. On Windows the `directories` crate uses the Win32
/// `SHGetKnownFolderPath` API and does not honour `APPDATA` / `LOCALAPPDATA`,
/// so this test is Unix-only.
#[cfg(unix)]
#[test]
fn open_existing_project_with_test_executable() {
    let _guard = OPEN_ENV_MTX.lock().unwrap();

    // Create a fake project at <root>/MyProject/.git
    let repo = TempProject::new("MyProject");
    repo.create_git_repo("MyProject");

    // Point the platform config at a temp dir so the test is hermetic.
    let tmp = TempDir::new().expect("create temp dir");
    let dir_str = tmp.path().to_string_lossy().into_owned();

    // Point the config at the temp project root.
    // The platform config path is `<XDG_CONFIG_HOME>/dev-cli/config.toml`.
    let config_path = tmp.path().join("dev-cli").join("config.toml");
    std::fs::create_dir_all(config_path.parent().unwrap()).unwrap();
    let projects_root_str = repo.root().to_string_lossy().into_owned();
    let config_toml =
        format!("projects_root = [\"{projects_root_str}\"]\ndefault_ide = \"vscode\"\n");
    std::fs::write(&config_path, config_toml).unwrap();

    // Create a fake executable for the test.
    let n = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let fake_exe = std::env::temp_dir().join(format!("devcli_open_test.{n}.sh"));
    std::fs::write(&fake_exe, "#!/bin/sh\nexit 0\n").expect("write fake exe");
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&fake_exe).unwrap().permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&fake_exe, perms).unwrap();
    }

    let mut cmd = Command::cargo_bin("dev").unwrap();
    cmd.env("XDG_CONFIG_HOME", &dir_str);
    cmd.env("HOME", &dir_str);
    cmd.env("DEVCLI_TEST_EXECUTABLE", fake_exe.to_string_lossy().to_string());
    cmd.arg("open").arg("MyProject").assert().success();

    let _ = std::fs::remove_file(&fake_exe);
}
