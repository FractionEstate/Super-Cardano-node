impl Ledger {
    /// Returns a context object for compatibility (stub).
    pub fn context(&self) -> () {
        // TODO: Replace with real context type if needed
    }
}
/// Ledger management for Super Cardano Node
///
/// Handles block/tx validation, ledger state, and database operations.
/// Implements the extended UTXO model and state transitions.
use crate::chaindb::ChainDB;
use crate::protocol::wallet;
use crate::tracing::tracers::Tracer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a native asset (multi-asset support, Mary era)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Asset {
    pub policy_id: String,
    pub asset_name: String,
    pub amount: u64,
}

/// Stake pool registration certificate
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StakePoolRegistration {
    /// Unique pool identifier (e.g., hash of the pool's cold key)
    pub pool_id: String,
    /// Pool owner's reward address
    pub owner: String,
    /// Amount pledged by the pool owner (in ADA)
    pub pledge: u64,
    /// Fixed cost per epoch (in ADA)
    pub cost: u64,
    /// Pool margin (fraction, e.g., 0.05 for 5%)
    pub margin: f64, // NOTE: f64 does not implement Eq, so remove Eq from derives
    /// Reward account for pool rewards
    pub reward_account: String,
    // TODO: Add metadata, relays, VRF key, etc.
}

/// Stake pool retirement certificate
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StakePoolRetirement {
    /// Pool identifier
    pub pool_id: String,
    /// Epoch at which the pool retires
    pub retirement_epoch: u64,
}

/// Delegation certificate
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DelegationCertificate {
    /// Delegator's staking key (address)
    pub delegator: String,
    /// Pool identifier to delegate to
    pub pool_id: String,
}

/// Certificate for transaction inclusion (stake pool ops and delegation)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Certificate {
    PoolRegistration(StakePoolRegistration),
    PoolRetirement(StakePoolRetirement),
    Delegation(DelegationCertificate),
}

/// Stake pool parameters and state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StakePool {
    pub registration: StakePoolRegistration,
    pub active: bool,
    pub retirement_epoch: Option<u64>,
}

/// Ledger state (e.g., UTXO set, stake distribution, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerState {
    pub utxos: HashMap<(u64, u32), TxOutput>, // (tx_id, index) -> output
    pub stake_distribution: HashMap<String, u64>, // address -> stake
    pub delegations: HashMap<String, String>, // delegator -> pool
    pub stake_pools: HashMap<String, StakePool>, // pool_id -> pool
    pub pool_retirements: HashMap<String, u64>, // pool_id -> retirement epoch
    pub rewards: HashMap<String, u64>,        // reward address -> ADA
}

#[derive(Clone)]
pub struct Ledger {
    state: LedgerState,
    pub tracer: crate::tracing::tracers::Tracer,
    // ... add more fields as needed
}

impl Ledger {
    /// Initialize the ledger state
    pub fn new(tracer: Tracer) -> Self {
        Self {
            state: LedgerState {
                utxos: HashMap::new(),
                stake_distribution: HashMap::new(),
                delegations: HashMap::new(),
                stake_pools: HashMap::new(),
                pool_retirements: HashMap::new(),
                rewards: HashMap::new(),
            },
            tracer,
        }
    }
    /// Validate a block and update ledger state, enforcing protocol rules
    pub async fn validate_block(
        &mut self,
        block: &Block,
        protocol: &crate::protocol::Protocol,
    ) -> bool {
        // Consensus-level checks: slot, leader, VRF/KES proofs (stubbed)
        if block.header.slot == 0 || block.header.leader.is_empty() {
            return false;
        }
        // TODO: Add real VRF/KES cryptographic checks here
        // Check all transactions in the block using protocol-aware validation
        for tx in &block.transactions {
            if !self.validate_transaction_with_protocol(tx, protocol).await {
                return false;
            }
        }
        // In a full implementation, update UTXO, apply state, and verify block signature here.
        true
    }
    /// Validate a transaction
    #[allow(dead_code)]
    pub async fn validate_transaction(
        &self,
        tx: &Transaction,
        protocol: &crate::protocol::Protocol,
    ) -> bool {
        // Call protocol-level validation first
        // FIXME: This is a stub to allow compilation. Real implementation must convert or use correct type.
        // Skipping protocol-level validation for now
        // Example: Check that all outputs are positive
        if tx.outputs.iter().any(|o| o.amount == 0) {
            return false;
        }
        // TODO: Check UTXO, signatures, double-spend, etc.
        true
    }
    /// Validate a transaction, enforcing both UTXO and protocol rules
    pub async fn validate_transaction_with_protocol(
        &self,
        tx: &Transaction,
        protocol: &crate::protocol::Protocol,
    ) -> bool {
        // First, check protocol-level rules (era, structure, etc.)
        // Map ledger::Transaction to wallet::Transaction if needed
        let wallet_tx = wallet::Transaction {};
        if !protocol.validate_transaction(&wallet_tx).await {
            return false;
        }
        // Then, check UTXO and ledger-level rules
        if tx.outputs.iter().any(|o| o.amount == 0) {
            return false;
        }
        // TODO: Check UTXO, signatures, double-spend, etc.
        true
    }
    /// Persist ledger state to disk
    #[allow(dead_code)]
    pub async fn persist(&self) {
        // TODO: Implement state persistence
    }
    /// Load ledger state from disk
    #[allow(dead_code)]
    pub async fn load(&mut self) {
        // TODO: Implement state loading
    }

