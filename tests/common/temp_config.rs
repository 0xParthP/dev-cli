use dev_cli::{config::Config, models::ide::Ide};
use tempfile::TempDir;

#[allow(dead_code)]
pub fn test_config() -> Config {
    Config {
        projects_root: vec![TempDir::new().unwrap().path().to_path_buf()],
        default_ide: Ide::Vscode,
        recent_projects: Vec::new(),
    }
}

#[allow(dead_code)]
pub fn with_temp_config<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let dir = TempDir::new().unwrap();

    unsafe {
        std::env::set_var("DEVCLI_CONFIG_DIR", dir.path());
    }

    let result = f();

    unsafe {
        std::env::remove_var("DEVCLI_CONFIG_DIR");
    }

    result
}
