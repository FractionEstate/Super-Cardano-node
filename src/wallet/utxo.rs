// Minimal stub for wallet::utxo
#[derive(Debug)]
pub struct UtxoSet;

impl UtxoSet {
    pub fn select_utxos(&self, _amount: u64) -> Option<Vec<String>> {
        Some(vec![])
    }
    pub fn get_address(&self, _input: &str) -> Option<String> {
        Some("addr_test1...".to_string())
    }
}

impl UtxoSet {
    pub fn new() -> Self {
        UtxoSet
    }
}