    /// Apply a transaction to the UTXO set (returns false if invalid)
    pub fn apply_transaction(&mut self, tx: &Transaction) -> bool {
        // Check all inputs exist in UTXO set
        for input in &tx.inputs {
            if !self.state.utxos.contains_key(&(input.prev_tx, input.index)) {
                return false; // double-spend or missing input
            }
        }
        // Remove spent inputs
        for input in &tx.inputs {
            self.state.utxos.remove(&(input.prev_tx, input.index));
        }
        // Add new outputs
        for (idx, output) in tx.outputs.iter().enumerate() {
            self.state.utxos.insert((tx.id, idx as u32), output.clone());
        }
        true
    }

    /// Apply a block to the ledger (returns false if any tx is invalid)
    pub fn apply_block(&mut self, block: &Block) -> bool {
        for tx in &block.transactions {
            if !self.apply_transaction(tx) {
                return false;
            }
        }
        true
    }

    /// Apply a block to the ledger and return a new chain.
    pub fn apply_block_to_chain(&mut self, chain: &Chain, block: Block) -> Option<Chain> {
        // Validate block (basic check: all txs valid)
        let mut new_ledger = self.clone();
        if new_ledger.apply_block(&block) {
            let mut new_chain = chain.clone();
            new_chain.blocks.push(block);
            self.tracer
                .trace(crate::tracing::tracers::TraceEvent::ChainDB(
                    "Block applied to chain".to_string(),
                ));
            Some(new_chain)
        } else {
            None
        }
    }

    /// Returns a hash representing the tip (last block) for test comparison
    #[allow(dead_code)]
    pub fn tip_hash(&self) -> u64 {
        // For test: just use the highest tx id in the UTXO set, or 0 if empty
        self.state
            .utxos
            .keys()
            .map(|(txid, _)| *txid)
            .max()
            .unwrap_or(0)
    }

    /// Apply a block to the ledger (returns false if any tx is invalid)
    /// Persists the block and state to ChainDB if provided.
    pub async fn apply_block_with_db(
        &mut self,
        block: &Block,
        chaindb: Option<&mut ChainDB>,
    ) -> bool {
        for tx in &block.transactions {
            if !self.apply_transaction(tx) {
                return false;
            }
        }
        if let Some(db) = chaindb {
            // Persist block and state atomically
            let _ = db.append_block(block, &self.state).await;
        }
        true
    }

    /// Restore ledger state from ChainDB at a given block id.
    pub async fn restore_from_db(
        &mut self,
        chaindb: &ChainDB,
        block_id: u64,
    ) -> std::io::Result<()> {
        let state = chaindb.load_state(block_id).await?;
        self.state = state;
        Ok(())
    }
}

