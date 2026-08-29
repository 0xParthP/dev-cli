//! Repository discovery engine.
//!
//! This module recursively walks configured project roots looking for Git
//! repositories and returns a collection of [`Project`] values.
//!
//! The scanner is intentionally filesystem-only. It does **not** inspect Git
//! metadata such as branches or remotes—that happens in Sprint 3.
//!
//! # Example
//!
//! ```no_run
//! use std::path::PathBuf;
//! use dev_cli::scanner::discover_projects;
//!
//! let roots = vec![PathBuf::from("C:/Users/parth/Documents/projects")];
//! let projects = discover_projects(&roots).unwrap();
//!
//! println!("Found {} repositories.", projects.len());
//! ```

use anyhow::Result;
use ignore::WalkBuilder;
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use crate::models::project::Project;

/// Directory names that should never be scanned.
///
/// These are typically dependency directories or build outputs that may contain
/// nested `.git` folders which are not real developer projects.
const IGNORED_DIRS: &[&str] =
    &[".git", "target", "node_modules", ".venv", "venv", "build", "dist", ".idea", ".vscode"];

/// Discover every Git repository beneath one or more configured project roots.
///
/// Duplicate repositories are removed using canonical filesystem paths, and the
/// returned list is sorted alphabetically by project name.
///
/// # Errors
///
/// Returns an error only if a discovered repository cannot be canonicalised.
pub fn discover_projects(roots: &[PathBuf]) -> Result<Vec<Project>> {
    let mut projects = Vec::new();
    let mut seen = HashSet::new();

    for root in roots {
        if !root.exists() {
            continue;
        }

        scan_root(root, &mut projects, &mut seen)?;
    }

    projects.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(projects)
}

/// Scan a single configured root for Git repositories.
fn scan_root(root: &Path, projects: &mut Vec<Project>, seen: &mut HashSet<PathBuf>) -> Result<()> {
    let walker = WalkBuilder::new(root)
        .hidden(false)
        .git_ignore(true)
        .git_exclude(true)
        .git_global(true)
        .filter_entry(|entry| {
            let name = entry.file_name().to_string_lossy();
            !IGNORED_DIRS.contains(&name.as_ref())
        })
        .build();

    for entry in walker {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        let path = entry.path();

        if is_git_repo(path) {
            let repo_root = path.canonicalize()?;

            if seen.insert(repo_root.clone()) {
                projects.push(Project::new(repo_root, root.to_path_buf()));
            }
        }
    }

    Ok(())
}

/// Returns `true` if the provided path is the root of a Git repository.
fn is_git_repo(path: &Path) -> bool {
    path.join(".git").is_dir()
}
