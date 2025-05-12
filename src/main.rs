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
use anyhow::{Context, Result};
use tracing::{info, warn, error, debug};

fn main() -> Result<()> {
    // Initialize tracing early for better diagnostics during startup
    let tracing = Tracing::new();
    let tracer = tracing.tracer.clone();
    
    // Parse CLI arguments with improved error context
    let cli = Cli::parse();
    if cli.version {
        println!("Super Cardano Node v0.1.0");
        return Ok(());
    }
    
    // Load configuration with better error handling
    let config = match cli.config.as_ref() {
        Some(path) => Config::load_from_file(path)
            .with_context(|| format!("Failed to load config from '{}'", path.display()))?,
        None => {
            info!("No config specified, using default configuration");
            Config::default()
        }
    };
    
    info!("Starting Super Cardano Node with {} consensus protocol", config.consensus.protocol);
    debug!("Configuration: {:?}", config);
    
    // Initialize components with better error handling and dependency injection
    let network = Network::new(config.network.clone(), tracer.clone())
        .context("Failed to initialize network subsystem")?;
        
    let mempool = mempool::Mempool::new(config.mempool.capacity)
        .context("Failed to initialize mempool")?;
        
    let mut consensus = Consensus::new(config.consensus.clone(), tracer.clone())
        .context("Failed to initialize consensus subsystem")?;
    consensus.set_mempool(mempool.clone());
    
    let protocol = Protocol::new(config.protocol.clone())
        .context("Failed to initialize protocol subsystem")?;
    
    let ledger = Ledger::new(tracer.clone())
        .context("Failed to initialize ledger")?;

    // Create a runtime with optimal thread configuration
    let runtime = build_runtime_with_metrics(&config.runtime)?;
    
    runtime.block_on(async move {
        // Initialize shared application state with better structure
        let app_state = initialize_app_state(
            &config, ledger, protocol, network, mempool, tracer.clone()
        ).await?;
        
        // Start services with dependency injection and lifecycle management
        let services = start_services(app_state.clone()).await?;
        
        // Set up graceful shutdown with proper cleanup
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
        builder.enable_time_travel();
    }
    
    builder.build().context("Failed to build Tokio runtime")
}
