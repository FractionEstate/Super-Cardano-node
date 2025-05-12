use crate::ledger::Block;
use async_trait::async_trait;
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
/// Networking module for Super Cardano Node
///
/// Handles async P2P networking, peer discovery, block/tx propagation, and DoS resistance.
///
/// Uses Tokio for async I/O and leverages configuration and tracing modules.
pub mod discovery;
pub mod error;
pub mod p2p;
pub mod peer;
pub mod protocols;

use crate::configuration::NetworkConfig;
use crate::tracing::Tracing;
use anyhow::Result;

use crate::networking::peer::PeerManager;
/// Main entry point for the networking subsystem.
use std::sync::Arc;

/// Main entry point for the networking subsystem.
pub struct Network {
    pub config: NetworkConfig,
    pub tracer: Tracing,
    pub peer_manager: Arc<PeerManager>,
}

impl Network {
    /// Create a new networking subsystem with the given configuration and tracer.

    /// Create a new networking subsystem with the given configuration and tracer.
    pub fn new(config: NetworkConfig, tracer: Tracing) -> Result<Self> {
        let peer_manager = Arc::new(PeerManager::new());
        Ok(Self {
            config,
            tracer,
            peer_manager,
        })
    }

    /// Start the networking event loop (async)
    /// Start the networking event loop (async)
    pub async fn run(&self) -> Result<()> {
        use std::net::SocketAddr;
        use tokio::net::TcpListener;
        let addr: SocketAddr = self.config.bind_addr.parse().expect("Invalid bind_addr");
        let listener = TcpListener::bind(addr)
            .await
            .expect("Failed to bind TCP listener");
        self.tracer.startup();
        println!(
            "[Networking] Listening on {} (max peers: {})",
            addr, self.config.max_peers
        );

        // Peer discovery: connect to static peers if provided
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
                println!("[Networking] uPnP peer discovery not yet implemented");
            }
            _ => {}
        }

        loop {
            if self.peer_manager.peer_count() >= self.config.max_peers {
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                continue;
            }
            match listener.accept().await {
                Ok((socket, peer_addr)) => {
                    println!("[Networking] Accepted connection from {}", peer_addr);
                    // TODO: Spawn a task to handle the peer connection and message relay
                    // self.peer_manager.add_peer(peer_addr, ...);
                    // Optionally: integrate with tracing
                    self.tracer.metric("peer_connected", 1.0);
                }
                Err(e) => {
                    eprintln!("[Networking] Accept error: {}", e);
                }
            }
        }
    }
}
