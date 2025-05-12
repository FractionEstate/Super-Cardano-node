//! Hard Fork Combinator for Super Cardano Node
//!
//! Manages transitions between different Cardano protocol eras.

use crate::protocol::{Era, EraLogic};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct HardForkTransition {
    pub era: Era,
    pub activation_epoch: u64,
    pub logic: Box<dyn EraLogic>,
}

/// Manages era transitions and delegates validation logic to the current era.
pub struct HardForkCombinator {
    current_era: Era,
    current_logic: Box<dyn EraLogic>,
    transitions: BTreeMap<u64, HardForkTransition>,
}

impl HardForkCombinator {
    /// Initialize the combinator with the initial era.
    pub fn new(initial_era: Era, initial_logic: Box<dyn EraLogic>) -> Self {
        Self {
            current_era: initial_era,
            current_logic: initial_logic,
            transitions: BTreeMap::new(),
        }
    }

    /// Schedule a new era transition.
    pub fn schedule_transition(&mut self, epoch: u64, era: Era, logic: Box<dyn EraLogic>) {
        self.transitions.insert(epoch, HardForkTransition { era, activation_epoch: epoch, logic });
    }

    /// Check and perform era transition if needed.
    pub fn check_transition(&mut self, current_epoch: u64) {
        if let Some((&epoch, transition)) = self.transitions.iter().next() {
            if current_epoch >= epoch {
                self.current_era = transition.era.clone();
                self.current_logic = transition.logic.clone();
                self.transitions.remove(&epoch);
            }
        }
    }

    /// Delegate transaction validation to the current era logic.
    pub fn validate_transaction(&self, tx: &crate::ledger::Transaction) -> bool {
        self.current_logic.validate_transaction(tx)
    }

    /// Delegate block validation to the current era logic.
    pub fn validate_block(&self, block: &crate::ledger::Block) -> bool {
        self.current_logic.validate_block(block)
    }

    /// Get the current era.
    pub fn current_era(&self) -> &Era {
        &self.current_era
    }
}
