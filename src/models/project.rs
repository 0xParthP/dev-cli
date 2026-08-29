//! Project/repository type definitions.
//!
//! Defines the [`Project`] struct representing a discovered Git repository.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    /// Project name (typically directory name).
    pub name: String,

    /// Full path to project directory.
    pub path: PathBuf,

    /// Primary programming language (Sprint 2+).
    pub language: Option<String>,

    /// Used framework or tool (Sprint 2+).
    pub framework: Option<String>,

    /// Current Git branch (Sprint 3+).
    pub branch: Option<String>,

    /// Whether working directory has uncommitted changes (Sprint 3+).
    pub dirty: bool,
}
