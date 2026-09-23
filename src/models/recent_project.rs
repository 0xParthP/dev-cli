//! Recently opened project.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::models::ide::Ide;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecentProject {
    /// Project name.
    pub name: String,

    /// Absolute project path.
    pub path: PathBuf,

    /// Unix timestamp (seconds).
    pub last_opened: u64,

    /// Last IDE used to open the project.
    #[serde(default)]
    pub ide: Option<Ide>,
}
