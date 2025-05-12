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
pub mod types;

pub use crate::protocol::allegra::*;
pub use crate::protocol::mary::*;
pub use crate::protocol::conway::*;
pub use crate::protocol::conway::ConwayProtocol;

/// Supported Cardano protocol eras
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Era {
    Byron,
    Shelley,
    Allegra,
    Mary,
    Alonzo,
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

#[allow(dead_code)]
/// Hard fork combinator: manages current era and transitions
pub struct HardForkCombinator {
    pub current_era: Era,
    pub era_logic: Box<dyn EraLogic>,
    pub next_era: Option<(Era, Box<dyn EraLogic>)>,
    pub transition_epoch: Option<u64>,
}

impl HardForkCombinator {
    /// Create a new hard fork combinator for the given era
    pub fn new(era: Era) -> Self {
        let era_logic: Box<dyn EraLogic> = match era {
            Era::Byron => Box::new(ByronEra),
            Era::Shelley => Box::new(ShelleyEra),
            Era::Allegra => Box::new(AllegraEra),
            Era::Mary => Box::new(MaryEra),
            Era::Alonzo => Box::new(AlonzoEra),
            Era::Conway => Box::new(ConwayProtocol {}),
        };
        Self {
            current_era: era,
            era_logic,
            next_era: None,
            transition_epoch: None,
        }
    }

    #[allow(dead_code)]
    /// Schedule a transition to a new era at a given epoch
    pub fn schedule_transition(&mut self, next_era: Era, epoch: u64) {
        let logic: Box<dyn EraLogic> = match next_era {
            Era::Byron => Box::new(ByronEra),
            Era::Shelley => Box::new(ShelleyEra),
            Era::Allegra => Box::new(AllegraEra),
            Era::Mary => Box::new(MaryEra),
            Era::Alonzo => Box::new(AlonzoEra),
            Era::Conway => Box::new(ConwayProtocol {}),
        };
        self.next_era = Some((next_era, logic));
        self.transition_epoch = Some(epoch);
    }

    #[allow(dead_code)]
    /// Check and perform era transition if the given epoch matches
    pub fn check_transition(&mut self, current_epoch: u64) {
        if let Some(epoch) = self.transition_epoch {
            if current_epoch >= epoch {
                if let Some((era, logic)) = self.next_era.take() {
                    self.current_era = era;
                    self.era_logic = logic;
                    self.transition_epoch = None;
                }
            }
        }
    }
}

/// Represents the protocol logic of the node, including multi-era support.
pub struct Protocol {
    #[allow(dead_code)]
    config: ProtocolConfig,
    pub hard_fork: HardForkCombinator,
}

#[allow(dead_code)]
impl Protocol {
    /// Create a new protocol handler with the given configuration.
    pub fn new(config: ProtocolConfig) -> Self {
        let era = Era::from_str(&config.era);
        let hard_fork = HardForkCombinator::new(era);
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
        self.hard_fork.era_logic.validate_transaction(tx)
    }

    /// Validate a block according to the current era's protocol rules.
    pub async fn validate_block(&self, block: &crate::ledger::Block) -> bool {
        self.hard_fork.era_logic.validate_block(block)
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
