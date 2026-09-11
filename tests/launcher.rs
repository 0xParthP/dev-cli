use dev_cli::{ide::launcher, models::ide::Ide};
use serial_test::serial;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::{env, fs};
use std::{path::Path, process::Command};
use temp_env::with_var;

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

fn with_mock_ide_in_path<F, R>(ide: Ide, f: F) -> R
where
    F: FnOnce() -> R,
{
    let dir = tempfile::TempDir::new().unwrap();
    let bin_dir = dir.path().join("bin");
    fs::create_dir_all(&bin_dir).unwrap();

    let exe_name = match ide {
        Ide::Cursor => {
            if cfg!(windows) {
                "cursor.exe"
            } else {
                "cursor"
            }
        }
        Ide::Claude => {
            if cfg!(windows) {
                "claude.exe"
            } else {
                "claude"
            }
        }
        Ide::Terminal => {
            if cfg!(windows) {
                "wt.exe"
            } else {
                "wt"
            }
        }
        _ => {
            if cfg!(windows) {
                "code.exe"
            } else {
                "code"
            }
        }
    };

    let path = bin_dir.join(exe_name);

    if cfg!(windows) {
        let comspec =
            env::var("COMSPEC").unwrap_or_else(|_| r"C:\Windows\System32\cmd.exe".to_string());
        fs::copy(comspec, &path).unwrap();
    } else {
        let script_content = "#!/bin/sh\nexit 0\n";
        fs::write(&path, script_content).unwrap();

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&path).unwrap().permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&path, perms).unwrap();
        }
    }

    let orig_path = env::var("PATH").unwrap_or_default();
    let path_sep = if cfg!(windows) { ";" } else { ":" };
    let new_path = format!("{}{}{}", bin_dir.display(), path_sep, orig_path);

    with_var("PATH", Some(new_path), f)
}

#[test]
#[serial]
fn launch_cursor_uses_test_executable() {
    with_mock_ide_in_path(Ide::Cursor, || {
        let result = launcher::launch(Ide::Cursor, Path::new("."));
        assert!(result.is_ok());
    });
}

#[test]
#[serial]
fn launch_terminal_uses_test_executable() {
    with_mock_ide_in_path(Ide::Terminal, || {
        let result = launcher::launch(Ide::Terminal, Path::new("."));
        assert!(result.is_ok());
    });
}

#[test]
#[serial]
fn launch_claude_uses_test_executable() {
    with_mock_ide_in_path(Ide::Claude, || {
        let result = launcher::launch(Ide::Claude, Path::new("."));
        assert!(result.is_ok());
    });
}

#[test]
#[serial]
fn launch_fails_when_executable_is_invalid() {
    let result = launcher::launch(Ide::Idea, Path::new("."));
    assert!(result.is_err());
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
    let executable = fake_executable();
    let result = launcher::launch_spawn(Ide::Claude, Path::new("."), Path::new(&executable));
    assert!(result.is_ok());
}

#[test]
#[serial]
fn launch_spawn_terminal() {
    let executable = fake_executable();
    let result = launcher::launch_spawn(Ide::Terminal, Path::new("."), Path::new(&executable));
    assert!(result.is_ok());
}

#[test]
#[serial]
fn launch_spawn_vscode() {
    let executable = fake_executable();
    let result = launcher::launch_spawn(Ide::Vscode, Path::new("."), Path::new(&executable));
    assert!(result.is_ok());
}

#[test]
#[serial]
fn launch_spawn_cursor() {
    let executable = fake_executable();
    let result = launcher::launch_spawn(Ide::Cursor, Path::new("."), Path::new(&executable));
    assert!(result.is_ok());
}

#[test]
#[serial]
fn launcher_accepts_all_supported_ides() {
    with_mock_ide_in_path(Ide::Terminal, || {
        let project = Path::new(".");
        assert!(dev_cli::ide::launcher::launch(Ide::Terminal, project).is_ok());
    });
}
