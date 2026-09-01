use dev_cli::{ide::launcher, models::ide::Ide};
use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::{env, fs};
use std::{path::Path, process::Command};

/// Serialises all launcher tests so that concurrent `set_var` / `remove_var`
/// calls on `DEVCLI_TEST_EXECUTABLE` cannot race each other.
/// (Same pattern as `tests/install.rs`.)
static LAUNCH_MTX: Mutex<()> = Mutex::new(());

/// Counter used to mint a unique fake-executable path per test invocation.
/// Each test gets its own script so a concurrent `exec` and a concurrent
/// `write` can never target the same inode (which would raise `ETXTBSY`
/// on Linux while another process is mid-`execve`).
static FAKE_EXE_COUNTER: AtomicU64 = AtomicU64::new(0);

fn fake_executable() -> String {
    // Choose appropriate script name and content based on OS
    let (suffix, content) = if cfg!(windows) {
        ("bat", "@echo off\r\nexit /b 0\r\n")
    } else {
        ("sh", "#!/bin/sh\nexit 0\n")
    };

    // Each call mints a fresh, unique path so parallel tests cannot
    // collide on the same inode. pid + counter guarantees uniqueness
    // even if multiple threads invoke this concurrently.
    let n = FAKE_EXE_COUNTER.fetch_add(1, Ordering::Relaxed);
    let path: PathBuf =
        env::temp_dir().join(format!("devcli_fake_launcher.{}.{n}.{suffix}", std::process::id()));

    fs::write(&path, content).expect("failed to create fake launcher");

    // On Unix, the script must be executable; Windows ignores the bit.
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&path).expect("metadata failed").permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&path, perms).expect("failed to set exec permission");
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
