//! Mary era protocol logic for Super Cardano Node

use crate::protocol::EraLogic;
use crate::ledger::{Transaction, Block};

/// Mary era logic (multi-asset)
pub struct MaryEra;

impl EraLogic for MaryEra {
    fn name(&self) -> &'static str { "Mary" }
    fn validate_transaction(&self, tx: &Transaction) -> bool {
        // TODO: Implement Mary-specific validation (multi-asset)
        !tx.inputs.is_empty() && !tx.outputs.is_empty()
    }
    fn validate_block(&self, _block: &Block) -> bool { true }
}
