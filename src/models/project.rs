use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[expect(dead_code, reason = "Used by the repository scanner in Sprint 2")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub path: PathBuf,

    pub language: Option<String>,
    pub framework: Option<String>,

    pub branch: Option<String>,

    pub dirty: bool,
}
