//! Application-facing integration test for ChainDB block/UTXO queries

use Super_Cardano_node::chaindb::ChainDB;
use tokio::runtime::Runtime;

#[test]
fn test_chaindb_application_queries() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let db = ChainDB::open("./testdata/chaindb").await.unwrap();
        // Query block stream (should be empty or contain blocks from previous test)
        let mut stream = Box::pin(db.stream_blocks().await.unwrap());
        let mut count = 0;
        use futures::StreamExt;
        while let Some(block) = stream.next().await {
            count += 1;
            assert!(block.id > 0);
        }
        // Query UTXO set (should not panic)
        let _ = db.query_utxo_set(1).await;
    });
}
