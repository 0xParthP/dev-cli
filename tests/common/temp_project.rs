#![allow(dead_code)]

use std::{fs, path::PathBuf};
use tempfile::TempDir;

/// Temporary Git repository used by integration tests.
///
/// This helper is shared across multiple test crates under `tests/`,
/// so some individual test crates won't use every method.
pub struct TempProject {
    dir: TempDir,
    repo: PathBuf,
}

impl TempProject {
    pub fn new(name: &str) -> Self {
        let dir = TempDir::new().unwrap();
        let repo = dir.path().join(name);

        fs::create_dir_all(repo.join(".git")).unwrap();

        Self { dir, repo }
    }

    pub fn root(&self) -> PathBuf {
        self.dir.path().to_path_buf()
    }

    pub fn path(&self) -> PathBuf {
        self.repo.clone()
    }
}
