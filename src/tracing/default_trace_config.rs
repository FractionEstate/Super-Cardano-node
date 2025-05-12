//! Default trace configuration for Super Cardano Node
//!
//! Provides default configuration for tracing.

/// Default trace configuration for the node.
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct DefaultTraceConfig {
    /// Enable or disable default tracing.
    pub enabled: bool,
    /// Default log level.
    pub log_level: String,
}

impl DefaultTraceConfig {
    /// Returns the default configuration.
    #[allow(dead_code)]
    pub fn default_config() -> Self {
        Self {
            enabled: true,
            log_level: "info".to_string(),
        }
    }
    /// Set log level.
    #[allow(dead_code)]
    pub fn set_log_level(&mut self, level: &str) {
        self.log_level = level.to_string();
    }
    /// Enable or disable tracing.
    #[allow(dead_code)]
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

/// Error type for default trace config.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum DefaultTraceConfigError {
    ConfigError(String),
}
