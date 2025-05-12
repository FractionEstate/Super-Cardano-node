impl Configuration {
    pub fn merge(self, other: Self) -> Self {
        // Naive merge: prefer non-defaults from `other` over `self`
        // (real logic should be more sophisticated)
        other
    }
}
/// Peer discovery method for networking.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscoveryMethod {
    Static,
    Dns,
    Upnp,
}
use serde::{Deserialize, Serialize};

/// Represents the main configuration structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    pub network: NetworkConfig,
    pub database: DatabaseConfig,
    pub consensus: ConsensusConfig,
    pub logging: LoggingConfig,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            network: NetworkConfig::default(),
            database: DatabaseConfig {
                path: "./data/chaindb".to_string(),
                cache_size: 1024,
            },
            consensus: ConsensusConfig {
                protocol: "OuroborosPraos".to_string(),
                slot_duration: 1000,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                file: None,
            },
        }
    }
}

/// Network-related configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkConfig {
    pub listen_address: String,
    pub port: u16,
    pub max_peers: usize,
    pub bind_addr: String,
    pub discovery: String,
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
