//! Byron protocol logic for Super Cardano Node
//!
//! Implements Byron-era consensus and ledger rules.

use crate::ledger::{Block, Transaction};
use crate::protocol::types::ProtocolEra;
use serde::{Deserialize, Serialize};
use serde_json;

/// Byron protocol configuration parameters.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ByronConfig {
    /// Maximum block size in bytes.
    pub max_block_size: usize,
    /// Maximum transaction size in bytes.
    pub max_tx_size: usize,
    /// Byron protocol version.
    pub version: u32,
}

#[allow(dead_code)]
/// Byron protocol state (can be extended for consensus state, etc.)
#[derive(Debug, Clone, Default)]
pub struct ByronState {
    /// Current slot number.
    pub slot: u64,
    /// Current epoch number.
    pub epoch: u64,
}

#[allow(dead_code)]
impl ByronState {
    /// Advance to the next slot.
    pub fn next_slot(&mut self) {
        self.slot += 1;
    }
    /// Advance to the next epoch.
    pub fn next_epoch(&mut self) {
        self.epoch += 1;
        self.slot = 0;
    }
}

/// Byron protocol era logic implementation for Cardano consensus and ledger.
#[derive(Debug, Default, Clone)]
pub struct ByronProtocol {
    // Add Byron-specific state fields here
}

#[allow(dead_code)]
impl ByronProtocol {
    /// Create a new Byron protocol instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Validate a Byron-era transaction.
    ///
    /// # Example
    ///
    /// Example usage for Byron transaction validation
    /// ```rust
    /// use Super_Cardano_node::ledger::Transaction;
    /// use Super_Cardano_node::protocol::byron::{ByronConfig, ByronProtocol};
    /// let tx = Transaction::default();
    /// let cfg = ByronConfig::default();
    /// let valid = ByronProtocol::validate_transaction(&tx, &cfg);
    /// ```
    pub fn validate_transaction(tx: &Transaction, cfg: &ByronConfig) -> bool {
        // Byron: basic checks (non-empty inputs/outputs, size limit)
        !tx.inputs.is_empty() && !tx.outputs.is_empty()
            && serde_json::to_vec(tx)
                .map(|bytes| bytes.len() <= cfg.max_tx_size)
                .unwrap_or(false)
    }

    /// Validate a Byron-era block.
    ///
    /// # Example
    ///
    /// Example usage for Byron block validation
    /// ```rust
    /// use Super_Cardano_node::ledger::Block;
    /// use Super_Cardano_node::protocol::byron::{ByronConfig, ByronProtocol};
    /// let block = Block::default();
    /// let cfg = ByronConfig::default();
    /// let valid = ByronProtocol::validate_block(&block, &cfg);
    /// ```
    pub fn validate_block(block: &Block, cfg: &ByronConfig) -> bool {
        // Byron: check block size and all txs valid
        serde_json::to_vec(block)
            .map(|bytes| bytes.len() <= cfg.max_block_size)
            .unwrap_or(false)
            && block.transactions.iter().all(|tx| Self::validate_transaction(tx, cfg))
    }
}

impl ProtocolEra for ByronProtocol {
    fn name(&self) -> &'static str {
        "Byron"
    }
    fn validate_block(&self, _block: &crate::ledger::Block) -> bool {
        // TODO: Byron-specific block validation logic
        true
    }
}

#[allow(dead_code)]
/// Error types for Byron protocol instantiation.
#[derive(Clone, Debug)]
pub enum ByronProtocolInstantiationError {
    CanonicalDecodeFailure(String),
    GenesisHashMismatch(u64, u64),
    DelegationCertificateFilepathNotSpecified,
    GenesisConfigurationError(String),
    GenesisReadError(String),
    CredentialsError(String),
    SigningKeyDeserialiseFailure(String),
    SigningKeyFilepathNotSpecified,
}
