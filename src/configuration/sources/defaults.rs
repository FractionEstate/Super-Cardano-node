use crate::configuration::{NetworkConfig, DatabaseConfig, ConsensusConfig, LoggingConfig};
// src/configuration/sources/defaults.rs
// Minimal stub for configuration defaults source

use crate::configuration::types::*;

use crate::configuration::types::Configuration;

/// Loads default configuration values.
pub fn load_defaults() -> Configuration {
    Configuration {
        network: NetworkConfig {
            listen_address: "0.0.0.0".to_string(),
            port: 3000,
            max_peers: 100,
            bind_addr: "0.0.0.0:3000".to_string(),
            discovery: "static".to_string(),
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
