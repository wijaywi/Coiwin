use crate::blockchain::block::Block;
use crate::wallet::hybrid_tx::HybridTransaction;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

const BLOCK_GENERATION_INTERVAL: i64 = 10;
const DIFFICULTY_ADJUSTMENT_INTERVAL: usize = 5;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub accounts: HashMap<String, u64>, // Address -> Balance
}

impl Blockchain {
    pub fn new() -> Self {
        let mut chain = Blockchain {
            blocks: Vec::new(),
            accounts: HashMap::new(),
        };
        chain.create_genesis_block();
        chain
    }

    pub fn save_to_disk(&self) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write("coiwin_data.json", json);
        }
    }

    pub fn load_from_disk() -> Option<Self> {
        if let Ok(json) = std::fs::read_to_string("coiwin_data.json") {
            if let Ok(chain) = serde_json::from_str(&json) {
                return Some(chain);
            }
        }
        None
    }

    fn create_genesis_block(&mut self) {
        let genesis_message = HybridTransaction {
            payload: crate::wallet::hybrid_tx::TransactionPayload {
                sender: "COINBASE".to_string(),
                receiver: "WSJ 02/Aug/2026 BRICS Nations Announce Alternative Settlement Network to Bypass Dollar".to_string(),
                amount: 0,
                nonce: 0,
            },
            ecdsa_signature: "00".to_string(),
            dilithium_signature: "00".to_string(),
            dilithium_public: "00".to_string(),
        };
        let genesis_block = Block::new(String::from("0"), vec![genesis_message], 4, 0); // initial diff = 4
        self.blocks.push(genesis_block);
    }

    pub fn get_latest_block(&self) -> Option<&Block> {
        self.blocks.last()
    }

    pub fn get_difficulty(&self) -> u32 {
        let latest_block = self.get_latest_block().unwrap();
        
        if self.blocks.len() > 1 && self.blocks.len() % DIFFICULTY_ADJUSTMENT_INTERVAL == 0 {
            self.get_adjusted_difficulty(latest_block)
        } else {
            latest_block.header.difficulty
        }
    }

    fn get_adjusted_difficulty(&self, latest_block: &Block) -> u32 {
        let prev_adjustment_block = &self.blocks[self.blocks.len() - DIFFICULTY_ADJUSTMENT_INTERVAL];
        let time_expected = BLOCK_GENERATION_INTERVAL * DIFFICULTY_ADJUSTMENT_INTERVAL as i64;
        let time_taken = latest_block.header.timestamp - prev_adjustment_block.header.timestamp;

        if time_taken < time_expected / 2 {
            latest_block.header.difficulty + 1
        } else if time_taken > time_expected * 2 {
            if latest_block.header.difficulty > 1 {
                latest_block.header.difficulty - 1
            } else {
                1
            }
        } else {
            latest_block.header.difficulty
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn update_balances(&mut self, txs: &[HybridTransaction]) {
        for tx in txs {
            // Subtract from sender (ignore coinbase/genesis which might have empty sender)
            if !tx.payload.sender.is_empty() && tx.payload.sender != "COINBASE" {
                let sender_bal = self.accounts.entry(tx.payload.sender.clone()).or_insert(0);
                if *sender_bal >= tx.payload.amount {
                    *sender_bal -= tx.payload.amount;
                }
            }
            
            // Add to receiver
            let receiver_bal = self.accounts.entry(tx.payload.receiver.clone()).or_insert(0);
            *receiver_bal += tx.payload.amount;
        }
    }
    
    pub fn get_balance(&self, address: &str) -> u64 {
        *self.accounts.get(address).unwrap_or(&0)
    }
}
