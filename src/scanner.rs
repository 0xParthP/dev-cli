//! Repository discovery engine.

use anyhow::Result;
use ignore::WalkBuilder;
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use crate::models::project::Project;

/// Directory names that should never be scanned.
const IGNORED_DIRS: &[&str] =
    &[".git", "target", "node_modules", ".venv", "venv", "build", "dist", ".idea", ".vscode"];

/// Discover every Git repository beneath one or more configured project roots.
pub fn discover_projects(roots: &[PathBuf]) -> Result<Vec<Project>> {
    let config = crate::config::Config::load().unwrap_or_default();
    discover_projects_with_config(roots, Some(config.max_depth), &config.ignore_patterns)
}

/// Discover Git repositories with explicit max depth and ignore patterns settings.
pub fn discover_projects_with_config(
    roots: &[PathBuf],
    max_depth: Option<usize>,
    ignore_patterns: &[String],
) -> Result<Vec<Project>> {
    let mut projects = Vec::new();
    let mut seen = HashSet::new();

    for root in roots {
        if !root.exists() {
            continue;
        }

        scan_root_opts(root, max_depth, ignore_patterns, &mut projects, &mut seen)?;
    }

    projects.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(projects)
}

/// Scan a single configured root for Git repositories with options.
fn scan_root_opts(
    root: &Path,
    max_depth: Option<usize>,
    ignore_patterns: &[String],
    projects: &mut Vec<Project>,
    seen: &mut HashSet<PathBuf>,
) -> Result<()> {
    let mut builder = WalkBuilder::new(root);
    builder.hidden(false).git_ignore(true).git_exclude(true).git_global(true);

    if let Some(depth) = max_depth {
        builder.max_depth(Some(depth));
    }

    let extra_ignores: Vec<String> = ignore_patterns.to_vec();

    let walker = builder
        .filter_entry(move |entry| {
            let name = entry.file_name().to_string_lossy();
            if IGNORED_DIRS.contains(&name.as_ref()) {
                return false;
            }
            if extra_ignores.iter().any(|p| p.eq_ignore_ascii_case(&name)) {
                return false;
            }
            true
        })
        .build();

    for entry in walker {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        let path = entry.path();

        if is_git_repo(path) {
            let repo_root = match path.canonicalize() {
                Ok(p) => p,
                Err(_) => continue,
            };

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
