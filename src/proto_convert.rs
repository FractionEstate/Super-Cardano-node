// Conversion utilities between ledger types and proto types for gRPC API
use crate::ledger::{Block as LedgerBlock, BlockHeader as LedgerBlockHeader, Transaction as LedgerTransaction, TxInput as LedgerTxInput, TxOutput as LedgerTxOutput};
use crate::chaindb_proto;

impl From<LedgerBlockHeader> for chaindb_proto::BlockHeader {
    fn from(h: LedgerBlockHeader) -> Self {
        Self {
            slot: h.slot,
            epoch: h.epoch,
            leader: h.leader,
            vrf_proof: h.vrf_proof,
            kes_signature: h.kes_signature,
        }
    }
}

impl From<LedgerTxInput> for chaindb_proto::TxInput {
    fn from(i: LedgerTxInput) -> Self {
        Self {
            prev_tx: i.prev_tx,
            index: i.index,
        }
    }
}

impl From<LedgerTxOutput> for chaindb_proto::TxOutput {
    fn from(o: LedgerTxOutput) -> Self {
        chaindb_proto::TxOutput {
            address: o.address,
            amount: o.amount,
            // If proto supports assets, map them; otherwise, ignore or add a TODO
            // assets: o.assets.map(|assets| assets.into_iter().map(Into::into).collect()),
        }
    }
}

impl From<LedgerTransaction> for chaindb_proto::Transaction {
    fn from(t: LedgerTransaction) -> Self {
        Self {
            id: t.id,
            inputs: t.inputs.into_iter().map(Into::into).collect(),
            outputs: t.outputs.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<LedgerBlock> for chaindb_proto::Block {
    fn from(b: LedgerBlock) -> Self {
        Self {
            id: b.id,
            header: Some(b.header.into()),
            transactions: b.transactions.into_iter().map(Into::into).collect(),
        }
    }
}

pub fn block_to_proto(b: &crate::ledger::Block) -> crate::chaindb_proto::Block {
    crate::chaindb_proto::Block {
        id: b.id,
        header: Some(crate::chaindb_proto::BlockHeader {
            slot: b.header.slot,
            epoch: b.header.epoch,
            leader: b.header.leader.clone(),
            vrf_proof: b.header.vrf_proof.clone(),
            kes_signature: b.header.kes_signature.clone(),
        }),
        transactions: b.transactions.iter().map(transaction_to_proto).collect(),
    }
}

pub fn txoutput_to_proto(o: &crate::ledger::TxOutput) -> crate::chaindb_proto::TxOutput {
    crate::chaindb_proto::TxOutput {
        address: o.address.clone(),
        amount: o.amount,
    }
}

pub fn transaction_to_proto(t: &crate::ledger::Transaction) -> crate::chaindb_proto::Transaction {
    crate::chaindb_proto::Transaction {
        id: t.id,
        inputs: t.inputs.iter().map(|i| crate::chaindb_proto::TxInput {
            prev_tx: i.prev_tx,
            index: i.index,
        }).collect(),
        outputs: t.outputs.iter().map(txoutput_to_proto).collect(),
    }
}
