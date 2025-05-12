use crate::configuration::types::Configuration;
use anyhow::Result;
use std::env;

/// Loads configuration from environment variables.
pub fn load_env_vars() -> Result<Configuration> {
    let config = Configuration {
        network: NetworkConfig {
            listen_address: env::var("LISTEN_ADDRESS").unwrap_or("0.0.0.0".to_string()),
            port: env::var("PORT").unwrap_or("3000".to_string()).parse()?,
            max_peers: env::var("MAX_PEERS").unwrap_or("100".to_string()).parse()?,
        },
        database: DatabaseConfig {
            path: env::var("DB_PATH").unwrap_or("./data".to_string()),
            cache_size: env::var("CACHE_SIZE").unwrap_or("1024".to_string()).parse()?,
        },
        consensus: ConsensusConfig {
            protocol: env::var("CONSENSUS_PROTOCOL").unwrap_or("Ouroboros".to_string()),
            slot_duration: env::var("SLOT_DURATION").unwrap_or("1000".to_string()).parse()?,
        },
        logging: LoggingConfig {
            level: env::var("LOG_LEVEL").unwrap_or("info".to_string()),
            file: env::var("LOG_FILE").ok(),
        },
    };

    Ok(config)
}
