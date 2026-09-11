mod common;

use assert_cmd::Command;
use common::assertions::contains_usage;
use predicates::prelude::*;
use serial_test::serial;
use tempfile::TempDir;

/// Create a temporary isolated configuration directory.
fn isolated_temp_dir() -> TempDir {
    TempDir::new().expect("create temp dir")
}

fn isolated_cmd(dir: &TempDir) -> Command {
    let bin_dir = dir.path().join("bin");
    std::fs::create_dir_all(&bin_dir).ok();

    #[cfg(windows)]
    let exe_name = "cursor.exe";
    #[cfg(not(windows))]
    let exe_name = "cursor";

    let exe_path = bin_dir.join(exe_name);
    if cfg!(windows) {
        let comspec =
            std::env::var("COMSPEC").unwrap_or_else(|_| r"C:\Windows\System32\cmd.exe".to_string());
        std::fs::copy(comspec, &exe_path).ok();
    } else {
        std::fs::write(&exe_path, "#!/bin/sh\nexit 0\n").ok();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(meta) = std::fs::metadata(&exe_path) {
                let mut perms = meta.permissions();
                perms.set_mode(0o755);
                let _ = std::fs::set_permissions(&exe_path, perms);
            }
        }
    }

    let path_sep = if cfg!(windows) { ";" } else { ":" };
    let path_var = match std::env::var("PATH") {
        Ok(p) => format!("{}{}{}", bin_dir.display(), path_sep, p),
        Err(_) => bin_dir.display().to_string(),
    };

    let mut cmd = Command::cargo_bin("dev").unwrap();

    cmd.env("DEVCLI_CONFIG_DIR", dir.path());
    cmd.env("PATH", path_var);

    cmd
}

#[test]
#[serial]
fn config_show_runs() {
    let tmp = isolated_temp_dir();

    isolated_cmd(&tmp)
        .args(["config", "show"])
        .assert()
        .success()
        .stdout(predicate::str::contains("default_ide"));
}

#[test]
#[serial]
fn config_init_runs() {
    let tmp = isolated_temp_dir();

    isolated_cmd(&tmp)
        .args(["config", "init"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Config created"));
}

#[test]
#[serial]
fn config_help_runs() {
    let tmp = isolated_temp_dir();

    isolated_cmd(&tmp).args(["config", "--help"]).assert().success().stdout(contains_usage());
}

#[test]
#[serial]
fn config_set_default_ide_persists() {
    let tmp = isolated_temp_dir();

    isolated_cmd(&tmp).args(["config", "init"]).assert().success();

    isolated_cmd(&tmp)
        .args(["config", "set-default-ide", "cursor"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Default IDE updated"));

    isolated_cmd(&tmp)
        .args(["config", "show"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Cursor"));
}

#[test]
#[serial]
fn config_set_default_ide_uninstalled_fails() {
    let tmp = isolated_temp_dir();

    // Use a command without dummy PATH
    let mut cmd = Command::cargo_bin("dev").unwrap();
    cmd.env("DEVCLI_CONFIG_DIR", tmp.path());
    cmd.env("PATH", "");

    cmd.args(["config", "set-default-ide", "cursor"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("is not installed on your system"));
}
