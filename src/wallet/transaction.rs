// Minimal stubs for wallet::transaction
pub struct Transaction;
#[derive(Debug)]
pub struct TransactionBuilder;

impl TransactionBuilder {
    pub fn new() -> Self {
        TransactionBuilder
    }
    pub fn get_inputs(&self) -> Vec<String> {
        vec![]
    }
    pub fn sign_input(&mut self, _idx: usize, _key: &str) -> bool {
        true
    }
    pub fn build(&self) -> Option<Transaction> {
        Some(Transaction)
    }
    pub fn add_output(&mut self, _address: crate::wallet::address::Address, _amount: u64) {}
    pub fn add_input(&mut self, _utxo: String) {}
    pub fn add_change_output(&mut self, _address: crate::wallet::address::Address, _amount: u64) {}
    pub fn total_input(&self) -> u64 {
        0
    }
    pub fn total_output(&self) -> u64 {
        0
    }
    pub fn set_fee(&mut self, _fee: u64) {}
    pub fn estimate_size(&self) -> usize {
        0
    }
}
