//! REST API endpoints for Super Cardano Node
//!
//! Implements OpenAPI-compatible endpoints for node, chain, and wallet.
use serde::{Deserialize, Serialize};

/// Health check endpoint: GET /health
/// Returns node health status.
///
/// # Example
/// curl http://localhost:8080/health
pub async fn get_health() -> &'static str {
    "ok"
}

/// Get the current chain tip: GET /tip
/// Returns the latest block hash and slot.
#[derive(Serialize, Deserialize)]
pub struct TipResponse {
    pub block_hash: String,
    pub slot: u64,
}

pub async fn get_tip() -> TipResponse {
    // TODO: Query real chain state
    TipResponse {
        block_hash: "dummy_block_hash".to_string(),
        slot: 0,
    }
}

/// Get a block by hash: GET /block/{hash}
#[derive(Serialize, Deserialize)]
pub struct BlockResponse {
    pub block_hash: String,
    pub slot: u64,
    pub transactions: Vec<String>,
}

pub async fn get_block(_hash: String) -> BlockResponse {
    // TODO: Query real block data
    BlockResponse {
        block_hash: _hash,
        slot: 0,
        transactions: vec![],
    }
}

/// Submit a transaction: POST /tx
#[derive(Serialize, Deserialize)]
pub struct SubmitTxRequest {
    pub cbor: String,
}

#[derive(Serialize, Deserialize)]
pub struct SubmitTxResponse {
    pub tx_hash: String,
    pub accepted: bool,
}

pub async fn submit_tx(_req: SubmitTxRequest) -> SubmitTxResponse {
    // TODO: Validate and submit transaction
    SubmitTxResponse {
        tx_hash: "dummy_tx_hash".to_string(),
        accepted: true,
    }
}

/// Get connected peers: GET /peers
#[derive(Serialize, Deserialize)]
pub struct PeerInfo {
    pub address: String,
    pub connected: bool,
}

pub async fn get_peers() -> Vec<PeerInfo> {
    // TODO: Query real peer manager
    vec![PeerInfo {
        address: "127.0.0.1:3001".to_string(),
        connected: true,
    }]
}

/// Add a peer: POST /peers/add
#[derive(Serialize, Deserialize)]
pub struct AddPeerRequest {
    pub address: String,
}

#[derive(Serialize, Deserialize)]
pub struct AddPeerResponse {
    pub success: bool,
}

pub async fn add_peer(_req: AddPeerRequest) -> AddPeerResponse {
    // TODO: Add peer to peer manager
    AddPeerResponse { success: true }
}

/// Remove a peer: POST /peers/remove
#[derive(Serialize, Deserialize)]
pub struct RemovePeerRequest {
    pub address: String,
}

#[derive(Serialize, Deserialize)]
pub struct RemovePeerResponse {
    pub success: bool,
}

pub async fn remove_peer(_req: RemovePeerRequest) -> RemovePeerResponse {
    // TODO: Remove peer from peer manager
    RemovePeerResponse { success: true }
}

/// Node lifecycle: POST /node/reload, POST /node/shutdown
#[derive(Serialize, Deserialize)]
pub struct NodeControlResponse {
    pub success: bool,
}

pub async fn reload_node() -> NodeControlResponse {
    // TODO: Reload node config/state
    NodeControlResponse { success: true }
}

pub async fn shutdown_node() -> NodeControlResponse {
    // TODO: Trigger graceful shutdown
    NodeControlResponse { success: true }
}

/// Consensus/protocol control: POST /consensus/pause, /consensus/resume
#[derive(Serialize, Deserialize)]
pub struct ConsensusControlResponse {
    pub success: bool,
}

pub async fn pause_consensus() -> ConsensusControlResponse {
    // TODO: Pause consensus
    ConsensusControlResponse { success: true }
}

pub async fn resume_consensus() -> ConsensusControlResponse {
    // TODO: Resume consensus
    ConsensusControlResponse { success: true }
}

/// Mempool query: GET /mempool
#[derive(Serialize, Deserialize)]
pub struct MempoolTx {
    pub tx_hash: String,
    pub size: usize,
}

pub async fn get_mempool() -> Vec<MempoolTx> {
    // TODO: Query real mempool
    vec![]
}

/// Tracing/metrics: GET /metrics
#[derive(Serialize, Deserialize)]
pub struct MetricsResponse {
    pub uptime: u64,
    pub block_count: u64,
    pub peer_count: u64,
}

pub async fn get_metrics() -> MetricsResponse {
    // TODO: Gather real metrics
    MetricsResponse {
        uptime: 0,
        block_count: 0,
        peer_count: 0,
    }
}
