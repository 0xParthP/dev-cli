#![allow(dead_code)]

use std::path::{Path, PathBuf};

use tempfile::TempDir;

/// Temporary workspace containing one or more fake Git repositories.
///
/// Used by integration tests for the scanner and project commands.
pub struct TempProject {
    root: TempDir,
}

impl TempProject {
    /// Creates a temporary workspace.
    pub fn new(_name: &str) -> Self {
        unsafe {
            std::env::set_var("DEVCLI_SKIP_ONBOARDING", "1");
        }
        Self { root: tempfile::tempdir().unwrap() }
    }

    /// Returns the workspace root.
    pub fn root(&self) -> PathBuf {
        self.root.path().to_path_buf()
    }

    /// Creates a fake Git repository inside the workspace.
    ///
    /// This creates:
    ///
    /// ```text
    /// workspace/
    /// └── <name>/
    ///     └── .git/
    /// ```
    pub fn create_git_repo(&self, name: &str) -> PathBuf {
        let repo = self.root.path().join(name);

        std::fs::create_dir_all(repo.join(".git")).unwrap();

        repo
    }

    /// Returns the workspace path.
    pub fn path(&self) -> &Path {
        self.root.path()
    }
}
