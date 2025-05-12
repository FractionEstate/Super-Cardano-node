//! LedgerDB configuration for Super Cardano Node
//!
//! Handles configuration for the ledger database and state snapshots.

use serde::{Serialize, Deserialize};

/// Configuration for the ledger database and state snapshots.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LedgerDBConfig {
    /// Path to the ledger database directory.
    pub db_path: String,
    /// Maximum number of state snapshots to retain.
    pub max_snapshots: usize,
    /// Enable or disable automatic pruning.
    pub pruning: bool,
}
