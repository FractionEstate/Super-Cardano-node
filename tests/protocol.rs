//! Tests for Cardano protocol eras and hard fork combinator.

use Super_Cardano_node::protocol::{Protocol, Era};
use Super_Cardano_node::configuration::ProtocolConfig;

#[test]
fn test_protocol_new_and_era_logic() {
    let config = ProtocolConfig { era: "Shelley".to_string() };
    let protocol = Protocol::new(config);
    assert!(matches!(protocol.hard_fork.current_era, Era::Shelley | Era::Byron | Era::Alonzo));
}
