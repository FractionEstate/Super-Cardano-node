//! Tracing API for Super Cardano Node
//!
//! Provides tracing API types and traits.

use serde_json;

#[allow(dead_code)]
/// This module is intentionally left empty for future tracing API traits.

/// Tracing API trait for emitting trace events.
pub trait TracingAPI: Send + Sync {
    /// Emit a trace event with a message.
    fn trace(&self, message: &str);
    /// Emit a trace event with a structured payload.
    fn trace_structured(&self, event: &serde_json::Value);

    /// Register a callback for trace events.
    fn on_trace<F: Fn(&str) + Send + Sync + 'static>(&self, _callback: F) {
        // Store the callback for later use (stub: not yet implemented)
        // In a real implementation, keep a list of callbacks and invoke them on trace events
        // TODO: Implement callback registration for trace events
    }
}

#[allow(dead_code)]
/// Error type for tracing API.
#[derive(Clone, Debug)]
pub enum TracingAPIError {
    APIError(String),
}
