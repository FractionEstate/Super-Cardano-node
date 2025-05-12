//! P2P topology configuration for Super Cardano Node
//!
//! Handles configuration for peer-to-peer network topology.

use serde::{Serialize, Deserialize};

/// P2P topology configuration for peer-to-peer networking.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TopologyP2PConfig {
    /// Enable P2P mode.
    pub enabled: bool,
    /// List of trusted relay nodes.
    pub trusted_relays: Vec<String>,
    /// Maximum peer connections.
    pub max_peers: usize,
}
