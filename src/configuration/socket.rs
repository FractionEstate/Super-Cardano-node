//! Socket configuration for Super Cardano Node
//!
//! Handles configuration for node socket communication.

use serde::{Serialize, Deserialize};

/// Socket configuration for node communication.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SocketConfig {
    /// Path to the node socket file (for IPC).
    pub socket_path: String,
    /// Enable TCP fallback if IPC is unavailable.
    pub tcp_fallback: bool,
}
