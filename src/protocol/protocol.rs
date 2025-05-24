// Minimal protocol struct for consensus/ledger
use crate::protocol::hard_fork_combinator::HardForkCombinator;
use std::sync::Arc;

pub struct Protocol {
    pub hard_fork: HardForkCombinator,
}

impl Protocol {
    /// Create a new Protocol instance with a given HardForkCombinator
    pub fn new(hard_fork: HardForkCombinator) -> Self {
        Self { hard_fork }
    }
}
