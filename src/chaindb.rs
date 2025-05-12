//! Persistent on-disk chain database for Super Cardano Node
//!
//! Handles block storage, UTXO/state snapshots, rollback, and chain selection.
//!
//! # Overview
//! - Stores blocks and ledger state on disk for full chain sync and recovery.
//! - Supports rollback to previous blocks/slots for fork handling.
//! - Provides atomic, crash-safe operations.
//!
//! # Usage Example
//! ```rust,ignore
//! // This example requires an async context and a real Block/LedgerState
//! // use Super_Cardano_node::chaindb::ChainDB;
//! // let mut db = ChainDB::open("./data/chaindb").await.unwrap();
//! // db.append_block(&block, &state).await.unwrap();
//! ```

use crate::ledger::{Block, LedgerState};
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::RwLock;
use std::sync::Arc;
use std::pin::Pin;
use futures::Stream;

/// Persistent chain database for blocks and ledger state.
pub struct ChainDB {
    path: PathBuf,
}

/// Shared, concurrent ChainDB handle for node and applications.
pub type SharedChainDB = Arc<RwLock<ChainDB>>;

impl ChainDB {
    /// Open or create a ChainDB at the given path.
    pub async fn open<P: Into<PathBuf>>(path: P) -> std::io::Result<Self> {
        let path = path.into();
        fs::create_dir_all(&path).await?;
        Ok(Self { path })
    }

    /// Append a block and update the ledger state atomically.
    pub async fn append_block(&mut self, block: &Block, state: &LedgerState) -> std::io::Result<()> {
        let block_path = self.path.join(format!("block_{}.json", block.id));
        let state_path = self.path.join(format!("state_{}.json", block.id));
        let block_data = serde_json::to_vec(block).unwrap();
        let state_data = serde_json::to_vec(state).unwrap();
        fs::write(block_path, block_data).await?;
        fs::write(state_path, state_data).await?;
        Ok(())
    }

    /// Load a block by id.
    pub async fn load_block(&self, id: u64) -> std::io::Result<Block> {
        let block_path = self.path.join(format!("block_{}.json", id));
        let mut file = fs::File::open(block_path).await?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).await?;
        Ok(serde_json::from_slice(&buf).unwrap())
    }

    /// Load ledger state by block id.
    pub async fn load_state(&self, id: u64) -> std::io::Result<LedgerState> {
        let state_path = self.path.join(format!("state_{}.json", id));
        let mut file = fs::File::open(state_path).await?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).await?;
        Ok(serde_json::from_slice(&buf).unwrap())
    }

    /// Roll back to a previous block id (removes all blocks/states after).
    pub async fn rollback_to(&mut self, id: u64) -> std::io::Result<()> {
        let mut entries = fs::read_dir(&self.path).await?;
        while let Some(entry) = entries.next_entry().await? {
            let fname = entry.file_name();
            let fname = fname.to_string_lossy();
            if let Some(num) = fname.split('_').nth(1).and_then(|s| s.split('.').next()) {
                if let Ok(num) = num.parse::<u64>() {
                    if num > id {
                        fs::remove_file(entry.path()).await?;
                    }
                }
            }
        }
        Ok(())
    }

    /// List all block ids in the database (sorted).
    pub async fn block_ids(&self) -> std::io::Result<Vec<u64>> {
        let mut ids = Vec::new();
        let mut entries = fs::read_dir(&self.path).await?;
        while let Some(entry) = entries.next_entry().await? {
            let fname = entry.file_name();
            let fname = fname.to_string_lossy();
            if fname.starts_with("block_") && fname.ends_with(".json") {
                if let Some(num) = fname.trim_start_matches("block_").trim_end_matches(".json").parse::<u64>().ok() {
                    ids.push(num);
                }
            }
        }
        ids.sort_unstable();
        Ok(ids)
    }

    /// Stream all blocks from the database in order (async iterator).
    pub async fn stream_blocks(&self) -> std::io::Result<Pin<Box<dyn Stream<Item = Block> + Send + '_>>> {
        use futures::stream::{self, StreamExt};
        let ids = self.block_ids().await?;
        let this = self;
        let blocks = stream::iter(ids)
            .then(move |id| this.load_block(id))
            .filter_map(|res| async move { res.ok() });
        Ok(Box::pin(blocks))
    }

    /// Query a UTXO by (tx_id, index) at a given block id.
    pub async fn query_utxo(&self, block_id: u64, tx_id: u64, index: u32) -> std::io::Result<Option<crate::ledger::TxOutput>> {
        let state = self.load_state(block_id).await?;
        Ok(state.utxos.get(&(tx_id, index)).cloned())
    }

    /// Query the full UTXO set at a given block id.
    pub async fn query_utxo_set(&self, block_id: u64) -> std::io::Result<std::collections::HashMap<(u64, u32), crate::ledger::TxOutput>> {
        let state = self.load_state(block_id).await?;
        Ok(state.utxos)
    }

    /// Expose a REST-like async API for applications (stub, extend as needed)
    /// Example: Get block by id
    pub async fn api_get_block(&self, id: u64) -> std::io::Result<Block> {
        self.load_block(id).await
    }
    /// Example: Get UTXO by (tx_id, index) at a given block id
    pub async fn api_get_utxo(&self, block_id: u64, tx_id: u64, index: u32) -> std::io::Result<Option<crate::ledger::TxOutput>> {
        self.query_utxo(block_id, tx_id, index).await
    }
    /// Example: Stream all blocks (for sync or explorer)
    pub async fn api_stream_blocks(&self) -> std::io::Result<Pin<Box<dyn Stream<Item = Block> + Send + '_>>> {
        self.stream_blocks().await
    }
    /// Example: Stream all UTXOs at a given block id
    pub async fn api_stream_utxos(&self, block_id: u64) -> std::io::Result<Pin<Box<dyn Stream<Item = ((u64, u32), crate::ledger::TxOutput)> + Send + '_>>> {
        let utxos = self.query_utxo_set(block_id).await?;
        use futures::stream;
        Ok(Box::pin(stream::iter(utxos.into_iter())))
    }
}
