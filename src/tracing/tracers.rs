//! Tracers for Super Cardano Node
//!
//! Provides tracer types and implementations.

use std::sync::{Arc, Mutex};

/// Structured trace events for the node.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum TraceEvent {
    /// Node startup event.
    Startup(String),
    /// Node shutdown event.
    Shutdown(String),
    /// Peer connected event.
    PeerConnected(String),
    /// Peer disconnected event.
    PeerDisconnected(String),
    /// Peer trustable event.
    PeerTrustable(String),
    /// Block produced event.
    BlockProduced(u64),
    /// Block received event.
    BlockReceived(u64),
    /// Transaction received event.
    TransactionReceived(u64),
    /// Metrics event (name, value).
    Metrics(String, f64),
    /// State changed event.
    StateChanged(String),
    /// Leadership check event.
    LeadershipCheck(String),
    /// Resource usage event.
    ResourceUsage(String, f64),
    /// Consensus event.
    Consensus(String),
    /// ChainDB event.
    ChainDB(String),
    /// Configuration event.
    Config(String),
    /// Protocol event.
    Protocol(String),
    /// Handler event.
    Handler(String),
    /// Query event.
    Query(String),
    /// Error event.
    Error(String),
    /// Warning event.
    Warning(String),
    /// Info event.
    Info(String),
    /// Debug event.
    Debug(String),
    /// Custom event for extensibility.
    Custom(String),
    /// Extend TraceEvent to support all major node events, matching the Haskell tracers.
    /// Add variants for ChainDB, Consensus, NodeToNode, NodeToClient, Diffusion, Startup, Shutdown, NodeInfo, NodeVersion, NodeStartupInfo, NodeState, Resources, Peers, etc.
    NodeToNode(String),
    NodeToClient(String),
    Diffusion(String),
    NodeInfo(String),
    NodeVersion(String),
    NodeStartupInfo(String),
    NodeState(String),
    Resources(String),
    Peers(Vec<String>),
    /// KES info event.
    KESInfo(String),
    /// Forging stats event.
    ForgingStats(String),
    /// Consensus startup exception event.
    ConsensusStartupException(String),
    /// Block replay progress event.
    BlockReplayProgress(String),
    /// StateRep event.
    StateRep(String),
    /// NonP2P event.
    NonP2P(String),
    /// P2P event.
    P2P(String),
}

impl TraceEvent {
    /// Returns a human-readable description of the event.
    #[allow(dead_code)]
    pub fn description(&self) -> String {
        match self {
            TraceEvent::Startup(msg) => format!("Startup: {}", msg),
            TraceEvent::Shutdown(msg) => format!("Shutdown: {}", msg),
            TraceEvent::PeerConnected(peer) => format!("Peer connected: {}", peer),
            TraceEvent::PeerDisconnected(peer) => format!("Peer disconnected: {}", peer),
            TraceEvent::PeerTrustable(peer) => format!("Peer trustable: {}", peer),
            TraceEvent::BlockProduced(id) => format!("Block produced: {}", id),
            TraceEvent::BlockReceived(id) => format!("Block received: {}", id),
            TraceEvent::TransactionReceived(id) => format!("Transaction received: {}", id),
            TraceEvent::Metrics(name, value) => format!("Metric {} = {}", name, value),
            TraceEvent::StateChanged(state) => format!("State changed: {}", state),
            TraceEvent::LeadershipCheck(info) => format!("Leadership check: {}", info),
            TraceEvent::ResourceUsage(name, value) => format!("Resource usage {} = {}", name, value),
            TraceEvent::Consensus(info) => format!("Consensus: {}", info),
            TraceEvent::ChainDB(info) => format!("ChainDB: {}", info),
            TraceEvent::Config(info) => format!("Config: {}", info),
            TraceEvent::Protocol(info) => format!("Protocol: {}", info),
            TraceEvent::Handler(info) => format!("Handler: {}", info),
            TraceEvent::Query(info) => format!("Query: {}", info),
            TraceEvent::Error(msg) => format!("Error: {}", msg),
            TraceEvent::Warning(msg) => format!("Warning: {}", msg),
            TraceEvent::Info(msg) => format!("Info: {}", msg),
            TraceEvent::Debug(msg) => format!("Debug: {}", msg),
            TraceEvent::Custom(msg) => format!("Custom: {}", msg),
            TraceEvent::NodeToNode(info) => format!("NodeToNode: {}", info),
            TraceEvent::NodeToClient(info) => format!("NodeToClient: {}", info),
            TraceEvent::Diffusion(info) => format!("Diffusion: {}", info),
            TraceEvent::NodeInfo(info) => format!("NodeInfo: {}", info),
            TraceEvent::NodeVersion(info) => format!("NodeVersion: {}", info),
            TraceEvent::NodeStartupInfo(info) => format!("NodeStartupInfo: {}", info),
            TraceEvent::NodeState(info) => format!("NodeState: {}", info),
            TraceEvent::Resources(info) => format!("Resources: {}", info),
            TraceEvent::Peers(peers) => format!("Peers: {:?}", peers),
            TraceEvent::KESInfo(info) => format!("KESInfo: {}", info),
            TraceEvent::ForgingStats(info) => format!("ForgingStats: {}", info),
            TraceEvent::ConsensusStartupException(info) => format!("ConsensusStartupException: {}", info),
            TraceEvent::BlockReplayProgress(info) => format!("BlockReplayProgress: {}", info),
            TraceEvent::StateRep(info) => format!("StateRep: {}", info),
            TraceEvent::NonP2P(info) => format!("NonP2P: {}", info),
            TraceEvent::P2P(info) => format!("P2P: {}", info),
        }
    }
}

/// Tracer type for emitting trace events.
#[derive(Clone, Default)]
pub struct Tracer {
    #[allow(dead_code)]
    handlers: Arc<Mutex<Vec<Box<dyn Fn(&TraceEvent) + Send + Sync>>>>,
}

impl Tracer {
    /// Emit a trace event to all registered handlers.
    pub fn trace(&self, event: TraceEvent) {
        let handlers = self.handlers.lock().unwrap();
        for handler in handlers.iter() {
            handler(&event);
        }
    }

    /// Register a new handler for trace events.
    #[allow(dead_code)]
    pub fn register_tracer<T: Fn(&TraceEvent) + Send + Sync + 'static>(&self, handler: T) {
        self.handlers.lock().unwrap().push(Box::new(handler));
    }

    /// Clear all registered handlers.
    #[allow(dead_code)]
    pub fn clear_handlers(&self) {
        self.handlers.lock().unwrap().clear();
    }
}

/// Error type for tracing operations.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum TracingError {
    HandlerError(String),
    EmitError(String),
}
