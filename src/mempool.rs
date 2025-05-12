//! Mempool module for Super Cardano Node
//!
//! Handles transaction pool, relay, and validation logic.

use crate::ledger::Transaction;
use std::collections::{VecDeque};
use std::sync::{Arc, Mutex};

/// Represents the node's mempool (transaction pool).
#[derive(Clone)]
pub struct Mempool {
    pool: Arc<Mutex<VecDeque<Transaction>>>,
    max_size: usize,
}

impl Mempool {
    /// Create a new mempool with a maximum size.
    pub fn new(max_size: usize) -> Self {
        Self {
            pool: Arc::new(Mutex::new(VecDeque::new())),
            max_size,
        }
    }

    /// Add a transaction to the mempool (returns false if full).
    pub fn add_transaction(&self, tx: Transaction) -> bool {
        let mut pool = self.pool.lock().unwrap();
        if pool.len() >= self.max_size {
            return false;
        }
        pool.push_back(tx);
        true
    }

    /// Get all transactions in the mempool.
    pub fn get_transactions(&self) -> Vec<Transaction> {
        let pool = self.pool.lock().unwrap();
        pool.iter().cloned().collect()
    }

    #[allow(dead_code)]
    /// Remove a transaction from the mempool by id.
    pub fn remove_transaction(&self, tx_id: u64) {
        let mut pool = self.pool.lock().unwrap();
        if let Some(pos) = pool.iter().position(|tx| tx.id == tx_id) {
            pool.remove(pos);
        }
    }

    /// Clear the mempool (e.g., after block production).
    pub fn clear(&self) {
        let mut pool = self.pool.lock().unwrap();
        pool.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ledger::{TxOutput};

    #[test]
    fn mempool_add_and_remove() {
        let mempool = Mempool::new(2);
        let tx1 = Transaction {
            id: 1,
            inputs: vec![],
            outputs: vec![TxOutput { address: "A".to_string(), amount: 10, assets: None }],
            certificates: vec![],
            plutus_witnesses: vec![],
        };
        let tx2 = Transaction {
            id: 2,
            inputs: vec![],
            outputs: vec![TxOutput { address: "B".to_string(), amount: 20, assets: None }],
            certificates: vec![],
            plutus_witnesses: vec![],
        };
        assert!(mempool.add_transaction(tx1.clone()));
        assert!(mempool.add_transaction(tx2.clone()));
        assert!(!mempool.add_transaction(tx2.clone())); // full
        let txs = mempool.get_transactions();
        assert_eq!(txs.len(), 2);
        mempool.remove_transaction(1);
        let txs = mempool.get_transactions();
        assert_eq!(txs.len(), 1);
        assert_eq!(txs[0].id, 2);
        mempool.clear();
        assert_eq!(mempool.get_transactions().len(), 0);
    }
}
