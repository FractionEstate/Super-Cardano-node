//! Integration tests for block propagation and fork choice in Super Cardano Node

#[tokio::test]
async fn test_block_propagation_and_fork_choice() {
    use crate::ledger::{Ledger, Block, Transaction, Chain, select_chain};
    use crate::protocol::Protocol;
    use crate::mempool::Mempool;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    let tracer = Super_Cardano_node::tracing::tracers::Tracer::default();
    let ledger1 = Arc::new(Mutex::new(Ledger::new(tracer.clone())));
    let ledger2 = Arc::new(Mutex::new(Ledger::new(tracer)));
    let protocol = Arc::new(Protocol::default());
    let mempool = Arc::new(Mutex::new(Mempool::new(100)));
    let tx = Transaction::default();
    mempool.lock().await.add_transaction(tx.clone());
    let block = {
        let mut l = ledger1.lock().await;
        Block::new_from_mempool(
            &mut *l,
            protocol.hard_fork.era_logic.as_ref(),
            &*mempool.lock().await,
            0, // slot
            0, // epoch
            "test-leader".to_string(), // leader
            vec![0u8; 32], // vrf_proof
            vec![0u8; 32], // kes_signature
        ).await.unwrap()
    };
    {
        let mut l2 = ledger2.lock().await;
        l2.apply_block(&block);
    }
    let tip1 = ledger1.lock().await.tip_hash();
    let tip2 = ledger2.lock().await.tip_hash();
    assert_eq!(tip1, tip2, "Block propagation failed: tips differ");
    let fork_block1 = Block::default();
    let fork_block2 = Block::default();
    let chains = vec![
        Chain { blocks: vec![block.clone(), fork_block1.clone()] },
        Chain { blocks: vec![block, fork_block2.clone()] }
    ];
    let selected = select_chain(&chains);
    assert!(selected.unwrap().blocks.len() >= 2);
}
