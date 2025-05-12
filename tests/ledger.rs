//! Integration and unit tests for the Ledger module (UTXO, block/tx validation).

use crate::ledger::Ledger;
use crate::tracing::tracers::Tracer;

#[test]
fn test_ledger_new_and_tip_hash() {
    let tracer = Tracer::default();
    let ledger = Ledger::new(tracer);
    assert_eq!(ledger.tip_hash(), 0);
}

#[test]
fn test_block_creation_and_validation() {
    let tracer = Tracer::default();
    let mut ledger = Ledger::new(tracer);
    // let tx = Transaction { id: 1, inputs: vec![], outputs: vec![TxOutput { address: "A".to_string(), amount: 10 }] };
    // Add transaction to mempool and create a block (mocked)
    // ...
    // assert!(ledger.validate_block(&block, &protocol));
}