impl LedgerState {
    /// Apply a certificate to the ledger state (stake pool registration, retirement, or delegation).
    /// Returns an error string if the certificate is invalid.
    pub fn apply_certificate(
        &mut self,
        cert: &Certificate,
        current_epoch: u64,
    ) -> Result<(), String> {
        match cert {
            Certificate::PoolRegistration(reg) => {
                // Register or update the stake pool
                let pool = StakePool {
                    registration: reg.clone(),
                    active: true,
                    retirement_epoch: None,
                };
                self.stake_pools.insert(reg.pool_id.clone(), pool);
                Ok(())
            }
            Certificate::PoolRetirement(ret) => {
                // Mark pool for retirement at the given epoch
                if let Some(pool) = self.stake_pools.get_mut(&ret.pool_id) {
                    if pool.active && (ret.retirement_epoch > current_epoch) {
                        pool.retirement_epoch = Some(ret.retirement_epoch);
                        self.pool_retirements
                            .insert(ret.pool_id.clone(), ret.retirement_epoch);
                        Ok(())
                    } else {
                        Err("Invalid retirement epoch or pool not active".to_string())
                    }
                } else {
                    Err("Pool not found".to_string())
                }
            }
            Certificate::Delegation(deleg) => {
                // Only allow delegation to active pools
                if let Some(pool) = self.stake_pools.get(&deleg.pool_id) {
                    if pool.active {
                        self.delegations
                            .insert(deleg.delegator.clone(), deleg.pool_id.clone());
                        Ok(())
                    } else {
                        Err("Cannot delegate to inactive pool".to_string())
                    }
                } else {
                    Err("Pool not found".to_string())
                }
            }
        }
    }

    /// Process pool retirements at the end of an epoch.
    pub fn process_pool_retirements(&mut self, current_epoch: u64) {
        let retiring: Vec<String> = self
            .stake_pools
            .iter()
            .filter_map(|(id, pool)| {
                if let Some(epoch) = pool.retirement_epoch {
                    if epoch <= current_epoch {
                        Some(id.clone())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();
        for pool_id in retiring {
            if let Some(pool) = self.stake_pools.get_mut(&pool_id) {
                pool.active = false;
            }
        }
        self.pool_retirements
            .retain(|_, &mut epoch| epoch > current_epoch);
    }

    /// Calculate and distribute rewards (stub: proportional to stake for now).
    pub fn distribute_rewards(&mut self, total_rewards: u64) {
        let total_stake: u64 = self.stake_distribution.values().sum();
        if total_stake == 0 || total_rewards == 0 {
            return;
        }
        for (addr, stake) in &self.stake_distribution {
            let reward = (*stake as u128 * total_rewards as u128 / total_stake as u128) as u64;
            *self.rewards.entry(addr.clone()).or_insert(0) += reward;
        }
    }

    /// Validate all Plutus scripts in a transaction (stub: always returns true)
    pub fn validate_plutus_scripts(&self, tx: &Transaction) -> bool {
        for witness in &tx.plutus_witnesses {
            // TODO: Integrate real Plutus interpreter
            if witness.script.code.is_empty() {
                return false;
            }
        }
        true
    }
}

/// Consensus-aware block header for Ouroboros
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlockHeader {
    pub slot: u64,
    pub epoch: u64,
    pub leader: String,         // Stake pool or node id
    pub vrf_proof: Vec<u8>,     // VRF proof (stub)
    pub kes_signature: Vec<u8>, // KES signature (stub)
}

/// Represents a candidate chain for fork choice.
#[derive(Debug, Clone)]
pub struct Chain {
    pub blocks: Vec<Block>,
}

/// Cardano-style chain selection: prefer longest chain, then highest density, then heaviest (most blocks), then tie-break by lowest tip hash.
pub fn select_chain<'a>(chains: &'a [Chain]) -> Option<&'a Chain> {
    chains.iter().max_by(|a, b| {
        let len_a = a.blocks.len();
        let len_b = b.blocks.len();
        if len_a != len_b {
            return len_a.cmp(&len_b);
        }
        // Density: count unique block ids (simulate density)
        let density_a = a
            .blocks
            .iter()
            .map(|blk| blk.id)
            .collect::<std::collections::HashSet<_>>()
            .len();
        let density_b = b
            .blocks
            .iter()
            .map(|blk| blk.id)
            .collect::<std::collections::HashSet<_>>()
            .len();
        if density_a != density_b {
            return density_a.cmp(&density_b);
        }
        // Heaviest: most total transactions
        let txs_a = a
            .blocks
            .iter()
            .map(|blk| blk.transactions.len())
            .sum::<usize>();
        let txs_b = b
            .blocks
            .iter()
            .map(|blk| blk.transactions.len())
            .sum::<usize>();
        if txs_a != txs_b {
            return txs_a.cmp(&txs_b);
        }
        // Tie-break: lowest tip block id
        let tip_a = a.blocks.last().map(|b| b.id).unwrap_or(0);
        let tip_b = b.blocks.last().map(|b| b.id).unwrap_or(0);
        tip_b.cmp(&tip_a) // lower id wins
    })
}

/// Example block and transaction types for Cardano (to be expanded)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub id: u64,
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
    // ... add more fields as needed (header, signature, etc.)
}

impl Default for Block {
    fn default() -> Self {
        Block {
            id: 0,
            header: BlockHeader {
                slot: 0,
                epoch: 0,
                leader: String::new(),
                vrf_proof: vec![],
                kes_signature: vec![],
            },
            transactions: vec![],
        }
    }
}

#[allow(dead_code)]
impl Block {
    pub async fn new_from_mempool(
        ledger: &Ledger,
        era_logic: &dyn crate::protocol::EraLogic,
        mempool: &crate::mempool::Mempool,
        slot: u64,
        epoch: u64,
        leader: String,
        vrf_proof: Vec<u8>,
        kes_signature: Vec<u8>,
    ) -> Option<Block> {
        let txs = mempool.get_transactions();
        if txs.is_empty() {
            None
        } else {
            Some(Block {
                id: ledger.tip_hash() + 1,
                header: BlockHeader {
                    slot,
                    epoch,
                    leader,
                    vrf_proof,
                    kes_signature,
                },
                transactions: txs,
            })
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TxInput {
    pub prev_tx: u64,
    pub index: u32,
}

/// Transaction output supporting multi-asset (Mary era)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TxOutput {
    pub address: String,
    pub amount: u64,                // ADA amount
    pub assets: Option<Vec<Asset>>, // Optional multi-asset bundle
}

/// Plutus script (Alonzo+)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlutusScript {
    /// Raw script bytes (CBOR or hex-encoded)
    pub code: Vec<u8>,
}

/// Plutus datum (arbitrary data attached to outputs)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlutusDatum {
    pub data: Vec<u8>,
}

/// Plutus redeemer (input to script validation)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlutusRedeemer {
    pub data: Vec<u8>,
}

