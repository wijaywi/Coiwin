use crate::wallet::hybrid_tx::HybridTransaction;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockHeader {
    pub prev_hash: String,
    pub timestamp: i64,
    pub nonce: u64,
    pub merkle_root: String,
    pub difficulty: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<HybridTransaction>,
    pub hash: String,
}

impl Block {
    pub fn new(
        prev_hash: String,
        transactions: Vec<HybridTransaction>,
        difficulty: u32,
        nonce: u64,
    ) -> Self {
        let mut block = Block {
            header: BlockHeader {
                prev_hash,
                timestamp: Utc::now().timestamp(),
                nonce,
                merkle_root: Self::calculate_merkle_root(&transactions),
                difficulty,
            },
            transactions,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_merkle_root(_txs: &[HybridTransaction]) -> String {
        // Simplified for PoC: Just hash the JSON serialization of all transactions
        let txs_bytes = serde_json::to_vec(_txs).unwrap_or_default();
        let mut hasher = Sha256::new();
        hasher.update(txs_bytes);
        hex::encode(hasher.finalize())
    }

    pub fn calculate_hash(&self) -> String {
        let header_bytes = serde_json::to_vec(&self.header).unwrap_or_default();
        let mut hasher = Sha256::new();
        hasher.update(header_bytes);
        hex::encode(hasher.finalize())
    }
}
