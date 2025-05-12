mod address;
mod keys;
mod transaction;
mod utxo;

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};

use crate::chaindb::SharedChainDB;
use self::address::Address;
use self::keys::{KeyPair, HDWallet, DerivationPath};
use self::transaction::{Transaction, TransactionBuilder};
use self::utxo::UtxoSet;

/// A Cardano wallet with HD key management and transaction capabilities
#[derive(Debug)]
pub struct Wallet {
    name: String,
    hd_wallet: HDWallet,
    utxo_set: UtxoSet,
    metadata: WalletMetadata,
    state: WalletState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletMetadata {
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_used: chrono::DateTime<chrono::Utc>,
    pub address_discovery_gap_limit: u32,
}

#[derive(Debug)]
pub struct WalletState {
    pub confirmed_balance: u64,
    pub pending_balance: u64,
    pub reward_balance: u64,
    pub address_index: u32,
    pub last_sync_height: u64,
    pub last_sync_hash: Vec<u8>,
}

/// Manages multiple wallets
#[derive(Debug)]
pub struct WalletManager {
    wallets: Vec<SharedWallet>,
    db_path: PathBuf,
    chaindb: SharedChainDB,
}

pub type SharedWallet = Arc<RwLock<Wallet>>;

impl Wallet {
    /// Creates a new wallet with the given name and mnemonic
    pub async fn new(name: &str, mnemonic: Option<&str>, password: Option<&str>) -> Result<Self> {
        let hd_wallet = if let Some(mnemonic_phrase) = mnemonic {
            HDWallet::from_mnemonic(mnemonic_phrase, password)?
        } else {
            HDWallet::generate_new(password)?
        };
        
        let now = chrono::Utc::now();
        let metadata = WalletMetadata {
            name: name.to_string(),
            created_at: now,
            last_used: now,
            address_discovery_gap_limit: 20,
        };
        
        let state = WalletState {
            confirmed_balance: 0,
            pending_balance: 0,
            reward_balance: 0,
            address_index: 0,
            last_sync_height: 0,
            last_sync_hash: vec![],
        };
        
        Ok(Self {
            name: name.to_string(),
            hd_wallet,
            utxo_set: UtxoSet::new(),
            metadata,
            state,
        })
    }
    
    /// Returns the wallet's name
    pub fn name(&self) -> &str {
        &self.name
    }
    
    /// Derives a new address at the given path
    pub fn derive_address(
        &mut self, 
        account: u32, 
        is_change: bool, 
        index: Option<u32>
    ) -> Result<Address> {
        let address_index = match index {
            Some(idx) => idx,
            None => {
                let idx = self.state.address_index;
                self.state.address_index += 1;
                idx
            }
        };
        
        let change_bit = if is_change { 1 } else { 0 };
        let path = DerivationPath::new(vec![
            crate::wallet::keys::CARDANO_PURPOSE,
            crate::wallet::keys::CARDANO_COIN_TYPE,
            account,
            change_bit,
            address_index,
        ]);
        
        let key_pair = self.hd_wallet.derive_key_pair(&path)?;
        Address::from_key_pair(&key_pair)
    }
    
    /// Creates a transaction that sends funds to the given addresses
    pub fn create_transaction(
        &self,
        outputs: Vec<(Address, u64)>,
        fee_algo: impl Fn(usize, usize) -> u64,
    ) -> Result<Transaction> {
        let mut builder = TransactionBuilder::new();
        
        // Add outputs
        for (address, amount) in outputs {
            builder.add_output(address, amount);
        }
        
        // Select inputs (UTXOs) to cover the payment and fee
        let total_output = builder.total_output();
        let selected_utxos = self.utxo_set.select_utxos(total_output)?;
        
        for utxo in selected_utxos {
            builder.add_input(utxo);
        }
        
        // Calculate and set fee
        let tx_size_bytes = builder.estimate_size();
        let fee = fee_algo(tx_size_bytes, outputs.len());
        builder.set_fee(fee);
        
        // Add change output if needed
        if builder.total_input() > builder.total_output() + fee {
            let change_amount = builder.total_input() - builder.total_output() - fee;
            let change_address = self.derive_address(0, true, None)?;
            builder.add_change_output(change_address, change_amount);
        }
        
        // Sign and return the transaction
        self.sign_transaction(builder)
    }
    
