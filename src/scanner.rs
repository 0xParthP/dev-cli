//! Repository discovery and scanning.
//!
//! Automatically discovers Git repositories in configured directories.
//!
//! # Future Implementation (Sprint 2+)
//!
//! This module will implement automatic project discovery by:
//! 1. Recursively scanning configured project root directories
//! 2. Finding directories containing `.git` subdirectories
//! 3. Building list of discoverable projects
//!
//! This will allow users to add project roots without manually listing each project.
//!
//! # Example (Future)
//!
//! ```no_run
//! # use anyhow::Result;
//! # fn example() -> Result<()> {
//! use dev_cli::scanner;
//! use std::path::PathBuf;
//!
//! let roots = vec![PathBuf::from("~/Projects")];
//! let projects = scanner::discover_projects(&roots);
//! println!("Found {} projects", projects.len());
//! # Ok(())
//! # }
//! ```
//!
//! # Current Status
//!
//! This is a placeholder for Sprint 2 implementation. Currently returns empty list.

#![allow(dead_code)]

use std::path::PathBuf;

/// Discover Git repositories in specified root directories.
///
/// **Note:** Currently a placeholder returning empty list. Full implementation
/// planned for Sprint 2.
///
/// When implemented, will recursively scan directories for `.git` folders
/// and return paths to discovered repositories.
///
/// # Arguments
///
/// * `_roots` — Directories to scan for projects
///
/// # Returns
///
/// Vector of paths to discovered projects. Currently empty (placeholder).
///
/// # Future Enhancements
///
/// - Recursive directory traversal
/// - `.gitignore` support to skip excluded directories
/// - Configurable max depth
/// - Caching for performance
/// - Watcher for changes
pub fn discover_projects(_roots: &[PathBuf]) -> Vec<PathBuf> {
    Vec::new()
}
