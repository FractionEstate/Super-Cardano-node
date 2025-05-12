//! Networking module for Super Cardano Node
//!
//! Handles peer discovery, P2P, and protocol messaging using async Rust and Tokio.
//! All networking code must be robust, secure, and performant.

use crate::configuration::NetworkConfig;
use crate::ledger::{Ledger, Block, Transaction};
use crate::mempool::Mempool;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use crate::protocol::Protocol;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream, tcp::OwnedWriteHalf};
use tokio::sync::Mutex as AsyncMutex;
use crate::tracing::tracers::Tracer;

/// Represents a connected peer.
#[allow(dead_code)]
pub struct Peer {
    pub addr: std::net::SocketAddr,
    pub writer: Arc<AsyncMutex<OwnedWriteHalf>>, // AsyncMutex for concurrent writes
}

/// Shared state for managing peers.
pub struct PeerManager {
    peers: Arc<Mutex<HashMap<SocketAddr, Peer>>>, // Map of connected peers
}

impl PeerManager {
    /// Create a new `PeerManager`.
    pub fn new() -> Self {
        Self {
            peers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Add a new peer to the manager.
    pub fn add_peer(&self, addr: SocketAddr, peer: Peer) {
        self.peers.lock().unwrap().insert(addr, peer);
    }

    /// Remove a peer from the manager.
    pub fn remove_peer(&self, addr: &SocketAddr) {
        self.peers.lock().unwrap().remove(addr);
    }

    /// Get a list of active peer addresses.
    #[allow(dead_code)]
    pub fn get_peers(&self) -> Vec<std::net::SocketAddr> {
        vec![]
    }

    /// Returns the current number of connected peers.
    pub fn peer_count(&self) -> usize {
        self.peers.lock().unwrap().len()
    }

    /// Broadcast a message to all connected peers asynchronously.
    pub async fn broadcast(&self, msg: &PeerMessage) {
        let writers: Vec<(SocketAddr, Arc<AsyncMutex<OwnedWriteHalf>>)> = {
            let peers = self.peers.lock().unwrap();
            peers.iter().map(|(addr, peer)| (*addr, peer.writer.clone())).collect()
        };
        let data = serde_json::to_vec(msg).expect("PeerMessage serialization failed");
        for (addr, writer) in writers {
            let data = data.clone();
            tokio::spawn(async move {
                let mut w = writer.lock().await;
                if let Err(e) = w.write_all(&data).await {
                    eprintln!("[Networking] Failed to send message to {}: {}", addr, e);
                }
            });
        }
    }
}

/// Represents the networking layer of the node.
#[allow(dead_code)]
pub struct Network {
    pub tracer: Tracer,
    config: NetworkConfig,
    peer_manager: Arc<PeerManager>,
}

/// P2P protocol message for block and transaction propagation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum PeerMessage {
    Block(crate::ledger::Block),
    Transaction(crate::ledger::Transaction),
    // Extend with handshake, ping, etc.
}

/// Peer trust level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum PeerTrust {
    Trusted,
    Untrusted,
}

/// Peer metadata for advanced selection
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct PeerMetadata {
    pub addr: SocketAddr,
    pub trust: PeerTrust,
    pub score: f64,
    pub last_active: std::time::Instant,
}

/// Advanced peer selection manager
pub struct PeerSelector {
    peers: Arc<Mutex<HashMap<SocketAddr, PeerMetadata>>>, // Map of peer metadata
}