    /// Signs a transaction with this wallet's keys
    fn sign_transaction(&self, mut builder: TransactionBuilder) -> Result<Transaction> {
        // For each input, find the appropriate key and sign
        for (input_idx, input) in builder.get_inputs().iter().enumerate() {
            if let Some(address) = self.utxo_set.get_address(input) {
                // Find the key used to create this address
                if let Some(key_pair) = self.find_key_for_address(&address)? {
                    builder.sign_input(input_idx, &key_pair)?;
                }
            }
        }
        
        builder.build().context("Failed to build transaction")
    }
    
    /// Finds the key that was used to create an address
    fn find_key_for_address(&self, address: &Address) -> Result<Option<KeyPair>> {
        // Implementation would scan through derived keys to find match
        // This is a simplification - a real implementation would use a cache
        Ok(None) // Placeholder
    }
    
    /// Synchronizes the wallet with the blockchain
    pub async fn sync(&mut self, chaindb: &crate::chaindb::ChainDB) -> Result<()> {
        // Implementation would scan blocks since last sync
        // Update UTXOs, balances, etc.
        Ok(())
    }
}

impl WalletManager {
    /// Creates a new wallet manager
    pub async fn new<P: AsRef<Path>>(db_path: P, chaindb: SharedChainDB) -> Result<Self> {
        let path = db_path.as_ref().to_path_buf();
        
        // Ensure wallet directory exists
        tokio::fs::create_dir_all(&path).await
            .context("Failed to create wallet directory")?;
        
        let mut manager = Self {
            wallets: Vec::new(),
            db_path: path,
            chaindb,
        };
        
        // Load existing wallets
        manager.load_wallets().await?;
        
        Ok(manager)
    }
    
    /// Loads all wallets from storage
    async fn load_wallets(&mut self) -> Result<()> {
        // Implementation would scan wallet directory and load each wallet
        Ok(())
    }
    
    /// Creates a new wallet
    pub async fn create_wallet(
        &mut self,
        name: &str,
        mnemonic: Option<&str>,
        password: Option<&str>,
    ) -> Result<SharedWallet> {
        // Check if wallet with this name already exists
        if self.wallets.iter().any(|w| async {
            let w = w.read().await;
            w.name() == name
        }.await) {
            return Err(anyhow::anyhow!("Wallet with name '{}' already exists", name));
        }
        
        // Create new wallet
        let wallet = Wallet::new(name, mnemonic, password).await?;
        let wallet_arc = Arc::new(RwLock::new(wallet));
        
        // Save wallet to storage
        self.persist_wallet(&wallet_arc).await?;
        
        // Add to list of managed wallets
        self.wallets.push(wallet_arc.clone());
        
        Ok(wallet_arc)
    }
    
    /// Persists a wallet to storage
    async fn persist_wallet(&self, wallet: &SharedWallet) -> Result<()> {
        // Implementation would serialize and save wallet data
        Ok(())
    }
    
    /// Returns all managed wallets
    pub fn get_wallets(&self) -> &[SharedWallet] {
        &self.wallets
    }
    
    /// Gets a wallet by name
    pub fn get_wallet_by_name(&self, name: &str) -> Option<SharedWallet> {
        self.wallets.iter().find(|w| async {
            let w = w.read().await;
            w.name() == name
        }.now_or_never().flatten().unwrap_or(false))
        .cloned()
    }
    
    /// Syncs all wallets with the blockchain
    pub async fn sync_all_wallets(&self) -> Result<()> {
        let chaindb = self.chaindb.read().await;
        
        for wallet in &self.wallets {
            let mut wallet = wallet.write().await;
            if let Err(e) = wallet.sync(&chaindb).await {
                tracing::warn!("Failed to sync wallet {}: {}", wallet.name(), e);
                // Continue with other wallets even if one fails
            }
        }
        
        Ok(())
    }
}
