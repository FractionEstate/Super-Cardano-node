//! Tracing documentation for Super Cardano Node
//!
//! Provides documentation for tracing events and configuration.

/// Tracing documentation struct.
///
/// Holds documentation (in Markdown or plain text) for tracing events and configuration.
///
/// # Example
/// ```
/// let mut doc = TracingDocumentation::default();
/// doc.set_doc("This documents a tracing event.");
/// assert_eq!(doc.get_doc(), "This documents a tracing event.");
/// ```
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct TracingDocumentation {
    /// Markdown or text documentation for tracing events.
    pub doc: String,
}

impl TracingDocumentation {
    /// Get documentation for a tracing event or subsystem.
    ///
    /// Returns the documentation string.
    pub fn get_doc(&self) -> &str {
        &self.doc
    }
    /// Set documentation for a tracing event or subsystem.
    ///
    /// # Arguments
    /// * `doc` - The documentation string to set.
    pub fn set_doc(&mut self, doc: &str) {
        self.doc = doc.to_string();
    }
}

/// Error type for tracing documentation.
///
/// Represents errors that can occur when handling tracing documentation.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum TracingDocumentationError {
    /// Error occurred while handling tracing documentation.
    DocumentationError(String),
}
