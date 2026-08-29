#![allow(dead_code)]
use std::{fs, path::PathBuf};

use tempfile::TempDir;

/// Creates a temporary Git repository for testing.
///
/// Returns `(TempDir, repo_path)`.
pub fn create_git_repo(name: &str) -> (TempDir, PathBuf) {
    let dir = TempDir::new().unwrap();

    let repo = dir.path().join(name);

    fs::create_dir_all(repo.join(".git")).unwrap();

    (dir, repo)
}

/// Creates an empty temporary directory.
pub fn create_workspace() -> TempDir {
    TempDir::new().unwrap()
}
