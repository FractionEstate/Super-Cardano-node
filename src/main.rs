//! Main entrypoint for Super Cardano Node
//!
//! Initializes configuration, networking, consensus, protocol, and ledger.
//! Uses Tokio async runtime for concurrency.

mod configuration;
mod networking;
mod consensus;
mod protocol;
mod ledger;
mod handlers;
mod queries;
mod mempool;
mod tracing;
mod chaindb;
mod api;
mod api_grpc;

use configuration::{Cli, Config, NetworkConfig, ConsensusConfig, ProtocolConfig};
use networking::Network;
use consensus::Consensus;
use protocol::Protocol;
use ledger::{Ledger, Chain, select_chain};
use std::sync::Arc;

use clap::Parser;

use tokio::runtime::Runtime;
use tokio::task;
use tokio::net::TcpListener;

use handlers::Handlers;
use crate::tracing::Tracing;
use api::rest_router;
use api_grpc::start_grpc_server;
use std::net::SocketAddr;
use Super_Cardano_node::chaindb::{ChainDB, SharedChainDB};
use crate::wallet;

fn main() {
    let cli = Cli::parse();
    if cli.version {
        println!("Super Cardano Node v0.1.0");
        return;
    }
    let config = if let Some(ref path) = cli.config {
        Config::load_from_file(path).unwrap_or_else(|e| panic!("Config error: {}", e))
    } else {
        // Fallback to default config for now
        Config {
            network: NetworkConfig { bind_addr: "127.0.0.1:3001".to_string(), max_peers: 8, discovery: None },
            consensus: ConsensusConfig { protocol: "Ouroboros".to_string() },
            protocol: ProtocolConfig { era: "Shelley".to_string() },
        }
    };
    let tracing = Tracing::new();
    let tracer = tracing.tracer.clone();

    let network = Network::new(config.network.clone(), tracer.clone());
    let mut consensus = Consensus::new(config.consensus.clone(), tracer.clone());
    consensus.mempool = Some(mempool::Mempool::new(1024));
    let protocol = Protocol::new(config.protocol.clone());
    let ledger = Ledger::new(tracer.clone());

    // Use Tokio async runtime for modern, efficient concurrency
    let rt = Runtime::new().unwrap();
    rt.block_on(async move {
        // Wrap shared state in Arc/Mutex for networking
        let ledger = Arc::new(tokio::sync::Mutex::new(ledger));
        let protocol = Arc::new(protocol);
        let network = Arc::new(network);
        // Wrap mempool in Arc<tokio::sync::Mutex<_>> for async sharing
        let mempool = Arc::new(tokio::sync::Mutex::new(consensus.mempool.clone().unwrap()));
        // Start networking in a background task, passing shared state
        let network_task = {
            let network = Arc::clone(&network);
            let ledger = Arc::clone(&ledger);
            let protocol = Arc::clone(&protocol);
            let mempool = Arc::clone(&mempool);
            task::spawn(async move {
                // Use the new start_with_state method
                network.start_with_state(ledger, mempool, protocol).await;
            })
        };
        // Main consensus loop: periodically try to produce a block from the mempool
        let consensus_task = {
            let network = Arc::clone(&network);
            let protocol = Arc::clone(&protocol);
            let ledger = Arc::clone(&ledger);
            task::spawn(async move {
                let mut chains = vec![Chain { blocks: vec![] }]; // Track candidate chains
                loop {
                    let mut ledger_guard = ledger.lock().await;
                    // Provide consensus header fields for block production
                    let slot = 0;
                    let epoch = 0;
                    let leader = "main-thread".to_string();
                    let vrf_proof = vec![0u8; 32];
                    let kes_signature = vec![0u8; 32];
                    if let Some(block) = consensus.produce_block_from_mempool(
                        &mut *ledger_guard,
                        &protocol,
                        slot,
                        epoch,
                        leader,
                        vrf_proof,
                        kes_signature,
                    ).await {
                        println!("[Consensus] Produced block from mempool: {:?}", block);
                        network.broadcast_block(&block).await;
                        let mut new_chains = vec![];
                        for chain in &chains {
                            if let Some(new_chain) = ledger_guard.apply_block_to_chain(chain, block.clone()) {
                                new_chains.push(new_chain);
                            }
                        }
                        if let Some(best_chain) = select_chain(&new_chains) {
                            println!("[Consensus] Selected chain with {} blocks", best_chain.blocks.len());
                            chains = new_chains;
                        }
                    }
                    drop(ledger_guard);
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                }
            })
        };
        // Register graceful shutdown signals
        let shutdown_task = tokio::spawn(async {
            Handlers::register_signals().await;
        });

        // --- ChainDB API server startup ---
        let chaindb = Arc::new(tokio::sync::RwLock::new(
            crate::chaindb::ChainDB::open("./testdata/chaindb").await.unwrap()
        ));
        // REST API (axum)
        let rest_addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        let rest_db: crate::chaindb::SharedChainDB = std::sync::Arc::new(tokio::sync::RwLock::new(crate::chaindb::ChainDB::open("./data/chaindb").await.unwrap()));
        let wallets: std::sync::Arc<tokio::sync::RwLock<Vec<wallet::SharedWallet>>> = std::sync::Arc::new(tokio::sync::RwLock::new(Vec::new()));
        let app_state: crate::api::SharedAppState = std::sync::Arc::new(tokio::sync::RwLock::new(crate::api::AppState {
            db: rest_db.clone(),
            wallets: wallets.clone(),
        }));
        tokio::spawn(async move {
            let app = rest_router(app_state);
            println!("[API] REST server listening on http://{}", rest_addr);
            let listener = TcpListener::bind(rest_addr).await.unwrap();
            axum::serve(listener, app).await.unwrap();
        });
        // gRPC API (tonic)
        let grpc_addr: SocketAddr = "127.0.0.1:50051".parse().unwrap();
        let grpc_db: crate::chaindb::SharedChainDB = rest_db.clone();
        tokio::spawn(async move {
            println!("[API] gRPC server listening on http://{}", grpc_addr);
            start_grpc_server(grpc_db, grpc_addr).await.unwrap();
        });

        // Wait for tasks (networking, consensus, shutdown)
        let _ = futures::join!(network_task, consensus_task, shutdown_task);

        tracing.shutdown();
    });
}
