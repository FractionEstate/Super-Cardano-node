//! Node address configuration for Super Cardano Node
//!
//! Handles configuration for node network addresses.

use serde::{Serialize, Deserialize};

/// Node network address configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NodeAddressConfig {
    /// Host or IP address to bind.
    pub host: String,
    /// Port to bind.
    pub port: u16,
}
