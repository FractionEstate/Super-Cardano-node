//! Peer tracing for Super Cardano Node
//!
//! Provides tracing for peer events.

use crate::tracing::tracers::{TraceEvent, Tracer};

#[allow(dead_code)]
/// Peer tracing struct.
#[derive(Clone, Default)]
pub struct PeerTracing {
    tracer: Tracer,
}

#[allow(dead_code)]
impl PeerTracing {
    /// Trace a peer connected event.
    pub fn trace_peer_connected(&self, peer: &str) {
        self.tracer.trace(TraceEvent::PeerConnected(peer.to_string()));
    }
    /// Trace a peer disconnected event.
    pub fn trace_peer_disconnected(&self, peer: &str) {
        self.tracer.trace(TraceEvent::PeerDisconnected(peer.to_string()));
    }
    /// Trace a peer trustable event.
    pub fn trace_peer_trustable(&self, peer: &str) {
        self.tracer.trace(TraceEvent::PeerTrustable(peer.to_string()));
    }
    /// Trace a peer list event.
    pub fn trace_peers(&self, peers: &[String]) {
        self.tracer.trace(TraceEvent::Peers(peers.to_vec()));
    }

    /// Register a handler for peer events.
    pub fn on_peer_event<T: Fn(&TraceEvent) + Send + Sync + 'static>(&self, handler: T) {
        self.tracer.register_tracer(handler);
    }
}

#[allow(dead_code)]
/// Error type for peer tracing.
#[derive(Debug, Clone)]
pub enum PeerTracingError {
    /// Error occurred during peer event tracing.
    PeerEventError(String),
}
