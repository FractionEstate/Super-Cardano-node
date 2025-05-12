//! Allegra era protocol logic for Super Cardano Node

use crate::protocol::EraLogic;
use crate::ledger::{Transaction, Block};

/// Allegra era logic (timelocks, multi-sig)
pub struct AllegraEra;

impl EraLogic for AllegraEra {
    fn name(&self) -> &'static str { "Allegra" }
    fn validate_transaction(&self, tx: &Transaction) -> bool {
        // TODO: Implement Allegra-specific validation (timelocks, multi-sig)
        !tx.inputs.is_empty() && !tx.outputs.is_empty()
    }
    fn validate_block(&self, _block: &Block) -> bool { true }
}
