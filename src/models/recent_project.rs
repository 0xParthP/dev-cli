//! Recently opened project.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecentProject {
    /// Project name.
    pub name: String,

    /// Absolute project path.
    pub path: PathBuf,

    /// Unix timestamp (seconds).
    pub last_opened: u64,
}
