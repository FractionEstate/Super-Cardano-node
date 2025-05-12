use crate::configuration::types::Configuration;
use crate::configuration::{ConsensusConfig, DatabaseConfig, LoggingConfig, NetworkConfig};
use anyhow::Result;
use clap::{Arg, Command};

/// Loads configuration from CLI arguments.
pub fn load_cli_args() -> Result<Configuration> {
    let matches = Command::new("Cardano Node")
        .version("1.0")
        .author("FractionEstate")
        .about("A high-performance Cardano Node implementation")
        .arg(
            Arg::new("network")
                .long("network")
                .help("Specifies the network type"),
        )
        .arg(
            Arg::new("port")
                .long("port")
                .help("Specifies the port to listen on"),
        )
        .get_matches();

    // Example: Parse CLI arguments into a Configuration struct
    // TODO: Parse all CLI arguments and map to config fields as needed
    let port = matches
        .get_one::<String>("port")
        .and_then(|s| s.parse().ok())
        .unwrap_or(3000);
    let network = matches
        .get_one::<String>("network")
        .cloned()
        .unwrap_or_else(|| "mainnet".to_string());

    let config = Configuration {
        network: NetworkConfig {
            listen_address: "0.0.0.0".to_string(),
            port,
            max_peers: 100,
            bind_addr: format!("0.0.0.0:{}", port),
            discovery: network,
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
    };

    Ok(config)
}
