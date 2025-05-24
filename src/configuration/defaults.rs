use crate::configuration::types::Configuration;
use crate::configuration::{ConsensusConfig, DatabaseConfig, LoggingConfig, NetworkConfig};

/// Provides default configuration values.
pub fn default_configuration() -> Configuration {
    Configuration {
        network: NetworkConfig {
            listen_address: "0.0.0.0".to_string(),
            bind_addr: "0.0.0.0:3000".to_string(),
            discovery: "static".to_string(),
            port: 3000,
            max_peers: 100,
        },
        database: DatabaseConfig {
            path: "./data".to_string(),
            cache_size: 1024,
        },
        consensus: ConsensusConfig {
            protocol: "Ouroboros".to_string(),
            slot_duration: 1000,
        },
        logging: LoggingConfig {
            level: "info".to_string(),
            file: None,
        },
    }
}
