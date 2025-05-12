use crate::configuration::types::Configuration;
use anyhow::Result;
use clap::{App, Arg};

/// Loads configuration from CLI arguments.
pub fn load_cli_args() -> Result<Configuration> {
    let matches = App::new("Cardano Node")
        .version("1.0")
        .author("FractionEstate")
        .about("A high-performance Cardano Node implementation")
        .arg(Arg::new("network")
            .long("network")
            .about("Specifies the network type")
            .takes_value(true))
        .arg(Arg::new("port")
            .long("port")
            .about("Specifies the port to listen on")
            .takes_value(true))
        .get_matches();

    // Example: Parse CLI arguments into a Configuration struct
    let config = Configuration {
        network: NetworkConfig {
            listen_address: "0.0.0.0".to_string(),
            port: matches.value_of("port").unwrap_or("3000").parse()?,
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
    };

    Ok(config)
}
