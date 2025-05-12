//! Protocol module for Super Cardano Node
//!
//! Handles Cardano protocol logic, including multi-era support (Byron, Shelley, Alonzo, etc.).
//! All protocol code must be modular, extensible, and well-documented.

use crate::configuration::ProtocolConfig;
use serde::{Serialize, Deserialize};
use crate::protocol::byron::ByronProtocolInstantiationError;
use crate::protocol::shelley::ShelleyProtocolInstantiationError;
use crate::protocol::alonzo::AlonzoProtocolInstantiationError;
use crate::protocol::conway::ConwayProtocolInstantiationError;

pub mod byron;
pub mod shelley;
pub mod alonzo;
pub mod conway;
pub mod allegra;
pub mod mary;
pub mod babbage;
pub mod types;

pub use crate::protocol::allegra::*;
pub use crate::protocol::mary::*;
pub use crate::protocol::conway::*;
pub use crate::protocol::conway::ConwayProtocol;
pub use crate::protocol::babbage::*;
pub use crate::protocol::babbage::BabbageProtocol;

mod hard_fork_combinator;
pub use hard_fork_combinator::HardForkCombinator;

/// Supported Cardano protocol eras
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Era {
    Byron,
    Shelley,
    Allegra,
    Mary,
    Alonzo,
    Babbage,
    Conway,
    // ... add more as needed
}

impl Era {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "byron" => Era::Byron,
            "shelley" => Era::Shelley,
            "allegra" => Era::Allegra,
            "mary" => Era::Mary,
            "alonzo" => Era::Alonzo,
            "babbage" => Era::Babbage,
            "conway" => Era::Conway,
            _ => Era::Byron,
        }
    }
}

#[allow(dead_code)]
/// Trait for Cardano protocol eras (Byron, Shelley, Alonzo, etc.)
pub trait EraLogic: Send + Sync {
    fn name(&self) -> &'static str;
    fn validate_transaction(&self, tx: &crate::ledger::Transaction) -> bool;
    fn validate_block(&self, block: &crate::ledger::Block) -> bool;
    // Add more era-specific hooks as needed
}

/// Byron era logic
pub struct ByronEra;
impl EraLogic for ByronEra {
    fn name(&self) -> &'static str { "Byron" }
    fn validate_transaction(&self, tx: &crate::ledger::Transaction) -> bool {
        !tx.inputs.is_empty() && !tx.outputs.is_empty()
    }
    fn validate_block(&self, _block: &crate::ledger::Block) -> bool { true }
}

/// Shelley era logic
pub struct ShelleyEra;
impl EraLogic for ShelleyEra {
    fn name(&self) -> &'static str { "Shelley" }
    fn validate_transaction(&self, tx: &crate::ledger::Transaction) -> bool {
        !tx.inputs.is_empty() && tx.outputs.iter().all(|o| o.amount > 0)
    }
    fn validate_block(&self, _block: &crate::ledger::Block) -> bool { true }
}

/// Alonzo era logic
pub struct AlonzoEra;
impl EraLogic for AlonzoEra {
    fn name(&self) -> &'static str { "Alonzo" }
    fn validate_transaction(&self, tx: &crate::ledger::Transaction) -> bool {
        !tx.inputs.is_empty() && tx.outputs.iter().all(|o| o.amount > 0)
    }
    fn validate_block(&self, _block: &crate::ledger::Block) -> bool { true }
}

/// Allegra era logic
pub struct AllegraEra;
impl EraLogic for AllegraEra {
    fn name(&self) -> &'static str { "Allegra" }
    fn validate_transaction(&self, tx: &crate::ledger::Transaction) -> bool {
        !tx.inputs.is_empty() && tx.outputs.iter().all(|o| o.amount > 0)
    }
    fn validate_block(&self, _block: &crate::ledger::Block) -> bool { true }
}

/// Mary era logic
pub struct MaryEra;
impl EraLogic for MaryEra {
    fn name(&self) -> &'static str { "Mary" }
    fn validate_transaction(&self, tx: &crate::ledger::Transaction) -> bool {
        !tx.inputs.is_empty() && tx.outputs.iter().all(|o| o.amount > 0)
    }
    fn validate_block(&self, _block: &crate::ledger::Block) -> bool { true }
}

