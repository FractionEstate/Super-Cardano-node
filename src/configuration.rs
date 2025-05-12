//! Configuration and CLI entrypoint for Super Cardano Node
//!
//! Mirrors the original Cardano Node's configuration and CLI structure.
//! Uses serde for config files and clap for CLI parsing.

use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use serde_json;

/// Command-line arguments for the node.
#[derive(Parser, Debug)]
#[command(name = "super-cardano-node")]
pub struct Cli {
    /// Path to the configuration file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,
    /// Print version and exit
    #[arg(long)]
    pub version: bool,
    // ... add more CLI options as needed
}

/// Node configuration loaded from file.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    /// Network configuration
    pub network: NetworkConfig,
    /// Consensus configuration
    pub consensus: ConsensusConfig,
    /// Protocol configuration
    pub protocol: ProtocolConfig,
    // ... add more fields as needed
}

/// Example network config (expand as needed)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetworkConfig {
    /// Address to bind the node's networking interface.
    pub bind_addr: String,
    /// Maximum number of peers.
    pub max_peers: usize,
    /// Peer discovery configuration.
    pub discovery: Option<PeerDiscoveryConfig>,
}

/// Peer discovery method
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PeerDiscoveryMethod {
    Static,
    Dns,
    Mdns,
}

/// Peer discovery configuration
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PeerDiscoveryConfig {
    /// Discovery method (static, DNS, mDNS).
    pub method: PeerDiscoveryMethod,
    /// List of peer addresses or DNS seeds.
    pub peers: Vec<String>,
}

/// Example consensus config (expand as needed)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConsensusConfig {
    /// Consensus protocol name (e.g., Ouroboros).
    pub protocol: String,
}

/// Example protocol config (expand as needed)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProtocolConfig {
    /// Protocol era (e.g., Byron, Shelley, Alonzo).
    pub era: String,
}

#[allow(dead_code)]
impl Config {
    /// Load configuration from a JSON file
    pub fn load_from_file(path: &PathBuf) -> Result<Self, String> {
        let content = fs::read_to_string(path).map_err(|e| format!("Failed to read config: {}", e))?;
        serde_json::from_str(&content).map_err(|e| format!("Failed to parse JSON: {}", e))
    }
    #[allow(dead_code)]
    /// Validate the loaded configuration
    pub fn validate(&self) -> bool {
        // Validate network config
        if self.network.bind_addr.is_empty() || self.network.max_peers == 0 {
            return false;
        }
        // Validate consensus config
        if self.consensus.protocol.is_empty() {
            return false;
        }
        // Validate protocol config
        if self.protocol.era.is_empty() {
            return false;
        }
        // Optionally: validate discovery peers if present
        if let Some(discovery) = &self.network.discovery {
            if discovery.peers.is_empty() {
                return false;
            }
        }
        true
    }
    #[allow(dead_code)]
    /// Parse configuration from CLI arguments and file
    pub fn from_cli() -> Self {
        let cli = Cli::parse();
        let mut config = Self::default();
        if let Some(config_path) = cli.config {
            config = Self::load_from_file(&config_path).unwrap_or_else(|_| Self::default());
        }
        config
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            bind_addr: "0.0.0.0:3001".to_string(),
            max_peers: 32,
            discovery: None,
        }
    }
}

impl Default for ConsensusConfig {
    fn default() -> Self {
        Self {
            protocol: "Ouroboros".to_string(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            network: NetworkConfig::default(),
            consensus: ConsensusConfig::default(),
            protocol: ProtocolConfig {
                era: "byron".to_string(),
            },
        }
    }
}

// Move the test module to configuration.rs for correct access to types
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.network.bind_addr, "0.0.0.0:3001");
        assert_eq!(config.network.max_peers, 32);
        assert_eq!(config.consensus.protocol, "Ouroboros");
        assert_eq!(config.protocol.era, "byron");
    }

    #[test]
    fn test_load_config() {
        let json = r#"{
            "network": {"bind_addr": "127.0.0.1:3001", "max_peers": 8},
            "consensus": {"protocol": "Ouroboros"},
            "protocol": {"era": "Shelley"}
        }"#;
        let config: Result<Config, _> = serde_json::from_str(json);
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.network.bind_addr, "127.0.0.1:3001");
        assert_eq!(config.network.max_peers, 8);
        assert_eq!(config.consensus.protocol, "Ouroboros");
        assert_eq!(config.protocol.era, "Shelley");
    }

    #[test]
    fn test_peer_discovery_config() {
        let json = r#"{
            "network": {
                "bind_addr": "127.0.0.1:3001",
                "max_peers": 8,
                "discovery": {
                    "method": "Static",
                    "peers": ["127.0.0.1:3002", "127.0.0.1:3003"]
                }
            },
            "consensus": {"protocol": "Ouroboros"},
            "protocol": {"era": "Shelley"}
        }"#;
        let config: Result<Config, _> = serde_json::from_str(json);
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.network.discovery.as_ref().unwrap().peers.len(), 2);
        assert_eq!(config.network.discovery.as_ref().unwrap().peers[0], "127.0.0.1:3002");
    }

    #[test]
    fn test_validate_config() {
        let config = Config::default();
        assert!(config.validate());
    }

    #[test]
    fn test_config_parsing() {
        let json = r#"{
            "network": {"bind_addr": "127.0.0.1:3001", "max_peers": 8},
            "consensus": {"protocol": "Ouroboros"},
            "protocol": {"era": "Shelley"}
        }"#;
        let config: Result<Config, _> = serde_json::from_str(json);
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.network.bind_addr, "127.0.0.1:3001");
        assert_eq!(config.network.max_peers, 8);
        assert_eq!(config.consensus.protocol, "Ouroboros");
        assert_eq!(config.protocol.era, "Shelley");
    }
}
