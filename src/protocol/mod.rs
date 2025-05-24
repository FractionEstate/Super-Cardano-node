// Re-export wallet for protocol modules
pub use crate::wallet;
// Minimal protocol module stubs
pub trait EraLogic: std::fmt::Debug + Send + Sync {
    fn name(&self) -> &'static str;
    fn validate_transaction(&self, tx: &wallet::Transaction) -> bool;
    fn validate_block(&self, block: &crate::ledger::Block) -> bool;
}
#[derive(Debug, Clone)]
pub struct Era;
pub trait ProtocolHandler {
    fn on_block_received(&self, block: &crate::ledger::Block) {}
    fn on_transaction_received(&self, tx: &crate::ledger::Transaction) {}
}

// Minimal stub for Protocol type to satisfy consensus/ledger references
/// Main protocol state for Cardano node, including hard fork combinator and era logic.
use crate::protocol::hard_fork_combinator::HardForkCombinator;
use std::sync::Arc;

pub struct Protocol {
    /// Hard fork combinator managing era transitions and logic
    pub hard_fork: HardForkCombinator,
}

impl Protocol {
    /// Handle protocol upgrade (era transition) for the given epoch.
    pub async fn handle_upgrade(&mut self, current_epoch: u64) {
        self.hard_fork.check_transition(current_epoch);
    }

    /// Validate a block using the current era logic.
    pub async fn validate_block(&self, block: &crate::ledger::Block) -> bool {
        self.hard_fork.validate_block(block)
    }

    /// Validate a transaction using the current era logic.
    pub async fn validate_transaction(&self, tx: &wallet::Transaction) -> bool {
        self.hard_fork.current_logic().validate_transaction(tx)
    }
}
pub mod allegra;
pub mod alonzo;
pub mod babbage;
pub mod byron;
pub mod conway;
pub mod hard_fork_combinator;
pub mod mary;
pub mod shelley;
pub mod types;
