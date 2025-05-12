//! Tracing and logging for Super Cardano Node
//!
//! Handles metrics, structured logs, startup/shutdown, and diagnostics.
//! Uses idiomatic Rust logging and metrics best practices.
//!
//! NOTE: Ensure you have the following dependencies in Cargo.toml:
//! tracing = "0.1"
//! tracing-subscriber = "0.3"

pub mod tracers;
pub mod staterep;
pub mod peers;
pub mod nodestartupinfo;
pub mod nodeinfo;
pub mod render;
pub mod formatting;
pub mod documentation;
pub mod default_trace_config;
pub mod consistency;
pub mod compat;
pub mod api;

use tracing_subscriber::EnvFilter;
use tracers::{Tracer, TraceEvent};

/// Tracing/logging configuration and state
pub struct Tracing {
    pub tracer: Tracer,
}

impl Tracing {
    /// Initialize tracing/logging
    pub fn new() -> Self {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();
        Self { tracer: Tracer::default() }
    }

    #[allow(dead_code)]
    /// Log node startup
    pub fn startup(&self) {
        self.tracer.trace(TraceEvent::Startup("Node startup complete".to_string()));
        tracing::info!("[Tracing] Node startup complete.");
    }

    #[allow(dead_code)]
    /// Log node shutdown
    pub fn shutdown(&self) {
        self.tracer.trace(TraceEvent::Shutdown("Node shutdown complete".to_string()));
        tracing::info!("[Tracing] Node shutdown complete.");
    }

    #[allow(dead_code)]
    /// Log a metric
    pub fn metric(&self, name: &str, value: f64) {
        self.tracer.trace(TraceEvent::Metrics(name.to_string(), value));
        tracing::info!("[Metric] {} = {}", name, value);
    }
}
