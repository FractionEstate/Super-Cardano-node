//! Network topology configuration for Super Cardano Node
//!
//! Handles configuration for network topology and peer discovery.

use serde::{Serialize, Deserialize};

/// Network topology configuration for peer discovery and connectivity.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TopologyConfig {
    /// List of static peer addresses.
    pub static_peers: Vec<String>,
    /// Enable DNS-based peer discovery.
    pub dns_seeds: Vec<String>,
    /// Enable mDNS peer discovery.
    pub mdns: bool,
}
