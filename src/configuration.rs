//! Configuration and CLI entrypoint for Super Cardano Node
//!
//! Mirrors the original Cardano Node's configuration and CLI structure.
//! Uses serde for config files and clap for CLI parsing.

use anyhow::{Context, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
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
    /// Network to connect to [mainnet, testnet, preview]
    #[clap(long, value_parser, default_value = "testnet")]
    pub network: String,
    /// Database path
    #[clap(long, value_parser)]
    pub db: Option<PathBuf>,
    /// Socket address to bind for REST API
    #[clap(long, value_parser)]
    pub rest_api: Option<String>,
    /// Socket address to bind for gRPC API
    #[clap(long, value_parser)]
    pub grpc_api: Option<String>,
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
    /// Mempool configuration
    pub mempool: MempoolConfig,
    /// ChainDB configuration
    pub chaindb: ChainDBConfig,
    /// API configuration
    pub api: ApiConfig,
    /// Runtime configuration
    pub runtime: RuntimeConfig,
    /// Wallet configuration
    pub wallet: WalletConfig,
    /// Metrics configuration
    pub metrics: MetricsConfig,
}

/// Example network config (expand as needed)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetworkConfig {
    /// Address to bind the node's networking interface.
    pub bind_addr: String,
    /// Maximum number of peers.
    pub max_peers: usize,
    /// Peer discovery configuration.
    pub discovery: Option<DiscoveryConfig>,
    /// Handshake timeout in seconds.
    pub handshake_timeout_secs: u64,
    /// List of bootstrap peers.
    pub bootstrap_peers: Vec<String>,
    /// Peer limits configuration.
    pub peer_limits: PeerLimitsConfig,
    /// Optional node ID.
    pub node_id: Option<String>,
}

/// Peer discovery method
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DiscoveryMethod {
    #[serde(rename = "static")]
    Static,
    #[serde(rename = "dns")]
    Dns,
    #[serde(rename = "upnp")]
    Upnp,
}

/// Peer discovery configuration
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiscoveryConfig {
    /// Discovery method (static, DNS, uPnP).
    pub method: DiscoveryMethod,
    /// List of DNS seeds for discovery.
    pub dns_seeds: Vec<String>,
}

/// Peer limits configuration
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PeerLimitsConfig {
    /// Maximum number of inbound peers.
    pub max_inbound: usize,
    /// Maximum number of outbound peers.
    pub max_outbound: usize,
    /// Maximum number of pending connections.
    pub max_pending: usize,
}

/// Example consensus config (expand as needed)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConsensusConfig {
    /// Consensus protocol name (e.g., Ouroboros).
    pub protocol: String,
    /// Path to the genesis file.
    pub genesis_file: PathBuf,
    /// Slot duration in milliseconds.
    pub slot_duration_ms: u64,
    /// Security parameter k.
    pub security_param_k: u64,
    /// Active slots coefficient.
    pub active_slots_coeff: f64,
}

/// Example protocol config (expand as needed)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProtocolConfig {
    /// Protocol era (e.g., Byron, Shelley, Alonzo).
    pub era: String,
    /// Network magic number.
    pub network_magic: u32,
    /// Path to the protocol parameters file.
    pub protocol_params: PathBuf,
    /// Maximum transaction size.
    pub max_tx_size: usize,
    /// Maximum block size.
    pub max_block_size: usize,
}

/// Mempool configuration
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MempoolConfig {
    /// Mempool capacity.
    pub capacity: usize,
    /// Maximum transaction age in slots.
    pub max_tx_age_slots: u64,
    /// Garbage collection interval in seconds.
    pub garbage_collection_interval: u64,
}

/// ChainDB configuration
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChainDBConfig {
    /// Path to the ChainDB directory.
    pub path: PathBuf,
    /// Maximum number of orphaned blocks.
    pub max_orphans: usize,
    /// Pruning configuration.
    pub pruning: PruningConfig,
}

/// Pruning configuration
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PruningConfig {
    /// Pruning mode (archive or recent).
    pub mode: PruningMode,
    /// Number of recent slots to keep (if in recent mode).
    pub keep_recent: Option<u32>,
}

/// Pruning mode
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum PruningMode {
    #[serde(rename = "archive")]
    Archive,
    #[serde(rename = "recent")]
    Recent,
}

/// API configuration
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiConfig {
    /// REST API configuration
    pub rest: RestApiConfig,
    /// gRPC API configuration
    pub grpc: GrpcApiConfig,
}

/// REST API configuration
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RestApiConfig {
    /// Enable or disable the REST API
    pub enabled: bool,
    /// Socket address to bind the REST API
    pub bind_addr: String,
    /// List of allowed CORS origins
    pub cors_origins: Vec<String>,
    /// Rate limit configuration
    pub rate_limit: Option<RateLimitConfig>,
}

