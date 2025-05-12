//! Common types for Cardano protocol eras.
//!
//! Shared types and traits for protocol eras.

/// ProtocolEra trait for all Cardano protocol eras.
#[allow(dead_code)]
pub trait ProtocolEra: Send + Sync {
    /// Returns the name of the era.
    fn name(&self) -> &'static str;
    /// Validate a block for this era.
    fn validate_block(&self, block: &crate::ledger::Block) -> bool;
    // Extend with shared protocol methods as needed
}

/// Example protocol type.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ProtocolType;
