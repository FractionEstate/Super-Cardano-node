//! Logging configuration for Super Cardano Node
//!
//! Handles configuration for logging and tracing backends.

use serde::{Serialize, Deserialize};

/// Logging and tracing configuration for the node.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoggingConfig {
    /// Log file path.
    pub log_file: String,
    /// Log level (e.g., info, debug, warn, error).
    pub log_level: String,
    /// Enable structured (JSON) logging.
    pub structured: bool,
}
