//! Tracing documentation for Super Cardano Node
//!
//! Provides documentation for tracing events and configuration.

/// Tracing documentation struct.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct TracingDocumentation {
    /// Markdown or text documentation for tracing events.
    pub doc: String,
}

impl TracingDocumentation {
    /// Get documentation for a tracing event or subsystem.
    #[allow(dead_code)]
    pub fn get_doc(&self) -> &str {
        &self.doc
    }
    /// Set documentation for a tracing event or subsystem.
    #[allow(dead_code)]
    pub fn set_doc(&mut self, doc: &str) {
        self.doc = doc.to_string();
    }
}

/// Error type for tracing documentation.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum TracingDocumentationError {
    DocumentationError(String),
}
