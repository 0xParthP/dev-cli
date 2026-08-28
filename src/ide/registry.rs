use std::path::PathBuf;

use crate::models::ide::Ide;

#[derive(Debug, Clone)]
pub struct InstalledIde {
    pub ide: Ide,
    pub display_name: String,
    pub executable: PathBuf,
}

impl InstalledIde {
    pub fn new(ide: Ide, name: &str, exe: PathBuf) -> Self {
        Self { ide, display_name: name.into(), executable: exe }
    }
}
