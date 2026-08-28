use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub path: PathBuf,

    pub language: Option<String>,
    pub framework: Option<String>,

    pub branch: Option<String>,

    pub dirty: bool,
}