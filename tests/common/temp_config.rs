#![allow(dead_code)]
use std::path::PathBuf;

use dev_cli::{config::Config, models::ide::Ide};

/// Returns a valid test configuration.
pub fn test_config() -> Config {
    Config { projects_root: vec![PathBuf::from("C:/Projects")], default_ide: Ide::Vscode }
}
