// Minimal protocol module stubs
pub trait EraLogic: std::fmt::Debug + Send + Sync {
    fn name(&self) -> &'static str;
    fn validate_transaction(&self, tx: &crate::wallet::transaction::Transaction) -> bool;
    fn validate_block(&self, block: &crate::ledger::Block) -> bool;
}
pub struct Era;
pub trait ProtocolHandler {}
pub mod allegra;
pub mod alonzo;
pub mod babbage;
pub mod byron;
pub mod conway;
pub mod hard_fork_combinator;
pub mod mary;
pub mod shelley;
pub mod types;
