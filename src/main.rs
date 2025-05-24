mod lifecycle;
use crate::lifecycle::{initialize_app_state, setup_graceful_shutdown, start_services};
/// Main entrypoint for Super Cardano Node
///
/// Initializes configuration, networking, consensus, protocol, and ledger.
/// Uses Tokio async runtime for concurrency.
mod api;
mod api_grpc;
mod chaindb;
mod configuration;
mod consensus;
mod handlers;
mod ledger;
mod mempool;
mod networking;
mod protocol;
mod queries;
mod tracing;

use configuration::types::Configuration as Config;
use configuration::{ConsensusConfig, NetworkConfig};
use consensus::Consensus;
use ledger::{Chain, Ledger, select_chain};
use networking::Network;
use protocol::Protocol;
use std::sync::Arc;

use clap::Parser;

use tokio::net::TcpListener;
use tokio::runtime::Runtime;
use tokio::task;
use tokio::time::{Duration, sleep};

use crate::tracing::Tracing;
use handlers::Handlers;
// use api::rest_router; // No rest_router defined in api/rest.rs
use crate::chaindb::{ChainDB, SharedChainDB};
use api_grpc::start_grpc_server;
use std::net::SocketAddr;
// use crate::wallet; // Use correct wallet path or remove if not used
use anyhow::{Context, Result};
use tracing::{debug, error, info, warn};

fn main() -> Result<()> {
    // TODO: Fix CLI parsing and ProtocolConfig usage. Currently, only Config, NetworkConfig, ConsensusConfig are available.
    // Initialize tracing early for better diagnostics during startup
    let tracing = Tracing::new();
    let tracer = tracing.tracer.clone();

    // Parse CLI arguments with improved error context
    // TEMP: Replace with real CLI parsing
    let cli = Config::default();
    let config = Config::default();

    info!(
        "Starting Super Cardano Node with {} consensus protocol",
        config.consensus.protocol
    );
    debug!("Configuration: {:?}", config);

    // Initialize components with better error handling and dependency injection
    let network = Network::new(config.network.clone(), tracer.clone())
        .context("Failed to initialize network subsystem")?;

    // TODO: Replace with real mempool config if needed
    let mempool = mempool::Mempool::new(1000);

    let mut consensus = Consensus::new(config.consensus.clone(), tracer.clone());
    consensus.set_mempool(mempool.clone());

    // TODO: Protocol::new should be constructed with correct arguments
    let protocol = Protocol {
        hard_fork: crate::protocol::hard_fork_combinator::HardForkCombinator::new(
            crate::protocol::Era,
            std::sync::Arc::new(crate::protocol::babbage::BabbageProtocol::default()),
        ),
    };

    let ledger = Ledger::new(tracer.clone());

    // Create a runtime with optimal thread configuration
    // TODO: Add runtime config if needed
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .enable_all()
        .build()
        .expect("Failed to build Tokio runtime");

    runtime.block_on(async move {
        let app_state = initialize_app_state(
            &config,
            ledger,
            protocol,
            network,
            mempool,
            consensus,
            std::sync::Arc::new(tracer.clone()),
        )
        .await?;

        // Spawn a task to periodically check for protocol upgrades
        let protocol_handle = app_state.protocol.clone();
        task::spawn(async move {
            let mut current_epoch = 0;
            loop {
                protocol_handle
                    .lock()
                    .await
                    .handle_upgrade(current_epoch)
                    .await;
                current_epoch += 1;
                sleep(Duration::from_secs(60)).await; // Check every minute
            }
        });

        let services = start_services(app_state.clone()).await?;
        setup_graceful_shutdown(app_state, services).await?;

        Ok(())
    })?;

    info!("Super Cardano Node shutdown complete");
    Ok(())
}

/// Builds an optimized Tokio runtime with instrumentation
fn build_runtime_with_metrics(config: &RuntimeConfig) -> Result<Runtime> {
    let mut builder = tokio::runtime::Builder::new_multi_thread();

    // Optimize thread pool based on config or system
    let worker_threads = config.worker_threads.unwrap_or_else(|| {
        let cores = num_cpus::get();
        std::cmp::max(2, cores)
    });

    builder
        .worker_threads(worker_threads)
        .thread_name("cardano-worker")
        .enable_all()
        .thread_stack_size(4 * 1024 * 1024) // 4MB stack for complex recursion
        .on_thread_start(|| {
            debug!("Tokio worker thread started");
        })
        .on_thread_stop(|| {
            debug!("Tokio worker thread stopped");
        });

    // Add metrics if enabled
    if config.metrics_enabled {
        builder.enable_time();
    }

    builder.build().context("Failed to build Tokio runtime")
}
