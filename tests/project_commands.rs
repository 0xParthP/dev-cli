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

#[test]
fn project_list_runs_with_empty_root() {
    let temp = TempProject::new("empty-root");
    write_temp_config(&temp);

    Command::cargo_bin("dev")
        .unwrap()
        .env("DEVCLI_CONFIG_DIR", temp.root().join("dev-cli"))
        .env("DEVCLI_SKIP_ONBOARDING", "1")
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

    Command::cargo_bin("dev")
        .unwrap()
        .env("DEVCLI_CONFIG_DIR", temp.root().join("dev-cli"))
        .env("DEVCLI_SKIP_ONBOARDING", "1")
        .args(["project", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("demo"));
}

#[test]
fn project_open_missing_project_returns_error() {
    let temp = TempProject::new("missing-project");
    write_temp_config(&temp);

    Command::cargo_bin("dev")
        .unwrap()
        .env("DEVCLI_CONFIG_DIR", temp.root().join("dev-cli"))
        .env("DEVCLI_SKIP_ONBOARDING", "1")
        .args(["project", "open", "does-not-exist"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}

fn run_open_test(ide: &str) {
    let temp = TempProject::new(&format!("open-{}", ide));

    temp.create_git_repo("demo");
    write_temp_config(&temp);

    let bin_dir = temp.root().join("bin");
    std::fs::create_dir_all(&bin_dir).ok();

    #[cfg(windows)]
    let exe = match ide {
        "cursor" => "cursor.exe",
        "terminal" => "wt.exe",
        "claude" => "claude.exe",
        _ => "code.exe",
    };
    #[cfg(not(windows))]
    let exe = match ide {
        "cursor" => "cursor",
        "terminal" => "wt",
        "claude" => "claude",
        _ => "code",
    };

    let exe_path = bin_dir.join(exe);
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

    Command::cargo_bin("dev")
        .unwrap()
        .env("DEVCLI_CONFIG_DIR", temp.root().join("dev-cli"))
        .env("DEVCLI_SKIP_ONBOARDING", "1")
        .env("PATH", path_var)
        .args(["project", "open", "demo", "--ide", ide])
        .assert()
        .success();
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

    let bin_dir = temp.root().join("bin");
    std::fs::create_dir_all(&bin_dir).ok();

    #[cfg(windows)]
    let exe = "cursor.exe";
    #[cfg(not(windows))]
    let exe = "cursor";

    let exe_path = bin_dir.join(exe);
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

    Command::cargo_bin("dev")
        .unwrap()
        .env("DEVCLI_CONFIG_DIR", temp.root().join("dev-cli"))
        .env("DEVCLI_SKIP_ONBOARDING", "1")
        .env("PATH", path_var)
        .args(["open", "demo"])
        .assert()
        .success();
}

#[test]
fn project_list_with_no_git_repositories_prints_header_only() {
    let temp = TempProject::new("no-repositories");
    write_temp_config(&temp);

    Command::cargo_bin("dev")
        .unwrap()
        .env("DEVCLI_CONFIG_DIR", temp.root().join("dev-cli"))
        .env("DEVCLI_SKIP_ONBOARDING", "1")
        .args(["project", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Configured Project Roots"))
        .stdout(predicate::str::contains("Discovered Git Repositories"));
}

#[test]
fn project_list_shows_multiple_git_repositories() {
    let temp = TempProject::new("multiple-projects");

    temp.create_git_repo("alpha");
    temp.create_git_repo("beta");

    write_temp_config(&temp);

    Command::cargo_bin("dev")
        .unwrap()
        .env("DEVCLI_CONFIG_DIR", temp.root().join("dev-cli"))
        .env("DEVCLI_SKIP_ONBOARDING", "1")
        .args(["project", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("alpha"))
        .stdout(predicate::str::contains("beta"));
}
