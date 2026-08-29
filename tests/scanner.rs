mod common;

use common::temp_project::TempProject;
use dev_cli::scanner::discover_projects;
use std::fs;
use tempfile::TempDir;

#[test]
fn empty_directory_returns_no_projects() {
    let dir = TempDir::new().unwrap();

    let projects = discover_projects(&[dir.path().to_path_buf()]).unwrap();

    assert!(projects.is_empty());
}

#[test]
fn discovers_single_git_repository() {
    let repo = TempProject::new("demo");

    let projects = discover_projects(&[repo.root()]).unwrap();

    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].name, "demo");
    assert_eq!(projects[0].path, repo.path().canonicalize().unwrap());
}

#[test]
fn discovers_multiple_repositories() {
    let dir = TempDir::new().unwrap();

    fs::create_dir_all(dir.path().join("repo1/.git")).unwrap();
    fs::create_dir_all(dir.path().join("repo2/.git")).unwrap();
    fs::create_dir_all(dir.path().join("repo3/.git")).unwrap();

    let projects = discover_projects(&[dir.path().to_path_buf()]).unwrap();

    assert_eq!(projects.len(), 3);
}

#[test]
fn ignores_node_modules() {
    let dir = TempDir::new().unwrap();

    fs::create_dir_all(dir.path().join("node_modules/repo/.git")).unwrap();

    let projects = discover_projects(&[dir.path().to_path_buf()]).unwrap();

    assert!(projects.is_empty());
}

#[test]
fn duplicate_roots_do_not_duplicate_projects() {
    let repo = TempProject::new("repo");

    let root = repo.root();

    let projects = discover_projects(&[root.clone(), root]).unwrap();

    assert_eq!(projects.len(), 1);
}
