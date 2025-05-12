use crate::configuration::types::Configuration;
use anyhow::{Result, anyhow};

/// Validates the configuration.
pub fn validate(config: &Configuration) -> Result<()> {
    if config.network.port == 0 {
        return Err(anyhow!("Port number cannot be 0"));
    }

    if config.database.cache_size == 0 {
        return Err(anyhow!("Cache size must be greater than 0"));
    }

    Ok(())
}