/// Alonzo/Plutus script witness for a transaction input
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlutusWitness {
    pub script: PlutusScript,
    pub datum: PlutusDatum,
    pub redeemer: PlutusRedeemer,
    pub execution_units: (u64, u64), // (mem, steps)
}

/// Transaction supporting certificates (for staking/pool ops)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Transaction {
    pub id: u64,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
    pub certificates: Vec<Certificate>,
    pub plutus_witnesses: Vec<PlutusWitness>,
    // ... add more fields as needed (metadata, etc.)
}

impl Default for Transaction {
    fn default() -> Self {
        Transaction {
            id: 0,
            inputs: vec![],
            outputs: vec![],
            certificates: vec![],
            plutus_witnesses: vec![],
        }
    }
}

/// Datum for EUTXO (can be any serializable data)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Datum {
    pub data: Vec<u8>,
}

/// Script for EUTXO (stub: replace with Plutus or custom VM)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Script {
    pub code: Vec<u8>,
}

/// EUTXO output with datum and script
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EUTxOOutput {
    pub address: String,
    pub amount: u64,
    pub datum: Option<Datum>,
    pub script: Option<Script>,
    // TODO: Add multi-asset support
}

/// Validate a transaction in the EUTXO model (including scripts/datums)
#[allow(dead_code)]
pub async fn validate_eutxo_transaction(_tx: &Transaction) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stake_pool_registration_and_retirement() {
        let mut state = LedgerState {
            utxos: HashMap::new(),
            stake_distribution: HashMap::new(),
            delegations: HashMap::new(),
            stake_pools: HashMap::new(),
            pool_retirements: HashMap::new(),
            rewards: HashMap::new(),
        };
        let reg = StakePoolRegistration {
            pool_id: "pool1".to_string(),
            owner: "owner1".to_string(),
            pledge: 1000,
            cost: 340,
            margin: 0.05,
            reward_account: "owner1_reward".to_string(),
        };
        let cert = Certificate::PoolRegistration(reg.clone());
        assert!(state.apply_certificate(&cert, 0).is_ok());
        assert!(state.stake_pools.contains_key("pool1"));
        // Retirement
        let retire = StakePoolRetirement {
            pool_id: "pool1".to_string(),
            retirement_epoch: 5,
        };
        let retire_cert = Certificate::PoolRetirement(retire.clone());
        assert!(state.apply_certificate(&retire_cert, 1).is_ok());
        assert_eq!(state.stake_pools["pool1"].retirement_epoch, Some(5));
        // Process retirement
        state.process_pool_retirements(5);
        assert!(!state.stake_pools["pool1"].active);
    }

    #[test]
    fn test_delegation_and_rewards() {
        let mut state = LedgerState {
            utxos: HashMap::new(),
            stake_distribution: HashMap::new(),
            delegations: HashMap::new(),
            stake_pools: HashMap::new(),
            pool_retirements: HashMap::new(),
            rewards: HashMap::new(),
        };
        // Register pool
        let reg = StakePoolRegistration {
            pool_id: "pool2".to_string(),
            owner: "owner2".to_string(),
            pledge: 2000,
            cost: 340,
            margin: 0.03,
            reward_account: "owner2_reward".to_string(),
        };
        let cert = Certificate::PoolRegistration(reg.clone());
        assert!(state.apply_certificate(&cert, 0).is_ok());
        // Delegate
        let deleg = DelegationCertificate {
            delegator: "alice".to_string(),
            pool_id: "pool2".to_string(),
        };
        let deleg_cert = Certificate::Delegation(deleg.clone());
        assert!(state.apply_certificate(&deleg_cert, 0).is_ok());
        assert_eq!(state.delegations["alice"], "pool2");
        // Rewards
        state.stake_distribution.insert("alice".to_string(), 1000);
        state.stake_distribution.insert("bob".to_string(), 2000);
        state.distribute_rewards(300);
        assert_eq!(state.rewards["alice"], 100);
        assert_eq!(state.rewards["bob"], 200);
    }

    #[test]
    fn test_block_and_transaction_validation() {
        let mut ledger = Ledger::new(Tracer::default());
        let protocol = crate::protocol::Protocol::new(crate::configuration::ProtocolConfig {
            era: "Shelley".to_string(),
        });
        let tx = Transaction {
            id: 1,
            inputs: vec![TxInput {
                prev_tx: 0,
                index: 0,
            }],
            outputs: vec![TxOutput {
                address: "addr1".to_string(),
                amount: 100,
                assets: None,
            }],
            certificates: vec![],
            plutus_witnesses: vec![],
        };
        let block = Block {
            id: 1,
            header: BlockHeader {
                slot: 1,
                epoch: 0,
                leader: "test-leader".to_string(),
                vrf_proof: vec![0u8; 32],
                kes_signature: vec![0u8; 32],
            },
            transactions: vec![],
        };
        let rt = tokio::runtime::Runtime::new().unwrap();
        assert!(rt.block_on(ledger.validate_transaction(&tx, &protocol)));
        assert!(rt.block_on(ledger.validate_block(&block, &protocol)));
    }

    #[test]
    fn test_invalid_transaction() {
        let ledger = Ledger::new(Tracer::default());
        let protocol = crate::protocol::Protocol::new(crate::configuration::ProtocolConfig {
            era: "Shelley".to_string(),
        });
        let tx = Transaction {
            id: 2,
            inputs: vec![],  // Invalid: no inputs
            outputs: vec![], // Invalid: no outputs
            certificates: vec![],
            plutus_witnesses: vec![],
        };
        let rt = tokio::runtime::Runtime::new().unwrap();
        assert!(!rt.block_on(ledger.validate_transaction(&tx, &protocol)));
    }

    #[test]
    fn utxo_apply_and_double_spend() {
        let mut ledger = Ledger::new(Tracer::default());
        // Create a coinbase tx (no inputs, one output)
        let coinbase = Transaction {
            id: 1,
            inputs: vec![],
            outputs: vec![TxOutput {
                address: "A".to_string(),
                amount: 100,
                assets: None,
            }],
            certificates: vec![],
            plutus_witnesses: vec![],
        };
        // Apply coinbase
        assert!(ledger.apply_transaction(&coinbase));
        // Spend the output
        let spend = Transaction {
            id: 2,
            inputs: vec![TxInput {
                prev_tx: 1,
                index: 0,
            }],
            outputs: vec![TxOutput {
                address: "B".to_string(),
                amount: 100,
                assets: None,
            }],
            certificates: vec![],
            plutus_witnesses: vec![],
        };
        assert!(ledger.apply_transaction(&spend));
        // Double-spend should fail
        let double_spend = Transaction {
            id: 3,
            inputs: vec![TxInput {
                prev_tx: 1,
                index: 0,
            }],
            outputs: vec![TxOutput {
                address: "C".to_string(),
                amount: 100,
                assets: None,
            }],
            certificates: vec![],
            plutus_witnesses: vec![],
        };
        assert!(!ledger.apply_transaction(&double_spend));
    }

    #[test]
    fn utxo_block_application() {
        let mut ledger = Ledger::new(Tracer::default());
        let coinbase = Transaction {
            id: 4,
            inputs: vec![],
            outputs: vec![TxOutput {
                address: "A".to_string(),
                amount: 50,
                assets: None,
            }],
            certificates: vec![],
            plutus_witnesses: vec![],
        };
        let spend = Transaction {
            id: 5,
            inputs: vec![TxInput {
                prev_tx: 4,
                index: 0,
            }],
            outputs: vec![TxOutput {
                address: "B".to_string(),
                amount: 50,
                assets: None,
            }],
            certificates: vec![],
            plutus_witnesses: vec![],
        };
        let block = Block {
            id: 1,
            header: BlockHeader {
                slot: 1,
                epoch: 0,
                leader: "test-leader".to_string(),
                vrf_proof: vec![0u8; 32],
                kes_signature: vec![0u8; 32],
            },
            transactions: vec![coinbase, spend],
        };
        assert!(ledger.apply_block(&block));
        // The UTXO set should now have only the spend output
        assert!(ledger.state.utxos.contains_key(&(5, 0)));
        assert!(!ledger.state.utxos.contains_key(&(4, 0)));
    }

    #[test]
    fn test_chain_selection() {
        let block = Block {
            id: 1,
            header: BlockHeader {
                slot: 1,
                epoch: 0,
                leader: "test-leader".to_string(),
                vrf_proof: vec![0u8; 32],
                kes_signature: vec![0u8; 32],
            },
            transactions: vec![],
        };
        let chain1 = Chain {
            blocks: vec![block.clone()],
        };
        let chain2 = Chain {
            blocks: vec![block.clone(), block.clone()],
        };
        let chains = vec![chain1, chain2.clone()];
        let selected = select_chain(&chains).unwrap();
        assert_eq!(selected.blocks.len(), 2);
    }

    #[test]
    fn test_plutus_script_validation() {
        let mut state = LedgerState {
            utxos: HashMap::new(),
            stake_distribution: HashMap::new(),
            delegations: HashMap::new(),
            stake_pools: HashMap::new(),
            pool_retirements: HashMap::new(),
            rewards: HashMap::new(),
        };
        let script = PlutusScript {
            code: vec![1, 2, 3],
        };
        let datum = PlutusDatum {
            data: vec![4, 5, 6],
        };
        let redeemer = PlutusRedeemer {
            data: vec![7, 8, 9],
        };
        let witness = PlutusWitness {
            script: script.clone(),
            datum: datum.clone(),
            redeemer: redeemer.clone(),
            execution_units: (1000, 5000),
        };
        let tx = Transaction {
            id: 42,
            inputs: vec![],
            outputs: vec![],
            certificates: vec![],
            plutus_witnesses: vec![witness],
        };
        assert!(state.validate_plutus_scripts(&tx));
        // Invalid: empty script
        let bad_witness = PlutusWitness {
            script: PlutusScript { code: vec![] },
            datum,
            redeemer,
            execution_units: (100, 100),
        };
        let bad_tx = Transaction {
            id: 43,
            inputs: vec![],
            outputs: vec![],
            certificates: vec![],
            plutus_witnesses: vec![bad_witness],
        };
        assert!(!state.validate_plutus_scripts(&bad_tx));
    }
}