/// Babbage era logic
pub struct BabbageEra;
impl EraLogic for BabbageEra {
    fn name(&self) -> &'static str { "Babbage" }
    fn validate_transaction(&self, tx: &crate::ledger::Transaction) -> bool {
        // Babbage: like Alonzo, but could add reference inputs/scripts checks here
        !tx.inputs.is_empty() && tx.outputs.iter().all(|o| o.amount > 0)
    }
    fn validate_block(&self, _block: &crate::ledger::Block) -> bool { true }
}

impl Protocol {
    /// Create a new protocol handler with the given configuration.
    pub fn new(config: ProtocolConfig) -> Self {
        let initial_era = Era::from_str(&config.era);
        let initial_logic: Box<dyn EraLogic> = match initial_era {
            Era::Byron => Box::new(ByronEra),
            Era::Shelley => Box::new(ShelleyEra),
            Era::Allegra => Box::new(AllegraEra),
            Era::Mary => Box::new(MaryEra),
            Era::Alonzo => Box::new(AlonzoEra),
            Era::Babbage => Box::new(BabbageProtocol {}),
            Era::Conway => Box::new(ConwayProtocol {}),
        };

        let mut hard_fork = HardForkCombinator::new(initial_era, initial_logic);

        // Example: Schedule Shelley to Allegra transition at epoch 208
        hard_fork.schedule_transition(208, Era::Allegra, Box::new(AllegraEra));
        // Example: Schedule Allegra to Mary at epoch 236
        hard_fork.schedule_transition(236, Era::Mary, Box::new(MaryEra));
        // Example: Schedule Mary to Alonzo at epoch 251
        hard_fork.schedule_transition(251, Era::Alonzo, Box::new(AlonzoEra));
        // Example: Schedule Alonzo to Babbage at epoch 360
        hard_fork.schedule_transition(360, Era::Babbage, Box::new(BabbageProtocol {}));
        // Example: Schedule Babbage to Conway at epoch 400
        hard_fork.schedule_transition(400, Era::Conway, Box::new(ConwayProtocol {}));

        Self { config, hard_fork }
    }

    #[allow(dead_code)]
    /// Start the protocol logic asynchronously.
    pub async fn start(&self) {
        println!("[Protocol] Cardano protocol logic initialized (async). Supporting era: {:?}", self.hard_fork.current_era);
        // Example: Check for protocol upgrades, initialize era-specific state
    }

    /// Validate a transaction according to the current era's protocol rules.
    pub async fn validate_transaction(&self, tx: &crate::ledger::Transaction) -> bool {
        self.hard_fork.validate_transaction(tx)
    }

    /// Validate a block according to the current era's protocol rules.
    pub async fn validate_block(&self, block: &crate::ledger::Block) -> bool {
        self.hard_fork.validate_block(block)
    }

    /// Handle protocol upgrades (hard forks).
    pub async fn handle_upgrade(&mut self, current_epoch: u64) {
        self.hard_fork.check_transition(current_epoch);
    }

    /// Schedule a protocol upgrade (hard fork) to a new era at a given epoch.
    pub fn schedule_upgrade(&mut self, next_era: Era, epoch: u64) {
        self.hard_fork.schedule_transition(next_era, epoch);
    }
}

impl Default for Protocol {
    fn default() -> Self {
        Protocol::new(ProtocolConfig { era: "Shelley".to_string() })
    }
}

#[allow(dead_code)]
/// Error types for Cardano protocol instantiation.
#[derive(Clone, Debug)]
pub enum CardanoProtocolInstantiationError {
    Byron(ByronProtocolInstantiationError),
    Shelley(ShelleyProtocolInstantiationError),
    Alonzo(AlonzoProtocolInstantiationError),
    Conway(ConwayProtocolInstantiationError),
    PraosLeaderCredentialsError(String),
    CheckpointsReadError(String),
}

#[allow(dead_code)]
/// Protocol-specific handler trait for modular logic
pub trait ProtocolHandler: Send + Sync {
    fn on_block_received(&self, block: &crate::ledger::Block);
    fn on_transaction_received(&self, tx: &crate::ledger::Transaction);
    // Add more hooks as needed
}

// Example: Byron handler
pub struct ByronHandler;
impl ProtocolHandler for ByronHandler {
    fn on_block_received(&self, block: &crate::ledger::Block) {
        println!("[ByronHandler] Block received: {:?}", block.id);
    }
    fn on_transaction_received(&self, tx: &crate::ledger::Transaction) {
        println!("[ByronHandler] Tx received: {:?}", tx.id);
    }
}
