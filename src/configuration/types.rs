use serde::{Deserialize, Serialize};

/// Represents the main configuration structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    pub network: NetworkConfig,
    pub database: DatabaseConfig,
    pub consensus: ConsensusConfig,
    pub logging: LoggingConfig,
}

/// Network-related configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub listen_address: String,
    pub port: u16,
    pub max_peers: usize,
}

/// Database-related configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub path: String,
    pub cache_size: usize,
}

/// Consensus-related configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusConfig {
    pub protocol: String,
    pub slot_duration: u64,
}

/// Logging-related configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub file: Option<String>,
}
