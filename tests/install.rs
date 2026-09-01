use dev_cli::installer;
use std::sync::Mutex;
use tempfile::TempDir;

/// Serialises all install tests so that concurrent `set_var` / `remove_var`
/// calls on `DEVCLI_INSTALL_DIR` cannot race each other.
static INSTALL_MTX: Mutex<()> = Mutex::new(());

/// Helper that gives the installer an isolated install directory.
///
/// Returns both the `TempDir` (keeps the directory alive) and a `MutexGuard`
/// that prevents other install tests from running concurrently.
fn with_temp_install_dir() -> (TempDir, std::sync::MutexGuard<'static, ()>) {
    let guard = INSTALL_MTX.lock().unwrap();
    let temp = TempDir::new().unwrap();

    unsafe {
        std::env::set_var("DEVCLI_INSTALL_DIR", temp.path());
    }

    (temp, guard)
}

fn cleanup() {
    unsafe {
        std::env::remove_var("DEVCLI_INSTALL_DIR");
    }
}

#[test]
fn install_location_exists() {
    let (temp, _guard) = with_temp_install_dir();

    let dir = installer::binary_install_dir().unwrap();
    assert_eq!(dir, temp.path());

    cleanup();
}

#[test]
fn install_returns_success() {
    let (temp, _guard) = with_temp_install_dir();

    let result = installer::install();
    assert!(result.is_ok());

    let binary = if cfg!(windows) { temp.path().join("dev.exe") } else { temp.path().join("dev") };

    assert!(binary.exists());

    cleanup();
}

#[test]
fn install_is_idempotent() {
    let (temp, _guard) = with_temp_install_dir();

    installer::install().unwrap();
    let second = installer::install();

    assert!(second.is_ok());

    let binary = if cfg!(windows) { temp.path().join("dev.exe") } else { temp.path().join("dev") };

    assert!(binary.exists());

    cleanup();
}