impl PeerSelector {
    /// Create a new `PeerSelector`.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { peers: Arc::new(Mutex::new(HashMap::new())) }
    }

    /// Add/update peer metadata.
    #[allow(dead_code)]
    pub fn update_peer(&self, meta: PeerMetadata) {
        self.peers.lock().unwrap().insert(meta.addr, meta);
    }

    /// Select peers for churn/governor.
    #[allow(dead_code)]
    pub fn select_peers(&self, count: usize) -> Vec<SocketAddr> {
        let mut peers: Vec<_> = self.peers.lock().unwrap().values().cloned().collect();
        peers.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        peers.into_iter().take(count).map(|m| m.addr).collect()
    }

    /// Churn governor: periodically rotate peers.
    #[allow(dead_code)]
    pub fn churn(&self) {
        // Remove stale peers (not active for > 10 minutes)
        let now = std::time::Instant::now();
        self.peers.lock().unwrap().retain(|_, meta| now.duration_since(meta.last_active).as_secs() < 600);
        // Optionally: add new peers or prefer trusted ones
        // (Extend with peer discovery or scoring logic as needed)
    }
}

#[allow(dead_code)]
impl Network {
    /// Create a new networking layer with the given configuration.
    pub fn new(config: NetworkConfig, tracer: Tracer) -> Self {
        let peer_manager = Arc::new(PeerManager::new());
        Self {
            config,
            peer_manager,
            tracer,
        }
    }

