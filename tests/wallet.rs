//! Integration tests for the wallet module

use crate::wallet::Wallet;
use crate::chaindb::ChainDB;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::test]
async fn test_wallet_create_and_balance() {
    let mut wallet = Wallet::create("test").await;
    assert_eq!(wallet.addresses.len(), 1);
    assert_eq!(wallet.get_balance(), 0);
}

#[tokio::test]
async fn test_wallet_derive_address() {
    let mut wallet = Wallet::create("test").await;
    let addr1 = wallet.derive_address().await;
    let addr2 = wallet.derive_address().await;
    assert_eq!(wallet.addresses.len(), 3);
    assert_ne!(addr1, addr2);
}

#[tokio::test]
async fn test_wallet_select_utxos_and_build_tx() {
    let mut wallet = Wallet::create("test").await;
    // Add fake UTXOs
    wallet.utxos.insert((1, 0), Super_Cardano_node::ledger::TxOutput { address: wallet.addresses[0].clone(), amount: 100 });
    wallet.utxos.insert((2, 0), Super_Cardano_node::ledger::TxOutput { address: wallet.addresses[0].clone(), amount: 50 });
    wallet.balance = 150;
    let tx = wallet.build_transaction("addr1_dest", 120).unwrap();
    assert_eq!(tx.outputs[0].address, "addr1_dest");
    assert_eq!(tx.outputs[0].amount, 120);
}
