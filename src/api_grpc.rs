//! gRPC server for ChainDB public API using tonic
//!
//! Implements the ChainDBService as described in the proto definition in api.rs.
//! All methods are async and use strong typing and error handling.

use crate::chaindb::SharedChainDB;
use tonic::{transport::Server, Request, Response, Status};
use std::pin::Pin;
use futures::StreamExt;
use futures_core::stream::Stream;

use Super_Cardano_node::chaindb_proto;
use crate::proto_convert::{block_to_proto, txoutput_to_proto, transaction_to_proto};
// Correct import for generated server trait (prost/tonic lowercases and underscores service names)
use chaindb_proto::chain_db_service_server::{ChainDbService, ChainDbServiceServer};
pub struct ChainDbGrpcServer {
    pub db: SharedChainDB,
}

#[tonic::async_trait]
impl chaindb_proto::chain_db_service_server::ChainDbService for ChainDbGrpcServer {
    type StreamBlocksStream = Pin<Box<dyn Stream<Item = Result<chaindb_proto::Block, Status>> + Send + 'static>>;
    type StreamUTXOsStream = Pin<Box<dyn Stream<Item = Result<chaindb_proto::UtxoEntry, Status>> + Send + 'static>>;

    async fn get_block(&self, request: Request<chaindb_proto::GetBlockRequest>) -> Result<Response<chaindb_proto::Block>, Status> {
        let id = request.into_inner().id;
        let db = self.db.read().await;
        let block = db.api_get_block(id).await.map_err(|e| Status::not_found(e.to_string()))?;
        Ok(Response::new(block_to_proto(&block)))
    }

    async fn get_utxo(&self, request: Request<chaindb_proto::GetUtxoRequest>) -> Result<Response<chaindb_proto::UtxoResponse>, Status> {
        let req = request.into_inner();
        let db = self.db.read().await;
        let utxo = db.api_get_utxo(req.block_id, req.tx_id, req.index).await.map_err(|e| Status::not_found(e.to_string()))?;
        Ok(Response::new(chaindb_proto::UtxoResponse { output: utxo.as_ref().map(|o| txoutput_to_proto(o)) }))
    }

    async fn stream_blocks(&self, _request: Request<chaindb_proto::StreamBlocksRequest>) -> Result<Response<<Self as chaindb_proto::chain_db_service_server::ChainDbService>::StreamBlocksStream>, Status> {
        let blocks: Vec<_> = {
            let db = self.db.read().await;
            let mut stream = db.api_stream_blocks().await.map_err(|e| Status::internal(e.to_string()))?;
            let mut blocks = Vec::new();
            use futures::StreamExt;
            while let Some(block) = stream.next().await {
                blocks.push(block_to_proto(&block));
            }
            blocks
        };
        let s = futures::stream::iter(blocks.into_iter().map(Ok));
        Ok(Response::new(Box::pin(s)))
    }

    async fn stream_utx_os(&self, request: Request<chaindb_proto::StreamUtxOsRequest>) -> Result<Response<<Self as chaindb_proto::chain_db_service_server::ChainDbService>::StreamUTXOsStream>, Status> {
        let utxos: Vec<_> = {
            let block_id = request.into_inner().block_id;
            let db = self.db.read().await;
            let mut stream = db.api_stream_utxos(block_id).await.map_err(|e| Status::internal(e.to_string()))?;
            let mut utxos = Vec::new();
            use futures::StreamExt;
            while let Some(((tx_id, index), output)) = stream.next().await {
                utxos.push(chaindb_proto::UtxoEntry { tx_id, index, output: Some(txoutput_to_proto(&output)) });
            }
            utxos
        };
        let s = futures::stream::iter(utxos.into_iter().map(Ok));
        Ok(Response::new(Box::pin(s)))
    }
}

/// Start the gRPC server on the given address.
pub async fn start_grpc_server(shared_db: crate::chaindb::SharedChainDB, addr: std::net::SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    let svc = ChainDbGrpcServer { db: shared_db };
    Server::builder()
        .add_service(ChainDbServiceServer::new(svc))
        .serve(addr)
        .await?;
    Ok(())
}
