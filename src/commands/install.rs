use anyhow::Result;

use crate::installer;

pub fn execute() -> Result<()> {
    installer::install()
}