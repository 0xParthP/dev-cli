use std::path::PathBuf;

use dev_cli::models::project::Project;

/// Creates a fake project for testing pure logic without filesystem reads.
#[allow(dead_code)]
pub fn fake_project(name: &str) -> Project {
    let root = PathBuf::from("/projects");
    let path = root.join(name);

    Project { name: name.into(), path: path.clone(), root, git_dir: path.join(".git") }
}

/// Creates a fake project that actually exists on disk in a temp directory.
#[allow(dead_code)]
pub fn temp_project(name: &str) -> Project {
    let root = std::env::temp_dir();
    let path = root.join(name);

    // Ensure the fake project directory actually exists.
    std::fs::create_dir_all(path.join(".git")).unwrap();

    Project { name: name.into(), path: path.clone(), root, git_dir: path.join(".git") }
}
