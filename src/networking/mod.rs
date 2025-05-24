//! Networking module for Super Cardano Node
//!
//! Handles async P2P networking, peer discovery, block/tx propagation, and DoS resistance.
//!
//! Uses Tokio for async I/O and leverages configuration and tracing modules.

use crate::ledger::Block;
use async_trait::async_trait;
use crate::configuration::NetworkConfig;
use crate::tracing::Tracing;
use anyhow::Result;
use crate::networking::peer::PeerManager;
use std::sync::Arc;

/// Extension trait for network operations.
#[async_trait]
pub trait NetworkExt {
    /// Broadcast a block to all connected peers.
    async fn broadcast_block(&self, block: &Block);
}

#[async_trait]
impl NetworkExt for Arc<Network> {
    async fn broadcast_block(&self, _block: &Block) {
        // TODO: Implement block broadcast logic
    }
}

/// Main entry point for the networking subsystem.
///
/// Manages peer connections, event loop, and network configuration.
#[derive(Debug)]
pub struct Network {
    /// Network configuration parameters.
    pub config: NetworkConfig,
    /// Tracing for network events.
    pub tracer: Tracing,
    /// Peer manager for handling peer connections.
    pub peer_manager: Arc<PeerManager>,
}

impl Network {
    /// Start the networking event loop (async)
    ///
    /// Binds to the configured address and listens for incoming peer connections.
    ///
    /// # Errors
    /// Returns an error if the address is invalid or the TCP listener fails to bind.
    pub async fn run(&self) -> Result<()> {
        use std::net::SocketAddr;
        use tokio::net::TcpListener;
        let addr: SocketAddr = self.config.bind_addr.parse()?;
        let listener = TcpListener::bind(addr).await?;
        self.tracer.startup();
        println!(
            "[Networking] Listening on {} (max peers: {})",
            addr, self.config.max_peers
        );

        // Peer discovery: connect to static peers if provided
        match self.config.discovery.as_str() {
            "static" => {
                // TODO: Load static peer addresses from config
                println!("[Networking] Static peer discovery not yet implemented");
            }
            "dns" => {
                println!("[Networking] DNS peer discovery not yet implemented");
            }
            "upnp" => {
                // TODO: Implement UPnP peer discovery
                println!("[Networking] UPnP peer discovery not yet implemented");
            }
            _ => {
                println!("[Networking] Unknown discovery mode");
            }
        }
        Ok(())
    }
}

pub mod discovery;
pub mod error;
pub mod p2p;
pub mod peer;
pub mod protocols;
