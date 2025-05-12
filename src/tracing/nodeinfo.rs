//! Node info tracing for Super Cardano Node
//!
//! Provides tracing for node information events.

use crate::tracing::tracers::{TraceEvent, Tracer};

#[allow(dead_code)]
/// Node info tracing struct.
#[derive(Clone, Default)]
pub struct NodeInfoTracing {
    tracer: Tracer,
}

impl NodeInfoTracing {
    /// Trace a node info event.
    #[allow(dead_code)]
    pub fn trace_info(&self, info: &str) {
        self.tracer.trace(TraceEvent::NodeInfo(info.to_string()));
    }

    /// Register a handler for node info events.
    #[allow(dead_code)]
    pub fn on_info<T: Fn(&str) + Send + Sync + 'static>(&self, handler: T) {
        self.tracer.register_tracer(move |event| {
            if let TraceEvent::NodeInfo(s) = event {
                handler(s);
            }
        });
    }
}

#[allow(dead_code)]
/// Error type for node info tracing.
#[derive(Debug, Clone)]
pub enum NodeInfoTracingError {
    /// Error occurred during node info event tracing.
    InfoEventError(String),
}
