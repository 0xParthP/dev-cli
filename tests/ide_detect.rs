//! Integration tests for Windows location detection logic.
//! These tests were moved from src/ide/detect.rs to comply with the
//! rule that all tests must reside under the `tests/` directory.

use dev_cli::ide::detect::detect_common_windows_locations_in;
use dev_cli::ide::registry::InstalledIde;
use dev_cli::models::ide::Ide;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use tempfile::TempDir;

/// Helper to create a fake executable at the given Windows-style relative path
fn create_windows_exe(home: &Path, rel: &str) {
    let exe = home.join(rel);
    fs::create_dir_all(exe.parent().unwrap()).unwrap();
    fs::write(&exe, "").unwrap();
}

#[test]
fn detects_vscode_in_windows_location() {
    let dir = TempDir::new().unwrap();
    let home = dir.path();
    create_windows_exe(home, "AppData/Local/Programs/Microsoft VS Code/bin/code.cmd");
    let mut list = Vec::new();
    detect_common_windows_locations_in(&mut list, home);
    assert!(list.iter().any(|i| i.ide == Ide::Vscode));
}

#[test]
fn detects_cursor_in_windows_location() {
    let dir = TempDir::new().unwrap();
    let home = dir.path();
    create_windows_exe(home, "AppData/Local/Programs/Cursor/Cursor.exe");
    let mut list = Vec::new();
    detect_common_windows_locations_in(&mut list, home);
    assert!(list.iter().any(|i| i.ide == Ide::Cursor));
}

#[test]
fn detects_claude_in_local_bin() {
    let dir = TempDir::new().unwrap();
    let home = dir.path();
    create_windows_exe(home, ".local/bin/claude.exe");
    let mut list = Vec::new();
    detect_common_windows_locations_in(&mut list, home);
    assert!(list.iter().any(|i| i.ide == Ide::Claude));
}

#[test]
fn empty_home_detects_nothing() {
    let dir = TempDir::new().unwrap();
    let mut list = Vec::new();
    detect_common_windows_locations_in(&mut list, dir.path());
    assert!(list.is_empty());
}

#[test]
fn vscode_not_added_twice() {
    let dir = TempDir::new().unwrap();
    let home = dir.path();
    create_windows_exe(home, "AppData/Local/Programs/Microsoft VS Code/bin/code.cmd");
    let mut list = Vec::new();
    // Simulate VS Code already found via PATH.
    list.push(InstalledIde::new(Ide::Vscode, "VS Code", PathBuf::from("code")));
    detect_common_windows_locations_in(&mut list, home);
    assert_eq!(list.iter().filter(|i| i.ide == Ide::Vscode).count(), 1);
}
