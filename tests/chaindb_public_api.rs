//! Example: Application integration with ChainDB public API

use crate::chaindb::{ChainDB, SharedChainDB};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::runtime::Runtime;

#[test]
fn test_application_api_usage() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let _ = std::fs::remove_dir_all("./testdata/chaindb");
        let mut ledger = crate::ledger::Ledger::new(Default::default());
        let mut db = ChainDB::open("./testdata/chaindb").await.unwrap();
        let _tx = Super_Cardano_node::ledger::Transaction::default();
        let block = Super_Cardano_node::ledger::Block {
            id: 1,
            ..Default::default()
        };
        ledger.apply_block_with_db(&block, Some(&mut db)).await;
        let shared: SharedChainDB = Arc::new(RwLock::new(db));
        // Simulate application query
        let _block = shared.read().await.api_get_block(1).await;
        let _utxo = shared.read().await.api_get_utxo(1, 0, 0).await;
        let shared_read = shared.read().await;
        let mut block_stream = Box::pin(shared_read.api_stream_blocks().await.unwrap());
        use futures::StreamExt;
        while let Some(_block) = block_stream.next().await {}
        let mut utxo_stream = Box::pin(shared_read.api_stream_utxos(1).await.unwrap());
        while let Some(_utxo) = utxo_stream.next().await {}
    });
}
