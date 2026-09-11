//! Project/repository type definitions.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Project {
    /// Human-readable repository name.
    ///
    /// Usually the final directory name.
    pub name: String,

    /// Absolute path to the repository root.
    pub path: PathBuf,

    /// Which configured project root this repository belongs to.
    pub root: PathBuf,

    /// Path to the `.git` directory.
    pub git_dir: PathBuf,
}

impl Project {
    /// Construct a project from discovered filesystem paths.
    pub fn new(path: PathBuf, root: PathBuf) -> Self {
        let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();

        let git_dir = path.join(".git");

        Self { name, path, root, git_dir }
    }
}
