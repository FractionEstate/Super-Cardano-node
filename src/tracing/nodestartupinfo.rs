//! Node startup info tracing for Super Cardano Node
//!
//! Provides tracing for node startup events.

use crate::tracing::tracers::{TraceEvent, Tracer};

/// Tracing for node startup events.
#[derive(Clone, Default)]
#[allow(dead_code)]
pub struct NodeStartupInfoTracing {
    tracer: Tracer,
}

impl NodeStartupInfoTracing {
    /// Trace a node startup event.
    #[allow(dead_code)]
    pub fn trace_startup(&self, info: &str) {
        self.tracer.trace(TraceEvent::NodeStartupInfo(info.to_string()));
    }

    /// Register a handler for startup events.
    #[allow(dead_code)]
    pub fn on_startup<T: Fn(&str) + Send + Sync + 'static>(&self, handler: T) {
        self.tracer.register_tracer(move |event| {
            if let TraceEvent::NodeStartupInfo(s) = event {
                handler(s);
            }
        });
    }
}

/// Error type for node startup info tracing.
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum NodeStartupInfoTracingError {
    /// Error occurred during startup event tracing.
    StartupEventError(String),
}
