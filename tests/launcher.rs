use dev_cli::{ide::launcher, models::ide::Ide};
use serial_test::serial;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::{env, fs};
use std::{path::Path, process::Command};

static FAKE_EXE_COUNTER: AtomicU64 = AtomicU64::new(0);

fn fake_executable() -> String {
    // Choose appropriate script name and content based on OS
    let (suffix, content) = if cfg!(windows) {
        ("bat", "@echo off\r\nexit /b 0\r\n")
    } else {
        ("sh", "#!/bin/sh\nexit 0\n")
    };

    // Each call mints a fresh, unique path so a run of serial tests cannot
    // collide on the same inode. pid + counter guarantees uniqueness even
    // if `cargo test` is re-run with stale files lying around.
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
#[serial]
fn launch_cursor_uses_test_executable() {
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
#[serial]
fn launch_terminal_uses_test_executable() {
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
#[serial]
fn launch_claude_uses_test_executable() {
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
#[serial]
fn launch_fails_when_executable_is_invalid() {
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
#[serial]
fn fake_executable_runs_successfully() {
    let mut cmd = Command::new(fake_executable());

    if cfg!(windows) {
        cmd.arg("/C").arg("exit 0");
    }

    let status = cmd.status().unwrap();

    assert!(status.success());
}

#[test]
#[serial]
fn launch_idea_not_installed_returns_error() {
    // Ensure no test executable is set
    unsafe {
        env::remove_var("DEVCLI_TEST_EXECUTABLE");
    }

    // Ide::Idea is never detected by detect_ides(), so this should fail
    let result = launcher::launch(Ide::Idea, Path::new("."));
    assert!(result.is_err());

    // Check that it's the "not installed" error
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Idea is not installed"));
}

#[test]
#[serial]
fn launch_spawn_claude() {
    unsafe {
        env::remove_var("DEVCLI_TEST_EXECUTABLE");
    }

    let executable = fake_executable();
    let result = launcher::launch_spawn(Ide::Claude, Path::new("."), Path::new(&executable));
    assert!(result.is_ok());
}

#[test]
#[serial]
fn launch_spawn_terminal() {
    unsafe {
        env::remove_var("DEVCLI_TEST_EXECUTABLE");
    }

    let executable = fake_executable();
    let result = launcher::launch_spawn(Ide::Terminal, Path::new("."), Path::new(&executable));
    assert!(result.is_ok());
}

#[test]
#[serial]
fn launch_spawn_vscode() {
    unsafe {
        env::remove_var("DEVCLI_TEST_EXECUTABLE");
    }

    let executable = fake_executable();
    let result = launcher::launch_spawn(Ide::Vscode, Path::new("."), Path::new(&executable));
    assert!(result.is_ok());
}

#[test]
#[serial]
fn launch_spawn_cursor() {
    unsafe {
        env::remove_var("DEVCLI_TEST_EXECUTABLE");
    }

    let executable = fake_executable();
    let result = launcher::launch_spawn(Ide::Cursor, Path::new("."), Path::new(&executable));
    assert!(result.is_ok());
}

#[cfg(unix)]
#[test]
#[serial]
fn launch_terminal_uses_true_on_linux() {
    unsafe {
        std::env::set_var("DEVCLI_TEST_EXECUTABLE", "true");
    }

    let result = dev_cli::ide::launcher::launch_terminal(".");

    assert!(result.is_ok());

    unsafe {
        std::env::remove_var("DEVCLI_TEST_EXECUTABLE");
    }
}

#[cfg(unix)]
#[test]
#[serial]
fn launch_cursor_uses_true_on_linux() {
    unsafe {
        std::env::set_var("DEVCLI_TEST_EXECUTABLE", "true");
    }

    let result = dev_cli::ide::launcher::launch_cursor(".");

    assert!(result.is_ok());

    unsafe {
        std::env::remove_var("DEVCLI_TEST_EXECUTABLE");
    }
}

#[cfg(unix)]
#[test]
#[serial]
fn launch_claude_uses_true_on_linux() {
    unsafe {
        std::env::set_var("DEVCLI_TEST_EXECUTABLE", "true");
    }

    let result = dev_cli::ide::launcher::launch_claude(".");

    assert!(result.is_ok());

    unsafe {
        std::env::remove_var("DEVCLI_TEST_EXECUTABLE");
    }
}