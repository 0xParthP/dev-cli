use dev_cli::{config::Config, models::ide::Ide};
use temp_env::with_var;
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
    with_var("DEVCLI_CONFIG_DIR", Some(dir.path()), f)
}
