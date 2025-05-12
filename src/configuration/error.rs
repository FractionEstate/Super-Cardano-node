use thiserror::Error;

/// Custom error types for configuration.
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Failed to load configuration file: {0}")]
    FileLoadError(String),

    #[error("Environment variable error: {0}")]
    EnvVarError(String),
}
