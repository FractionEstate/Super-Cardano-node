//! Peer tracing for Super Cardano Node
//!
//! Provides tracing for peer-related events, including peer trust, peer lists, and event handlers.
//!
//! # Example
//! ```rust,ignore
//! let peer_tracing = PeerTracing::default();
//! let peers = vec!["127.0.0.1:3001".to_string(), "127.0.0.1:3002".to_string()];
//! peer_tracing.trace_peers(&peers);
//! peer_tracing.on_peer_event(|event| println!("Peer event: {:?}", event));
//! ```

use crate::tracing::tracers::{TraceEvent, Tracer};

/// Peer tracing struct.
///
/// Provides methods to trace peer-related events such as connections, disconnections, and trust status.
///
/// # Example
/// ```rust,ignore
/// let peer_tracing = PeerTracing::default();
/// peer_tracing.trace_peer_connected("127.0.0.1:3001");
/// ```
#[derive(Clone)]
pub struct PeerTracing {
    pub tracer: Tracer,
}

impl Default for PeerTracing {
    fn default() -> Self {
        Self { tracer: Tracer::default() }
    }
}

impl PeerTracing {
    /// Trace a peer connected event.
    ///
    /// # Arguments
    /// * `peer` - The address of the connected peer.
    pub fn trace_peer_connected(&self, peer: &str) {
        self.tracer.trace(TraceEvent::PeerConnected(peer.to_string()));
    }
    /// Trace a peer disconnected event.
    ///
    /// # Arguments
    /// * `peer` - The address of the disconnected peer.
    pub fn trace_peer_disconnected(&self, peer: &str) {
        self.tracer.trace(TraceEvent::PeerDisconnected(peer.to_string()));
    }
    /// Trace a peer trustable event.
    ///
    /// # Arguments
    /// * `peer` - The address of the peer considered trustable.
    ///
    /// # Example
    /// ```rust,ignore
    /// let peer_tracing = PeerTracing::default();
    /// peer_tracing.trace_peer_trustable("127.0.0.1:3001");
    /// ```
    pub fn trace_peer_trustable(&self, peer: &str) {
        self.tracer.trace(TraceEvent::PeerTrustable(peer.to_string()));
    }
    /// Trace a peer list event.
    ///
    /// # Arguments
    /// * `peers` - A slice of peer addresses.
    ///
    /// # Example
    /// ```rust,ignore
    /// let peer_tracing = PeerTracing::default();
    /// let peers = vec!["127.0.0.1:3001".to_string(), "127.0.0.1:3002".to_string()];
    /// peer_tracing.trace_peers(&peers);
    /// ```
    pub fn trace_peers(&self, peers: &[String]) {
        self.tracer.trace(TraceEvent::Peers(peers.to_vec()));
    }
    /// Register a handler for peer events.
    ///
    /// # Arguments
    /// * `handler` - A function to handle peer trace events.
    ///
    /// # Example
    /// ```rust,ignore
    /// let peer_tracing = PeerTracing::default();
    /// peer_tracing.on_peer_event(|event| println!("Peer event: {:?}", event));
    /// ```
    pub fn on_peer_event<T: Fn(&TraceEvent) + Send + Sync + 'static>(&self, handler: T) {
        self.tracer.register_tracer(handler);
    }
}

/// Error type for peer tracing.
///
/// Represents errors that can occur during peer event tracing.
#[derive(Debug, Clone)]
pub enum PeerTracingError {
    /// Error occurred during peer event tracing.
    PeerEventError(String),
}
