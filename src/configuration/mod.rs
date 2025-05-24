pub mod types;
pub mod sources;
pub mod validation;
pub mod defaults;
pub mod error;

// Re-export commonly used items
pub use types::*;
pub use sources::*;
pub use validation::*;
pub use defaults::*;
pub use error::*;

// Main entry point for loading configuration
use anyhow::Result;

/// Loads the configuration from all sources (CLI, file, env).
pub fn load_configuration() -> Result<types::Configuration> {
    let cli_config = sources::cli::load_cli_args()?;
    let file_config = sources::file::load_config_file()?;
    let env_config = sources::env::load_env_vars()?;

    // Merge configurations with priority: CLI > Env > File
    let merged_config = cli_config.merge(env_config).merge(file_config);

    // Validate the final configuration
    validation::validate(&merged_config)?;

    Ok(merged_config)
}
