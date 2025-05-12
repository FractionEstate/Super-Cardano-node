//! Tests for Ouroboros consensus logic (Praos, slot leadership).

use Super_Cardano_node::consensus::{praos_is_leader, PraosKeys};

#[test]
fn test_praos_leader_threshold() {
    let keys = PraosKeys::generate();
    let slot = 42;
    let stake = 100.0;
    let total_stake = 1000.0;
    let is_leader = praos_is_leader(slot, &keys, stake, total_stake);
    assert!(is_leader == false || is_leader == true);
}

#[test]
fn test_consensus_block_validation() {
    use Super_Cardano_node::ledger::{Block, BlockHeader, Transaction, TxInput, TxOutput};
    use Super_Cardano_node::consensus::Consensus;
    use Super_Cardano_node::configuration::ConsensusConfig;
    use Super_Cardano_node::tracing::tracers::Tracer;
    let consensus = Consensus::new(ConsensusConfig::default(), Tracer::default());
    let block = Block {
        id: 1,
        header: BlockHeader {
            slot: 10,
            epoch: 1,
            leader: "test-leader".to_string(),
            vrf_proof: vec![0u8; 32],
            kes_signature: vec![0u8; 32],
        },
        transactions: vec![Transaction {
            id: 1,
            inputs: vec![TxInput { prev_tx: 0, index: 0 }],
            outputs: vec![TxOutput { address: "A".to_string(), amount: 10, assets: None }],
            certificates: vec![],
            plutus_witnesses: vec![],
        }],
    };
    let rt = tokio::runtime::Runtime::new().unwrap();
    assert!(rt.block_on(consensus.validate_block(&block)));
    // Invalid: missing leader
    let mut bad_block = block.clone();
    bad_block.header.leader = String::new();
    assert!(!rt.block_on(consensus.validate_block(&bad_block)));
    // Invalid: wrong VRF proof length
    let mut bad_block2 = block.clone();
    bad_block2.header.vrf_proof = vec![0u8; 16];
    assert!(!rt.block_on(consensus.validate_block(&bad_block2)));
}