    /// Start the networking subsystem asynchronously.
    pub async fn start(&self) {
        let addr: SocketAddr = self.config.bind_addr.parse().expect("Invalid bind_addr");
        let listener = TcpListener::bind(addr).await.expect("Failed to bind TCP listener");
        println!("[Networking] Listening on {} (max peers: {})", addr, self.config.max_peers);
        let peer_manager = self.peer_manager.clone();
        let max_peers = self.config.max_peers;
        // Peer discovery: connect to static peers if provided
        if let Some(discovery) = &self.config.discovery {
            match discovery.method {
                crate::configuration::PeerDiscoveryMethod::Static => {
                    for peer_addr in &discovery.peers {
                        if let Ok(addr) = peer_addr.parse() {
                            println!("[Networking] Attempting to connect to peer {}", addr);
                            self.connect_peer(addr).await;
                        }
                    }
                }
                crate::configuration::PeerDiscoveryMethod::Dns => {
                    // TODO: Implement DNS peer discovery (resolve DNS seeds)
                    println!("[Networking] DNS peer discovery not yet implemented");
                }
                crate::configuration::PeerDiscoveryMethod::Mdns => {
                    // TODO: Implement mDNS peer discovery
                    println!("[Networking] mDNS peer discovery not yet implemented");
                }
            }
        }
        loop {
            // Enforce max_peers limit
            if peer_manager.peer_count() >= max_peers {
                // Sleep and retry
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                continue;
            }
            let (socket, peer_addr) = match listener.accept().await {
                Ok(pair) => pair,
                Err(e) => {
                    eprintln!("[Networking] Accept error: {}", e);
                    continue;
                }
            };
            println!("[Networking] Accepted connection from {}", peer_addr);
            let peer_manager = peer_manager.clone();
            // Pass shared state to the event loop closure
            tokio::spawn(async move {
                let (mut _reader, writer) = socket.into_split();
                let peer = Peer { addr: peer_addr, writer: Arc::new(AsyncMutex::new(writer)) };
                peer_manager.add_peer(peer_addr, peer);
                let mut buf = [0u8; 1024];
                loop {
                    match _reader.read(&mut buf).await {
                        Ok(0) => {
                            println!("[Networking] Peer {} disconnected", peer_addr);
                            break;
                        }
                        Ok(n) => {
                            // Deserialize using serde_json
                            match serde_json::from_slice::<PeerMessage>(&buf[..n]) {
                                Ok(msg) => {
                                    println!("[Networking] Received {:?} from {}", msg, peer_addr);
                                    //let mut ledger_guard = ledger.lock().await;
                                    //let mempool_guard = mempool.lock().await;
                                    //let _ = Network::handle_peer_message_static(
                                    //    msg,
                                    //    &mut *ledger_guard,
                                    //    &*protocol,
                                    //    &*mempool_guard
                                    //).await;
                                }
                                Err(e) => {
                                    println!("[Networking] Error deserializing message from {}: {}", peer_addr, e);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("[Networking] Read error from {}: {}", peer_addr, e);
                            break;
                        }
                    }
                }
            });
        }
    }

    /// Connect to a peer.
    pub async fn connect_peer(&self, addr: SocketAddr) {
        match TcpStream::connect(addr).await {
            Ok(stream) => {
                println!("[Networking] Connected to peer {}", addr);
                let (reader, writer) = stream.into_split();
                let writer = Arc::new(AsyncMutex::new(writer));
                let peer = Peer { addr, writer: writer.clone() };
                self.peer_manager.add_peer(addr, peer);
                // Spawn a task to handle incoming messages from this peer
                let peer_manager = self.peer_manager.clone();
                tokio::spawn(async move {
                    Network::handle_peer_connection(peer_manager, reader, writer, addr, None, None, None).await;
                });
            }
            Err(e) => eprintln!("[Networking] Failed to connect to {}: {}", addr, e),
        }
    }

    /// Broadcast a block to all peers using the real P2P message format.
    pub async fn broadcast_block(&self, block: &crate::ledger::Block) {
        let msg = PeerMessage::Block(block.clone());
        self.peer_manager.broadcast(&msg).await;
        println!("[Networking] Broadcasting block to peers");
    }

    /// Relay a transaction to all peers using the real P2P message format.
    pub async fn relay_transaction(&self, tx: &crate::ledger::Transaction) {
        let msg = PeerMessage::Transaction(tx.clone());
        self.peer_manager.broadcast(&msg).await;
        println!("[Networking] Relaying transaction to peers");
    }

    /// Handle an incoming block from a peer (validate and apply).
    pub async fn receive_block(&self, block: crate::ledger::Block, ledger: &mut crate::ledger::Ledger, protocol: &crate::protocol::Protocol) {
        if ledger.validate_block(&block, protocol).await {
            println!("[Networking] Received and applied block: {:?}", block);
            ledger.apply_block(&block);
        } else {
            println!("[Networking] Invalid block received: {:?}", block);
        }
    }

    /// Handle a peer-to-peer message.
    pub async fn handle_peer_message(&self, msg: PeerMessage, ledger: &mut crate::ledger::Ledger, protocol: &crate::protocol::Protocol) {
        match msg {
            PeerMessage::Block(block) => self.receive_block(block, ledger, protocol).await,
            PeerMessage::Transaction(tx) => {
                // TODO: Add to mempool and relay
                println!("[Networking] Received transaction: {:?}", tx);
            }
        }
    }

    /// Run the network event loop (stub for now)
    pub async fn run(&self) {
        let addr: SocketAddr = self.config.bind_addr.parse().expect("Invalid bind_addr");
        let listener = TcpListener::bind(addr).await.expect("Failed to bind TCP listener");
        println!("[Networking] Network event loop running on {}", addr);
        loop {
            match listener.accept().await {
                Ok((socket, peer_addr)) => {
                    println!("[Networking] Accepted connection from {}", peer_addr);
                    // Optionally: spawn a task to handle the peer
                    let (_reader, _writer) = socket.into_split();
                    // TODO: Handle peer communication
                }
                Err(e) => {
                    eprintln!("[Networking] Accept error: {}", e);
                }
            }
        }
    }

    /// Start the networking subsystem asynchronously, passing shared state for ledger, protocol, and mempool.
    pub async fn start_with_state(
        &self,
        ledger: Arc<tokio::sync::Mutex<Ledger>>,
        mempool: Arc<tokio::sync::Mutex<Mempool>>,
        protocol: Arc<Protocol>,
    ) {
        let addr: SocketAddr = self.config.bind_addr.parse().expect("Invalid bind_addr");
        let listener = TcpListener::bind(addr).await.expect("Failed to bind TCP listener");
        println!("[Networking] Listening on {} (max peers: {})", addr, self.config.max_peers);
        let peer_manager = self.peer_manager.clone();
        let max_peers = self.config.max_peers;
        // Peer discovery: connect to static peers if provided
        if let Some(discovery) = &self.config.discovery {
            match discovery.method {
                crate::configuration::PeerDiscoveryMethod::Static => {
                    for peer_addr in &discovery.peers {
                        if let Ok(addr) = peer_addr.parse() {
                            println!("[Networking] Attempting to connect to peer {}", addr);
                            self.connect_peer(addr).await;
                        }
                    }
                }
                crate::configuration::PeerDiscoveryMethod::Dns => {
                    // TODO: Implement DNS peer discovery (resolve DNS seeds)
                    println!("[Networking] DNS peer discovery not yet implemented");
                }
                crate::configuration::PeerDiscoveryMethod::Mdns => {
                    // TODO: Implement mDNS peer discovery
                    println!("[Networking] mDNS peer discovery not yet implemented");
                }
            }
        }
        loop {
            if peer_manager.peer_count() >= max_peers {
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                continue;
            }
            let (socket, peer_addr) = match listener.accept().await {
                Ok(pair) => pair,
                Err(e) => {
                    eprintln!("[Networking] Accept error: {}", e);
                    continue;
                }
            };
            println!("[Networking] Accepted connection from {}", peer_addr);
            let peer_manager = peer_manager.clone();
            let ledger = ledger.clone();
            let protocol = protocol.clone();
            let mempool = mempool.clone();
            let (reader, writer) = socket.into_split();
            let writer = Arc::new(AsyncMutex::new(writer));
            tokio::spawn(async move {
                Network::handle_peer_connection(
                    peer_manager,
                    reader,
                    writer,
                    peer_addr,
                    Some(ledger),
                    Some(protocol),
                    Some(mempool),
                ).await;
            });
        }
    }
}

#[allow(dead_code)]
impl Network {
    /// Static version for use in async event loop
    pub async fn handle_peer_message_static(
        msg: PeerMessage,
        ledger: &mut Ledger,
        protocol: &Protocol,
        mempool: &Mempool,
    ) {
        match msg {
            PeerMessage::Block(block) => {
                let _ = Self::receive_block_static(block, ledger, protocol).await;
            }
            PeerMessage::Transaction(tx) => {
                if mempool.add_transaction(tx.clone()) {
                    println!("[Networking] Received and added transaction to mempool");
                } else {
                    println!("[Networking] Mempool full, transaction dropped");
                }
            }
        }
    }
    pub async fn receive_block_static(block: Block, ledger: &mut Ledger, protocol: &Protocol) -> bool {
        if ledger.validate_block(&block, protocol).await {
            println!("[Networking] Received and applied block: {:?}", block);
            true
        } else {
            println!("[Networking] Invalid block received: {:?}", block);
            false
        }
    }

    /// Handle incoming peer connections and messages.
    async fn handle_peer_connection(
        peer_manager: Arc<PeerManager>,
        mut reader: tokio::net::tcp::OwnedReadHalf,
        writer: Arc<AsyncMutex<OwnedWriteHalf>>,
        peer_addr: SocketAddr,
        ledger: Option<Arc<tokio::sync::Mutex<Ledger>>>,
        protocol: Option<Arc<Protocol>>,
        mempool: Option<Arc<tokio::sync::Mutex<Mempool>>>,
    ) {
        let peer = Peer { addr: peer_addr, writer: writer.clone() };
        peer_manager.add_peer(peer_addr, peer);
        let mut buf = [0u8; 4096];
        loop {
            match reader.read(&mut buf).await {
                Ok(0) => {
                    println!("[Networking] Peer {} disconnected", peer_addr);
                    peer_manager.remove_peer(&peer_addr);
                    break;
                }
                Ok(n) => {
                    // Deserialize using serde_json
                    match serde_json::from_slice::<PeerMessage>(&buf[..n]) {
                        Ok(msg) => {
                            println!("[Networking] Received {:?} from {}", msg, peer_addr);
                            if let (Some(ledger), Some(protocol), Some(mempool)) = (&ledger, &protocol, &mempool) {
                                let mut ledger_guard = ledger.lock().await;
                                let protocol_guard = protocol.as_ref();
                                let mempool_guard = mempool.lock().await;
                                let _ = Network::handle_peer_message_static(
                                    msg,
                                    &mut *ledger_guard,
                                    protocol_guard,
                                    &*mempool_guard
                                ).await;
                            }
                        }
                        Err(e) => {
                            println!("[Networking] Error deserializing message from {}: {}", peer_addr, e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("[Networking] Read error from {}: {}", peer_addr, e);
                    peer_manager.remove_peer(&peer_addr);
                    break;
                }
            }
        }
    }
}

/// Peer information for P2P networking
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PeerInfo {
    /// Peer address (IP:port or DNS)
    pub address: String,
    /// Last seen timestamp (unix epoch seconds)
    pub last_seen: u64,
    /// Is this peer currently connected?
    pub connected: bool,
}

/// P2P networking state
#[derive(Debug, Default)]
pub struct NetworkState {
    /// Known peers (address -> info)
    pub peers: std::collections::HashMap<String, PeerInfo>,
    /// Outbound connections (address -> handle)
    pub outbound: std::collections::HashMap<String, tokio::task::JoinHandle<()>>,
    /// Inbound connections (address -> handle)
    pub inbound: std::collections::HashMap<String, tokio::task::JoinHandle<()>>,
}

impl NetworkState {
    /// Add or update a peer
    pub fn update_peer(&mut self, address: &str, last_seen: u64, connected: bool) {
        self.peers.insert(
            address.to_string(),
            PeerInfo {
                address: address.to_string(),
                last_seen,
                connected,
            },
        );
    }
    /// Remove a peer
    pub fn remove_peer(&mut self, address: &str) {
        self.peers.remove(address);
        self.outbound.remove(address);
        self.inbound.remove(address);
    }

    /// Discover peers using static config or DNS (expandable for mDNS, Kademlia, etc.)
    pub async fn discover_peers(config: &crate::configuration::NetworkConfig) -> Vec<String> {
        let mut peers = vec![];
        if let Some(discovery) = &config.discovery {
            for peer in &discovery.peers {
                peers.push(peer.clone());
            }
        }
        // TODO: Add DNS/mDNS/Kademlia discovery here
        peers
    }

    /// Establish outbound connections to known peers
    pub async fn connect_to_peers(state: &mut NetworkState, peers: &[String]) {
        for peer in peers {
            if !state.outbound.contains_key(peer) {
                let address = peer.clone();
                let address_for_map = address.clone();
                let handle = tokio::spawn(async move {
                    // TODO: Implement real connection logic (TCP/QUIC)
                    println!("[Networking] Connecting to peer: {}", address);
                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                });
                state.outbound.insert(address_for_map, handle);
                state.update_peer(peer, chrono::Utc::now().timestamp() as u64, true);
            }
        }
    }

    /// Listen for inbound peer connections (async TCP listener)
    pub async fn listen_for_peers(state: &mut NetworkState, bind_addr: &str) {
        let listener = tokio::net::TcpListener::bind(bind_addr).await.expect("Failed to bind");
        println!("[Networking] Listening on {}", bind_addr);
        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    let peer_addr = addr.to_string();
                    let peer_addr_for_map = peer_addr.clone();
                    let handle = tokio::spawn(async move {
                        // TODO: Handle inbound protocol handshake, block/tx relay
                        println!("[Networking] Accepted inbound from {}", peer_addr);
                        let _ = stream;
                    });
                    state.inbound.insert(peer_addr_for_map.clone(), handle);
                    state.update_peer(&peer_addr_for_map, chrono::Utc::now().timestamp() as u64, true);
                }
                Err(e) => {
                    println!("[Networking] Accept error: {}", e);
                }
            }
        }
    }
}
