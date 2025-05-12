//! Partial/Parsed Node configuration (POM) for Super Cardano Node
//!
//! Handles partial and parsed node configuration management.

use serde::{Serialize, Deserialize};

/// Partial node configuration (POM) for layered config management.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartialNodeConfig {
    /// Optional network config.
    pub network: Option<String>,
    /// Optional protocol config.
    pub protocol: Option<String>,
    /// Optional consensus config.
    pub consensus: Option<String>,
    /// Optional logging config.
    pub logging: Option<String>,
}
