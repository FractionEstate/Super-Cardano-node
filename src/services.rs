use anyhow::{Context, Result};
use futures::future::try_join_all;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::signal;
use tokio::task::JoinHandle;
use tracing::{info, warn, error};

use crate::app_state::SharedAppState;
use crate::api::rest_router;
use crate::api_grpc::start_grpc_server;

pub struct ServiceManager {
    app_state: SharedAppState,
    handles: Vec<JoinHandle<Result<()>>>,
}

impl ServiceManager {
    pub fn new(app_state: SharedAppState) -> Self {
        Self {
            app_state,
            handles: Vec::new(),
        }
    }

    /// Start all network services
    pub async fn start_all(&mut self) -> Result<()> {
        // Start networking service
        self.start_network_service().await?;
        
        // Start consensus service
        self.start_consensus_service().await?;
        
        // Start API services
        self.start_rest_api().await?;
        self.start_grpc_api().await?;
        
        // Start metrics service if enabled
        if self.app_state.config.metrics.enabled {
            self.start_metrics_service().await?;
        }
        
        info!("All services started successfully");
        Ok(())
    }
    
    /// Start the P2P networking service
    async fn start_network_service(&mut self) -> Result<()> {
        let app_state = self.app_state.clone();
        let mut shutdown = app_state.subscribe_shutdown();
        
        let handle = tokio::spawn(async move {
            let network = app_state.network.clone();
            let ledger = app_state.ledger.clone();
            let mempool = app_state.mempool.clone();
            let protocol = app_state.protocol.clone();
            
            info!("Starting networking service on {}", 
                app_state.config.network.bind_addr);
            
            // Start network with proper lifecycle management
            let network_future = network.start_with_state(ledger, mempool, protocol);
            
            tokio::select! {
                result = network_future => {
                    if let Err(e) = result {
                        error!("Network service error: {}", e);
                        return Err(e.into());
                    }
                }
                _ = shutdown.recv() => {
                    info!("Shutting down network service gracefully");
                    network.shutdown().await?;
                }
            }
            
            Ok(())
        });
        
        self.handles.push(handle);
        Ok(())
    }

    /// Start the consensus service
    async fn start_consensus_service(&mut self) -> Result<()> {
        let app_state = self.app_state.clone();
        let mut shutdown = app_state.subscribe_shutdown();
        
        let handle = tokio::spawn(async move {
            let network = app_state.network.clone();
            let protocol = app_state.protocol.clone();
            let ledger = app_state.ledger.clone();
            let consensus = app_state.consensus.clone();
            let chaindb = app_state.chaindb.clone();
            
            info!("Starting consensus service with {} protocol", 
                app_state.config.consensus.protocol);
            
            // Implement proper consensus loop with error handling
            let consensus_future = async {
                loop {
                    let mut chains = {
                        let db = chaindb.read().await;
                        db.get_candidate_chains(3).await? // Get top 3 candidate chains
                    };
                    
                    if chains.is_empty() {
                        chains = vec![crate::ledger::Chain { blocks: vec![] }];
                    }
                    
                    // Critical section with fine-grained locking
                    let res = {
                        let ledger_guard = ledger.read().await;
                        let mut consensus_guard = consensus.write().await;
                        
                        // Get slot leader info with proper error handling
                        let slot_info = consensus_guard.get_slot_leader_info().await?;
                        
                        // Try to produce a block if we're the slot leader
                        if slot_info.is_slot_leader {
                            consensus_guard.produce_block_from_mempool(
                                &*ledger_guard,
                                &*protocol,
                                slot_info.slot,
                                slot_info.epoch,
                                slot_info.leader_id.clone(),
                                slot_info.vrf_proof.clone(),
                                slot_info.kes_signature.clone(),
                            ).await
                        } else {
                            Ok(None)
                        }
                    };
                    
                    // Process the block result outside the critical section
                    match res {
                        Ok(Some(block)) => {
                            info!("Produced block: {} at slot {}", 
                                  block.hash, block.header.slot);
                            
                            // Broadcast the block and update chains
                            network.broadcast_block(&block).await?;
                            
                            // Apply block to candidate chains
                            let ledger_guard = ledger.read().await;
                            let mut new_chains = vec![];
                            for chain in &chains {
                                if let Some(new_chain) = ledger_guard.apply_block_to_chain(chain, block.clone()) {
                                    new_chains.push(new_chain);
                                }
                            }
                            
                            // Select best chain and persist to DB
                            if let Some(best_chain) = crate::ledger::select_chain(&new_chains) {
                                info!("Selected chain with height {}", best_chain.blocks.len());
                                let mut db = chaindb.write().await;
                                db.store_chain(best_chain).await?;
                            }
                        },
                        Ok(None) => {
                            // Not a slot leader or no transactions to include
                        },
                        Err(e) => {
                            warn!("Block production error: {}", e);
                            // Continue consensus loop despite errors
                        }
                    }
                    
                    // Wait for next slot with jitter to avoid network congestion
                    let wait_time = calculate_next_slot_time() + 
                        rand::random::<u64>() % 100; // Add jitter
                    tokio::time::sleep(std::time::Duration::from_millis(wait_time)).await;
                }
            };
            
            tokio::select! {
                result = consensus_future => {
                    if let Err(e) = result {
                        error!("Consensus service error: {}", e);
                        return Err(e.into());
                    }
                }
                _ = shutdown.recv() => {
                    info!("Shutting down consensus service gracefully");
                    // Additional cleanup if needed
                }
            }
            
            Ok(())
        });
        
        self.handles.push(handle);
        Ok(())
    }
    
    // Additional service methods...
    
    /// Wait for all services to complete
    pub async fn wait_for_completion(self) -> Result<()> {
        // Wait for all service tasks to complete
        let results = futures::future::join_all(self.handles).await;
        
        // Check for errors
        for (i, result) in results.into_iter().enumerate() {
            if let Err(e) = result {
                error!("Service {} panicked: {}", i, e);
                return Err(anyhow::anyhow!("Service failure"));
            }
        }
        
        Ok(())
    }
}

// Helper function to calculate next slot time
fn calculate_next_slot_time() -> u64 {
    // Implementation that accounts for network parameters
    20_000 // 20 seconds in milliseconds as a placeholder
}
