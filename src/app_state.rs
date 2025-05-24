//! Central application state for Super Cardano Node
//!
//! Provides a type-safe container for all major subsystems and shared resources.

use anyhow::Result;
use std::sync::Arc;
use tokio::sync::{RwLock, Mutex};

use crate::configuration::Config;
use crate::consensus::Consensus;
use crate::networking::Network;
use crate::protocol::Protocol;
use crate::ledger::Ledger;
use crate::mempool::Mempool;
use crate::chaindb::{ChainDB, SharedChainDB};
use crate::tracing::Tracer;
use crate::wallet::{WalletManager, SharedWallet};

/// Central application state with proper type-safe access.
///
/// Holds references to all major node subsystems and shared resources, allowing for coordinated access and shutdown.
///
/// # Example
/// ```
/// // Example usage (pseudo-code):
/// let app_state = AppState::new(...).await.unwrap();
/// app_state.initiate_shutdown();
/// ```
#[derive(Clone)]
pub struct AppState {
    /// Node configuration.
    pub config: Arc<Config>,
    /// Shared ledger state.
    pub ledger: Arc<RwLock<Ledger>>,
    /// Protocol handler.
    pub protocol: Arc<Protocol>,
    /// Networking subsystem.
    pub network: Arc<Network>,
    /// Shared mempool.
    pub mempool: Arc<RwLock<Mempool>>,
    /// Consensus subsystem.
    pub consensus: Arc<RwLock<Consensus>>,
    /// Shared chain database.
    pub chaindb: SharedChainDB,
    /// Tracing and logging.
    pub tracer: Arc<Tracer>,
    /// Wallet manager.
    pub wallets: Arc<RwLock<WalletManager>>,

    // Service shutdown coordination
    pub shutdown: Arc<tokio::sync::broadcast::Sender<()>>,
}

impl AppState {
    /// Creates a new AppState with all components properly initialized.
    ///
    /// # Arguments
    /// * `config` - Node configuration.
    /// * `ledger` - Ledger state.
    /// * `protocol` - Protocol handler.
    /// * `network` - Networking subsystem.
    /// * `mempool` - Mempool instance.
    /// * `consensus` - Consensus subsystem.
    /// * `tracer` - Tracing and logging.
    ///
    /// # Errors
    /// Returns an error if any subsystem fails to initialize.
    pub async fn new(
        config: Config,
        ledger: Ledger,
        protocol: Protocol,
        network: Network,
        mempool: Mempool,
        consensus: Consensus,
        tracer: Arc<Tracer>,
    ) -> Result<Self> {
        let chaindb_path = config.chaindb.path.clone();
        let chaindb = Arc::new(RwLock::new(
            ChainDB::open(&chaindb_path).await?
        ));

        let wallet_manager = WalletManager::new(
            config.wallet.path.clone(),
            chaindb.clone()
        ).await?;

        let (shutdown_tx, _) = tokio::sync::broadcast::channel(1);

        Ok(Self {
            config: Arc::new(config),
            ledger: Arc::new(RwLock::new(ledger)),
            protocol: Arc::new(protocol),
            network: Arc::new(network),
            mempool: Arc::new(RwLock::new(mempool)),
            consensus: Arc::new(RwLock::new(consensus)),
            chaindb,
            tracer,
            wallets: Arc::new(RwLock::new(wallet_manager)),
            shutdown: Arc::new(shutdown_tx),
        })
    }

    /// Gets a receiver that will be notified on shutdown.
    pub fn subscribe_shutdown(&self) -> tokio::sync::broadcast::Receiver<()> {
        self.shutdown.subscribe()
    }

    /// Initiates application shutdown.
    pub fn initiate_shutdown(&self) {
        let _ = self.shutdown.send(());
    }
}

/// Shared application state type alias.
pub type SharedAppState = Arc<AppState>;
