//! REST and gRPC API server for Super Cardano Node
//!
//! Exposes ChainDB public API for external applications via REST (axum) and gRPC (tonic).
//!
//! - REST endpoints: /block/{id}, /utxo/{block_id}/{tx_id}/{index}, /blocks, /utxos/{block_id}
//! - gRPC service: ChainDBService (see .proto definition below)
//!
//! All endpoints are async and use strong typing and error handling.

use crate::chaindb::SharedChainDB;
use crate::wallet;
use crate::ledger::Transaction;
use axum::{extract::Path, response::IntoResponse, routing::get, Json, Router, extract::State};
use axum::routing::post;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Application state for API handlers.
///
/// Holds references to the shared chain database and wallet list.
#[derive(Debug, Clone)]
pub struct AppState {
    /// Shared chain database.
    pub db: SharedChainDB,
    /// List of shared wallets.
    pub wallets: Arc<RwLock<Vec<wallet::SharedWallet>>>,
}

/// Shared application state type alias for API handlers.
pub type SharedAppState = Arc<RwLock<AppState>>;

/// Build the REST API router for ChainDB.
///
/// # Arguments
/// * `app_state` - Shared application state for handlers.
///
/// # Returns
/// An Axum `Router` with all REST endpoints registered.
pub fn rest_router(app_state: SharedAppState) -> Router {
    Router::new()
        .route("/block/:id", get(get_block))
        .route("/utxo/:block_id/:tx_id/:index", get(get_utxo))
        .route("/blocks", get(stream_blocks))
        .route("/utxos/:block_id", get(stream_utxos))
        // Wallet endpoints
        .route("/wallet/create", post(create_wallet))
        .route("/wallet/:idx/balance", get(get_wallet_balance))
        .route("/wallet/:idx/derive", post(derive_wallet_address))
        .route("/wallet/:idx/build_tx", post(build_wallet_transaction))
        .with_state(app_state)
}

async fn get_block(
    Path(id): Path<u64>,
    State(app_state): State<SharedAppState>,
) -> impl IntoResponse {
    let state = app_state.read().await;
    let db = state.db.read().await;
    match db.api_get_block(id).await {
        Ok(block) => Json(block).into_response(),
        Err(e) => (axum::http::StatusCode::NOT_FOUND, e.to_string()).into_response(),
    }
}

async fn get_utxo(
    Path((block_id, tx_id, index)): Path<(u64, u64, u32)>,
    State(app_state): State<SharedAppState>,
) -> impl IntoResponse {
    let state = app_state.read().await;
    let db = state.db.read().await;
    match db.api_get_utxo(block_id, tx_id, index).await {
        Ok(Some(utxo)) => Json(utxo).into_response(),
        Ok(None) => (axum::http::StatusCode::NOT_FOUND, "UTXO not found").into_response(),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn stream_blocks(
    State(app_state): State<SharedAppState>,
) -> impl IntoResponse {
    let state = app_state.read().await;
    let db = state.db.read().await;
    let mut stream = match db.api_stream_blocks().await {
        Ok(s) => s,
        Err(e) => return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };
    use futures::StreamExt;
    let mut blocks = Vec::new();
    while let Some(block) = stream.next().await {
        blocks.push(block);
    }
    Json(blocks).into_response()
}

async fn stream_utxos(
    Path(block_id): Path<u64>,
    State(app_state): State<SharedAppState>,
) -> impl IntoResponse {
    let state = app_state.read().await;
    let db = state.db.read().await;
    let mut stream = match db.api_stream_utxos(block_id).await {
        Ok(s) => s,
        Err(e) => return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };
    use futures::StreamExt;
    let mut utxos = Vec::new();
    while let Some(utxo) = stream.next().await {
        utxos.push(utxo);
    }
    Json(utxos).into_response()
}

/// Create a new wallet
///
/// # Arguments
/// * `app_state` - Shared application state for handlers.
/// * `req` - JSON payload with the wallet name.
///
/// # Returns
/// A JSON response with the created wallet data.
pub async fn create_wallet(State(app_state): State<SharedAppState>, Json(req): Json<CreateWalletRequest>) -> Json<wallet::Wallet> {
    let mut state = app_state.write().await;
    let wallet = wallet::Wallet::create(&req.name).await;
    let shared = Arc::new(RwLock::new(wallet.clone()));
    state.wallets.write().await.push(shared);
    Json(wallet)
}

/// Get wallet balance
///
/// # Arguments
/// * `app_state` - Shared application state for handlers.
/// * `idx` - Wallet index in the shared wallet list.
///
/// # Returns
/// A JSON response with the wallet balance.
pub async fn get_wallet_balance(State(app_state): State<SharedAppState>, Path(idx): Path<usize>) -> Json<u64> {
    let state = app_state.read().await;
    let wallets = state.wallets.read().await;
    let wallet = wallets.get(idx).unwrap();
    let wallet = wallet.read().await;
    Json(wallet.get_balance())
}

/// Derive a new address
///
/// # Arguments
/// * `app_state` - Shared application state for handlers.
/// * `idx` - Wallet index in the shared wallet list.
///
/// # Returns
/// A JSON response with the derived address.
pub async fn derive_wallet_address(State(app_state): State<SharedAppState>, Path(idx): Path<usize>) -> Json<String> {
    let state = app_state.read().await;
    let wallets = state.wallets.read().await;
    let wallet = wallets.get(idx).unwrap();
    let mut wallet = wallet.write().await;
    let addr = wallet.derive_address().await;
    Json(addr)
}

/// Build a transaction
///
/// # Arguments
/// * `app_state` - Shared application state for handlers.
/// * `idx` - Wallet index in the shared wallet list.
/// * `req` - JSON payload with transaction details (to_address, amount).
///
/// # Returns
/// A JSON response with the built transaction data, or null if insufficient funds.
pub async fn build_wallet_transaction(State(app_state): State<SharedAppState>, Path(idx): Path<usize>, Json(req): Json<BuildTxRequest>) -> Json<Option<Transaction>> {
    let state = app_state.read().await;
    let wallets = state.wallets.read().await;
    let wallet = wallets.get(idx).unwrap();
    let wallet = wallet.read().await;
    Json(wallet.build_transaction(&req.to_address, req.amount))
}

#[derive(Deserialize)]
pub struct CreateWalletRequest {
    /// Name of the wallet to be created.
    pub name: String,
}

#[derive(Deserialize)]
pub struct BuildTxRequest {
    /// Recipient's address for the transaction.
    pub to_address: String,
    /// Amount of lovelace to send.
    pub amount: u64,
}

// --- gRPC API (tonic) ---

// Protobuf definition (for reference):
//
// syntax = "proto3";
// package chaindb;
//
// service ChainDBService {
//   rpc GetBlock (GetBlockRequest) returns (Block);
//   rpc GetUTXO (GetUTXORequest) returns (UTXOResponse);
//   rpc StreamBlocks (StreamBlocksRequest) returns (stream Block);
//   rpc StreamUTXOs (StreamUTXOsRequest) returns (stream UTXOEntry);
// }
//
// message GetBlockRequest { uint64 id = 1; }
// message GetUTXORequest { uint64 block_id = 1; uint64 tx_id = 2; uint32 index = 3; }
// message StreamBlocksRequest {}
// message StreamUTXOsRequest { uint64 block_id = 1; }
// message Block { ... }
// message UTXOEntry { uint64 tx_id = 1; uint32 index = 2; TxOutput output = 3; }
// message TxOutput { ... }
// message UTXOResponse { optional TxOutput output = 1; }

// See src/api_grpc.rs for full tonic implementation (to be created)
