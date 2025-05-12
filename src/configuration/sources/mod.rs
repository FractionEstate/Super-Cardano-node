pub mod cli;
pub mod file;
pub mod env;

// Common trait for configuration sources
use crate::configuration::types::Configuration;
use anyhow::Result;

/// Trait for loading configuration from a source.
pub trait ConfigSource {
    fn load() -> Result<Configuration>;
}
