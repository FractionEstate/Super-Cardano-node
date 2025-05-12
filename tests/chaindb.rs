//! Integration test for ChainDB persistence, rollback, and recovery

use Super_Cardano_node::ledger::{Ledger, Block, Transaction};
use Super_Cardano_node::chaindb::ChainDB;
use tokio::runtime::Runtime;
use std::fs;

#[test]
fn test_chaindb_persistence_and_rollback() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let _ = fs::remove_dir_all("./testdata/chaindb");
        let mut db = ChainDB::open("./testdata/chaindb").await.unwrap();
        let mut ledger = Ledger::new(Default::default());
        let tx = Transaction::default();
        let block = Block {
            id: 1,
            ..Default::default()
        };
        ledger.apply_block_with_db(&block, Some(&mut db)).await;
        // Check block and state are persisted
        let loaded_block = db.load_block(1).await.unwrap();
        assert_eq!(loaded_block.id, 1);
        let loaded_state = db.load_state(1).await.unwrap();
        // Rollback and check file removal
        db.rollback_to(0).await.unwrap();
        assert!(db.load_block(1).await.is_err());
        assert!(db.load_state(1).await.is_err());
    });
}
