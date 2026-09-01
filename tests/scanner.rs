mod common;

use std::path::PathBuf;

use common::temp_project::TempProject;
use dev_cli::scanner::discover_projects;

#[test]
fn empty_directory_returns_no_projects() {
    let root = TempProject::new("empty");

    let projects = discover_projects(&[root.root().to_path_buf()]).unwrap();

    assert!(projects.is_empty());
}

#[test]
fn discovers_single_git_repository() {
    let root = TempProject::new("workspace");
    let repo = root.create_git_repo("demo");

    let projects = discover_projects(&[root.root().to_path_buf()]).unwrap();

    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].name, "demo");

    // Windows returns \\?\ paths, so canonicalise both sides.
    assert_eq!(projects[0].path.canonicalize().unwrap(), repo.canonicalize().unwrap());
}

#[test]
fn discovers_multiple_repositories() {
    let root = TempProject::new("workspace");

    root.create_git_repo("alpha");
    root.create_git_repo("beta");
    root.create_git_repo("gamma");

    let projects = discover_projects(&[root.root().to_path_buf()]).unwrap();

    let mut names: Vec<String> = projects.into_iter().map(|p| p.name).collect();
    names.sort();

    assert_eq!(names, vec!["alpha", "beta", "gamma"]);
}

#[test]
fn ignores_node_modules() {
    let root = TempProject::new("workspace");

    root.create_git_repo("real-project");

    let fake_repo = root.root().join("node_modules").join("ignored-package").join(".git");

    std::fs::create_dir_all(fake_repo).unwrap();

    let projects = discover_projects(&[root.root().to_path_buf()]).unwrap();

    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].name, "real-project");
}

#[test]
fn duplicate_roots_do_not_duplicate_projects() {
    let root = TempProject::new("workspace");

    root.create_git_repo("shared");

    let roots: Vec<PathBuf> = vec![root.root().to_path_buf(), root.root().to_path_buf()];

    let projects = discover_projects(&roots).unwrap();

    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].name, "shared");
}
