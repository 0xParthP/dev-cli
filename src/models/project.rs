//! Project/repository type definitions.
//!
//! Defines the [`Project`] struct representing a discovered Git repository.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// A Git repository discovered by dev-cli.
///
/// Represents a single project that can be opened in an IDE.
/// Currently stores basic information; Sprint 2+ will add enhanced metadata.
///
/// # Fields
///
/// - `name` — Project directory name
/// - `path` — Full path to project directory
/// - `language` — Primary language (optional, future use)
/// - `framework` — Used framework (optional, future use)
/// - `branch` — Current Git branch (optional, future use)
/// - `dirty` — Whether working directory has uncommitted changes (future use)

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
