/// Shared application state for API handlers (stub)
pub struct AppState;
pub type SharedAppState = std::sync::Arc<AppState>;
// Public API module for Super Cardano Node
//
// Exposes REST and gRPC endpoints for node, chain, and wallet operations.
//
// - REST API: Axum
// - gRPC API: Tonic

pub mod grpc;
pub mod rest;

/// Starts the REST and gRPC servers asynchronously.
pub async fn start_api_services() {
    // TODO: Implement Axum REST and Tonic gRPC servers
}
