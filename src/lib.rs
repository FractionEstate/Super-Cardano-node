#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused)]
//! # Super Cardano Node
//!
//! This crate implements a modern, high-performance Cardano node in Rust, supporting multi-era protocol logic, Ouroboros consensus, extended UTXO, and robust networking.
//!
//! ## Features
//! - Modular architecture: configuration, networking, consensus, protocol, ledger, tracing, handlers, queries, and more.
//! - Cardano protocol support: Byron, Shelley, Alonzo, Conway eras.
//! - Ouroboros consensus algorithm (Praos, BFT, etc.).
//! - Extended UTXO model.
//! - Async/await networking with Tokio.
//! - Secure, testable, and extensible codebase.
//!
//! ## Usage Example
//! ```rust
//! use Super_Cardano_node::configuration::Config;
//! use Super_Cardano_node::ledger::Ledger;
//! use Super_Cardano_node::protocol::Protocol;
//! // ...
//! ```
//!
//! ## Modules
//! - [`configuration`]: Node configuration and CLI parsing.
//! - [`networking`]: Peer-to-peer networking and relay logic.
//! - [`consensus`]: Ouroboros consensus and slot leadership.
//! - [`protocol`]: Cardano protocol eras and hard fork combinator.
//! - [`ledger`]: UTXO, block/tx validation, and state transitions.
//! - [`tracing`]: Structured logging, metrics, and diagnostics.
//! - [`handlers`]: Node event and error handlers.
//! - [`queries`]: Node state and chain queries.
//! - [`chaindb`]: Persistent on-disk chain database for blocks, UTXOs, and state.
//!
//! ## Testing
//! Run all tests:
//! ```sh
//! cargo test
//! ```
//!
//! ## Security
//! - All external input is validated and sanitized.
//! - Panics are avoided in production code.
//! - Minimal unsafe code, with justification.
//!
//! ## License
//! MIT OR Apache-2.0

#[cfg(test)]
mod tests {
    // Removed: use super::*;
    // Fix test import: use crate::configuration::{...} for correct path in integration tests
    use crate::configuration::{Config, NetworkConfig, ConsensusConfig, ProtocolConfig};

    #[test]
    fn test_config_validation() {
        let config = Config {
            network: NetworkConfig { bind_addr: "127.0.0.1:3001".to_string(), max_peers: 8, discovery: None },
            consensus: ConsensusConfig { protocol: "Ouroboros".to_string() },
            protocol: ProtocolConfig { era: "Shelley".to_string() },
        };
        assert!(config.validate());
    }
}

pub mod configuration;
pub mod consensus;
pub mod handlers;
pub mod ledger;
pub mod mempool;
pub mod networking;
pub mod protocol;
pub mod queries;
pub mod tracing;
pub mod chaindb;
pub mod proto_convert;
pub mod wallet;
pub mod api;

pub mod chaindb_proto {
    tonic::include_proto!("chaindb");
}

pub use crate::api::{AppState, SharedAppState};
