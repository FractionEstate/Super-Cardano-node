//! State representation tracing for Super Cardano Node
//!
//! Provides tracing for state representation events.

use crate::tracing::tracers::{TraceEvent, Tracer};

/// Tracing for state representation events.
#[derive(Clone, Default)]
#[allow(dead_code)]
pub struct StateRepTracing {
    tracer: Tracer,
}

impl StateRepTracing {
    /// Trace a state representation event.
    #[allow(dead_code)]
    pub fn trace_state(&self, state: &str) {
        self.tracer.trace(TraceEvent::StateRep(state.to_string()));
    }

    /// Register a handler for state representation events.
    #[allow(dead_code)]
    pub fn on_state_change<T: Fn(&str) + Send + Sync + 'static>(&self, handler: T) {
        self.tracer.register_tracer(move |event| {
            if let TraceEvent::StateRep(s) = event {
                handler(s);
            }
        });
    }
}

/// Error type for state representation tracing.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum StateRepTracingError {
    /// Error occurred during state change tracing.
    StateChangeError(String),
}
