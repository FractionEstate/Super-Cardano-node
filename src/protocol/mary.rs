//! Mary era protocol logic for Super Cardano Node

use crate::ledger::Block;
use crate::protocol::EraLogic;
use crate::protocol::wallet;

/// Mary era logic (multi-asset)
#[derive(Debug)]
pub struct MaryEra;

impl EraLogic for MaryEra {
    fn name(&self) -> &'static str {
        "Mary"
    }
    fn validate_transaction(&self, tx: &wallet::Transaction) -> bool {
        // TODO: Implement Mary-specific validation (multi-asset)
        true
    }
    fn validate_block(&self, _block: &crate::ledger::Block) -> bool {
        true
    }
}
