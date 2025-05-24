use crate::configuration::types::Configuration;
use anyhow::Result;
use serde_yaml;
use std::fs;

/// Loads configuration from a YAML file.
pub fn load_config_file() -> Result<Configuration> {
    let file_content = fs::read_to_string("config.yaml")?;
    let config: Configuration = serde_yaml::from_str(&file_content)?;
    Ok(config)
}