/// gRPC API configuration
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GrpcApiConfig {
    /// Enable or disable the gRPC API
    pub enabled: bool,
    /// Socket address to bind the gRPC API
    pub bind_addr: String,
    /// TLS configuration (optional)
    pub tls: Option<TlsConfig>,
}

/// Rate limit configuration
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RateLimitConfig {
    /// Maximum number of requests per second
    pub requests_per_second: u32,
    /// Burst capacity for rate limiting
    pub burst: u32,
}

/// TLS configuration
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TlsConfig {
    /// Path to the TLS certificate file
    pub cert_path: PathBuf,
    /// Path to the TLS key file
    pub key_path: PathBuf,
}

/// Runtime configuration
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RuntimeConfig {
    /// Number of worker threads (optional)
    pub worker_threads: Option<usize>,
    /// Enable or disable metrics
    pub metrics_enabled: bool,
}

/// Wallet configuration
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WalletConfig {
    /// Path to the wallet directory
    pub path: PathBuf,
    /// Enable or disable automatic wallet synchronization
    pub auto_sync: bool,
}

/// Metrics configuration
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetricsConfig {
    /// Enable or disable metrics collection
    pub enabled: bool,
    /// Prometheus endpoint for metrics
    pub prometheus_endpoint: Option<String>,
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
            if discovery.dns_seeds.is_empty() {
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

    /// Override config with CLI options
    pub fn apply_cli_overrides(&mut self, cli: &Cli) {
        if let Some(db_path) = &cli.db {
            self.chaindb.path = db_path.clone();
        }

        if let Some(rest_api) = &cli.rest_api {
            self.api.rest.bind_addr = rest_api.clone();
        }

        if let Some(grpc_api) = &cli.grpc_api {
            self.api.grpc.bind_addr = grpc_api.clone();
        }

        // Apply network preset if specified
        match cli.network.as_str() {
            "mainnet" => self.apply_mainnet_preset(),
            "testnet" => self.apply_testnet_preset(),
            "preview" => self.apply_preview_preset(),
            _ => {}
        }
    }

    fn apply_mainnet_preset(&mut self) {
        self.protocol.network_magic = 764824073;
        // Additional mainnet-specific settings
    }

    fn apply_testnet_preset(&mut self) {
        self.protocol.network_magic = 1097911063;
        // Additional testnet-specific settings
    }

    fn apply_preview_preset(&mut self) {
        self.protocol.network_magic = 2;
        // Additional preview-specific settings
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            bind_addr: "0.0.0.0:3001".to_string(),
            max_peers: 32,
            discovery: None,
            handshake_timeout_secs: 15,
            bootstrap_peers: vec![],
            peer_limits: PeerLimitsConfig {
                max_inbound: 32,
                max_outbound: 16,
                max_pending: 32,
            },
            node_id: None,
        }
    }
}

impl Default for ConsensusConfig {
    fn default() -> Self {
        Self {
            protocol: "Ouroboros".to_string(),
            genesis_file: PathBuf::from("testnet-genesis.json"),
            slot_duration_ms: 1000,
            security_param_k: 2160,
            active_slots_coeff: 0.05,
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
                network_magic: 1097911063, // testnet
                protocol_params: PathBuf::from("testnet-params.json"),
                max_tx_size: 16384,
                max_block_size: 65536,
            },
            mempool: MempoolConfig {
                capacity: 10000,
                max_tx_age_slots: 20,
                garbage_collection_interval: 100,
            },
            chaindb: ChainDBConfig {
                path: PathBuf::from("./data/chaindb"),
                max_orphans: 1000,
                pruning: PruningConfig {
                    mode: PruningMode::Recent,
                    keep_recent: Some(2160),
                },
            },
            api: ApiConfig {
                rest: RestApiConfig {
                    enabled: true,
                    bind_addr: "127.0.0.1:8080".to_string(),
                    cors_origins: vec!["*".to_string()],
                    rate_limit: Some(RateLimitConfig {
                        requests_per_second: 100,
                        burst: 200,
                    }),
                },
                grpc: GrpcApiConfig {
                    enabled: true,
                    bind_addr: "127.0.0.1:50051".to_string(),
                    tls: None,
                },
            },
            runtime: RuntimeConfig {
                worker_threads: None,
                metrics_enabled: false,
            },
            wallet: WalletConfig {
                path: PathBuf::from("./data/wallet"),
                auto_sync: true,
            },
            metrics: MetricsConfig {
                enabled: false,
                prometheus_endpoint: Some("127.0.0.1:9091".to_string()),
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
        assert_eq!(config.network.discovery.as_ref().unwrap().dns_seeds.len(), 2);
        assert_eq!(config.network.discovery.as_ref().unwrap().dns_seeds[0], "127.0.0.1:3002");
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
