//! Property-based tests for consensus and networking in Super Cardano Node

use proptest::prelude::*;
use crate::ledger::{Transaction, TxInput, TxOutput};
use crate::mempool::Mempool;

// Local wrapper for Transaction to allow Arbitrary impl
#[derive(Debug, Clone)]
struct PropTransaction(pub Transaction);

impl Arbitrary for PropTransaction {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        (
            any::<u64>(),
            proptest::collection::vec((any::<u64>(), any::<u32>()), 0..4),
            proptest::collection::vec((any::<String>(), any::<u64>()), 0..4)
        ).prop_map(|(id, inputs, outputs)| PropTransaction(Transaction {
            id,
            inputs: inputs.into_iter().map(|(prev_tx, index)| TxInput { prev_tx, index }).collect(),
            outputs: outputs.into_iter().map(|(address, amount)| TxOutput { address, amount, assets: None }).collect(),
            certificates: vec![],
            plutus_witnesses: vec![],
        })).boxed()
    }
}

impl PartialEq for PropTransaction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for PropTransaction {}

proptest! {
    #[test]
    fn prop_tx_roundtrip(tx in any::<PropTransaction>()) {
        let mempool = Mempool::new(100);
        assert!(mempool.add_transaction(tx.0.clone()));
        let txs = mempool.get_transactions();
        assert!(txs.contains(&tx.0));
    }
}

proptest! {
    #[test]
    fn prop_protocol_upgrade_transition(
        start_epoch in 0u64..5,
        fork_epoch in 1u64..10,
        tx in any::<PropTransaction>()
    ) {
        use Super_Cardano_node::protocol::{Protocol, Era};
        use Super_Cardano_node::configuration::ProtocolConfig;
        use futures::executor::block_on;
        let mut protocol = Protocol::new(ProtocolConfig { era: "Byron".to_string() });
        protocol.schedule_upgrade(Era::Shelley, fork_epoch);
        // Simulate epochs and check transition
        for epoch in start_epoch..(fork_epoch+2) {
            block_on(protocol.handle_upgrade(epoch));
            if epoch < fork_epoch {
                assert_eq!(protocol.hard_fork.current_era, Era::Byron);
            } else {
                assert_eq!(protocol.hard_fork.current_era, Era::Shelley);
            }
        }
        // Transaction validation should follow current era
        let valid = block_on(protocol.validate_transaction(&tx.0));
        // Byron: only checks non-empty inputs/outputs, Shelley: no zero outputs
        if protocol.hard_fork.current_era == Era::Shelley {
            assert_eq!(valid, tx.0.inputs.len() > 0 && tx.0.outputs.iter().all(|o| o.amount > 0));
        }
    }
}
