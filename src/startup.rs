use anyhow::Result;

use crate::{
    config::Config,
    onboarding,
};

pub fn ensure_initialized() -> Result<()> {
    // CI/tests can skip onboarding.
    if std::env::var("DEVCLI_SKIP_ONBOARDING").is_ok() {
        return Ok(());
    }

    if Config::exists()? {
        return Ok(());
    }

    onboarding::run()?;

    Ok(())
}