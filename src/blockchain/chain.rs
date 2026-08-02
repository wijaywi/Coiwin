use crate::blockchain::block::Block;
use crate::wallet::hybrid_tx::HybridTransaction;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub accounts: HashMap<String, u64>, // Address -> Balance
    pub difficulty: u32,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut chain = Blockchain {
            blocks: Vec::new(),
            accounts: HashMap::new(),
            difficulty: 4, // 4 leading hex zeros
        };
        chain.create_genesis_block();
        chain
    }

    fn create_genesis_block(&mut self) {
        let genesis_block = Block::new(String::from("0"), vec![], self.difficulty, 0);
        self.blocks.push(genesis_block);
    }

    pub fn get_latest_block(&self) -> Option<&Block> {
        self.blocks.last()
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
