use anyhow::Result;
use futures::future::FutureExt;
use std::time::Duration;
use tokio::signal;
use tokio::time::timeout;
use tracing::{error, info, warn};

use crate::app_state::SharedAppState;
use crate::configuration::types::Configuration as Config;
use crate::services::ServiceManager;

/// Initializes application state with all components.
pub async fn initialize_app_state(
    config: &Config,
    ledger: crate::ledger::Ledger,
    protocol: crate::protocol::Protocol,
    network: crate::networking::Network,
    mempool: crate::mempool::Mempool,
    consensus: crate::consensus::Consensus,
    tracer: std::sync::Arc<crate::tracing::tracers::Tracer>,
) -> Result<SharedAppState> {
    info!("Initializing application state...");

    let app_state = crate::app_state::AppState::new(
        config.clone(),
        ledger,
        protocol,
        network,
        mempool,
        consensus,
        tracer,
    )
    .await?;

    info!("Application state initialized successfully");

    Ok(std::sync::Arc::new(app_state))
}

/// Starts all services with the given app state
pub async fn start_services(app_state: SharedAppState) -> Result<ServiceManager> {
    info!("Starting services...");

    let mut service_manager = ServiceManager::new(app_state);
    service_manager.start_all().await?;

    info!("All services started successfully");

    Ok(service_manager)
}

/// Sets up signal handlers for graceful shutdown
pub async fn setup_graceful_shutdown(
    app_state: SharedAppState,
    services: ServiceManager,
) -> Result<()> {
    // Wait for termination signal
    let shutdown_future = async {
        let ctrl_c = signal::ctrl_c().fuse();
        let term = async {
            #[cfg(unix)]
            {
                let mut term = signal::unix::signal(signal::unix::SignalKind::terminate())?;
                term.recv().await;
                Ok::<_, anyhow::Error>(())
            }
            #[cfg(not(unix))]
            {
                // Just wait forever on non-unix platforms
                std::future::pending::<()>().await;
                unreachable!()
            }
        }
        .fuse();

        futures::pin_mut!(ctrl_c);
        futures::pin_mut!(term);

        futures::select! {
            _ = ctrl_c => info!("Received Ctrl-C signal"),
            result = term => {
                if let Err(e) = result {
                    error!("Error setting up termination signal handler: {}", e);
                } else {
                    info!("Received termination signal");
                }
            }
        }

        Ok::<_, anyhow::Error>(())
    };

    // Wait for shutdown signal
    shutdown_future.await?;

    // Begin graceful shutdown
    info!("Starting graceful shutdown...");
    app_state.initiate_shutdown();

    // Give services time to shut down gracefully
    match timeout(Duration::from_secs(10), services.wait_for_completion()).await {
        Ok(result) => {
            if let Err(e) = result {
                error!("Error during service shutdown: {}", e);
            }
        }
        Err(_) => {
            warn!("Services shutdown timed out after 10 seconds");
        }
    }

    // Final cleanup
    info!("Persisting final state...");
    let db = app_state.chaindb.read().await;
    if let Err(e) = db.flush().await {
        error!("Error flushing database: {}", e);
    }
    drop(db); // Ensure the lock is released

    info!("Graceful shutdown completed");

    Ok(())
}
