use dev_cli::installer;
use serial_test::serial;
use tempfile::TempDir;

/// Helper that gives the installer an isolated install directory.
fn with_temp_install_dir() -> TempDir {
    let temp = TempDir::new().unwrap();

    unsafe {
        std::env::set_var("DEVCLI_INSTALL_DIR", temp.path());
    }

    temp
}

fn cleanup() {
    unsafe {
        std::env::remove_var("DEVCLI_INSTALL_DIR");
    }
}

#[test]
#[serial]
fn install_location_exists() {
    let temp = with_temp_install_dir();

    let dir = installer::binary_install_dir().unwrap();
    assert_eq!(dir, temp.path());

    cleanup();
}

#[test]
#[serial]
fn install_returns_success() {
    let temp = with_temp_install_dir();

    let result = installer::install();
    assert!(result.is_ok());

    let binary = if cfg!(windows) { temp.path().join("dev.exe") } else { temp.path().join("dev") };

    assert!(binary.exists());

    cleanup();
}

#[test]
#[serial]
fn install_is_idempotent() {
    let temp = with_temp_install_dir();

    installer::install().unwrap();
    let second = installer::install();

    assert!(second.is_ok());

    let binary = if cfg!(windows) { temp.path().join("dev.exe") } else { temp.path().join("dev") };

    assert!(binary.exists());

    cleanup();
}
