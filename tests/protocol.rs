//! Tests for Cardano protocol eras and hard fork combinator.

use crate::protocol::{Protocol, Era};
use crate::configuration::ProtocolConfig;

#[test]
fn test_protocol_new_and_era_logic() {
    let config = ProtocolConfig { era: "Shelley".to_string() };
    let protocol = Protocol::new(config);
    assert!(matches!(protocol.hard_fork.current_era, Era::Shelley | Era::Byron | Era::Alonzo));
}
