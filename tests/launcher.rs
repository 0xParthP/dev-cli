use dev_cli::{ide::launcher, models::ide::Ide};
use std::path::PathBuf;
use std::sync::Mutex;
use std::{env, fs};
use std::{path::Path, process::Command};

/// Serialises all launcher tests so that concurrent `set_var` / `remove_var`
/// calls on `DEVCLI_TEST_EXECUTABLE` cannot race each other.
/// (Same pattern as `tests/install.rs`.)
static LAUNCH_MTX: Mutex<()> = Mutex::new(());

fn fake_executable() -> String {
    let path: PathBuf = env::temp_dir().join("devcli_fake_launcher.bat");

    if !path.exists() {
        fs::write(&path, "@echo off\r\nexit /b 0\r\n").expect("failed to create fake launcher");
    }

    path.to_string_lossy().into_owned()
}

#[test]
fn launch_cursor_uses_test_executable() {
    let _guard = LAUNCH_MTX.lock().unwrap();

    unsafe {
        std::env::set_var("DEVCLI_TEST_EXECUTABLE", fake_executable());
    }

    let result = launcher::launch(Ide::Cursor, Path::new("."));
    assert!(result.is_ok());

    unsafe {
        std::env::remove_var("DEVCLI_TEST_EXECUTABLE");
    }
}

#[test]
fn launch_terminal_uses_test_executable() {
    let _guard = LAUNCH_MTX.lock().unwrap();

    unsafe {
        std::env::set_var("DEVCLI_TEST_EXECUTABLE", fake_executable());
    }

    let result = launcher::launch(Ide::Terminal, Path::new("."));
    assert!(result.is_ok());

    unsafe {
        std::env::remove_var("DEVCLI_TEST_EXECUTABLE");
    }
}

#[test]
fn launch_claude_uses_test_executable() {
    let _guard = LAUNCH_MTX.lock().unwrap();

    unsafe {
        std::env::set_var("DEVCLI_TEST_EXECUTABLE", fake_executable());
    }

    let result = launcher::launch(Ide::Claude, Path::new("."));
    assert!(result.is_ok());

    unsafe {
        std::env::remove_var("DEVCLI_TEST_EXECUTABLE");
    }
}

#[test]
fn launch_fails_when_executable_is_invalid() {
    let _guard = LAUNCH_MTX.lock().unwrap();

    unsafe {
        std::env::set_var("DEVCLI_TEST_EXECUTABLE", "definitely-not-a-real-executable");
    }

    let result = launcher::launch(Ide::Cursor, Path::new("."));
    assert!(result.is_err());

    unsafe {
        std::env::remove_var("DEVCLI_TEST_EXECUTABLE");
    }
}

#[test]
fn fake_executable_runs_successfully() {
    let mut cmd = Command::new(fake_executable());

    if cfg!(windows) {
        cmd.arg("/C").arg("exit 0");
    }

    let status = cmd.status().unwrap();

    assert!(status.success());
}
