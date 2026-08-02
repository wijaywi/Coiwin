use coiwin::blockchain::block::Block;
use coiwin::blockchain::chain::Blockchain;
use coiwin::consensus::pow::ProofOfWork;
use coiwin::consensus::validator::Validator;
use coiwin::wallet::hybrid_tx::{HybridTransaction, HybridWallet, TransactionPayload};
use std::io::{self, Write};
use anyhow::Result;

fn main() -> Result<()> {
    println!("=== Coiwin Blockchain Node ===");
    println!("Initializing Blockchain...");
    let mut chain = if let Some(loaded) = Blockchain::load_from_disk() {
        println!("Loaded blockchain from disk ({} blocks).", loaded.blocks.len());
        loaded
    } else {
        println!("No local blockchain found. Creating Genesis block...");
        Blockchain::new()
    };
    
    // We will generate a wallet to act as the Miner's wallet (where rewards go)
    let miner_wallet = HybridWallet::generate()?;
    let miner_address = miner_wallet.ecdsa_public.clone();
    println!("Miner Address: {}", miner_address);

    loop {
        print!("> ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let command = input.trim();

        match command {
            "mine" => {
                println!("Mining a new block...");
                // Create a coinbase transaction for the block reward
                let coinbase_tx = HybridTransaction {
                    payload: TransactionPayload {
                        sender: "COINBASE".to_string(),
                        receiver: miner_address.clone(),
                        amount: 50, // 50 Coiwin reward
                        nonce: 0,
                    },
                    ecdsa_signature: "00".to_string(),
                    dilithium_signature: "00".to_string(),
                    dilithium_public: "00".to_string(),
                };

                let prev_hash = chain.get_latest_block().unwrap().hash.clone();
                let current_difficulty = chain.get_difficulty();
                println!("Current network difficulty: {}", current_difficulty);
                let mut new_block = Block::new(prev_hash, vec![coinbase_tx], current_difficulty, 0);
                
                // PoW
                ProofOfWork::mine(&mut new_block);
                println!("Block mined! Hash: {}", new_block.hash);

                // Add to chain
                chain.update_balances(&new_block.transactions);
                chain.add_block(new_block);
                chain.save_to_disk();

                let latest = chain.get_latest_block().unwrap();
                print_ascii_block(chain.blocks.len() - 1, &latest.hash, &latest.header.prev_hash);
            }
            "balance" => {
                let bal = chain.get_balance(&miner_address);
                println!("Miner Balance: {} Coiwin", bal);
            }
            "status" => {
                let height = chain.blocks.len() - 1;
                let latest = chain.get_latest_block().unwrap();
                println!("Blockchain height: {}", height + 1);
                print_ascii_block(height, &latest.hash, &latest.header.prev_hash);
                println!("Latest block hash: {}", chain.get_latest_block().unwrap().hash);
                println!("Next block difficulty: {}", chain.get_difficulty());
            }
            "exit" => {
                break;
            }
            "" => continue,
            _ => {
                println!("Unknown command. Available commands: mine, balance, status, exit");
            }
        }
    }

    Ok(())
}

fn print_ascii_block(height: usize, hash: &str, prev_hash: &str) {
    let hash_short = if hash.len() > 16 { format!("{}...", &hash[..16]) } else { hash.to_string() };
    let prev_short = if prev_hash.len() > 16 { format!("{}...", &prev_hash[..16]) } else { prev_hash.to_string() };
    
    println!(r#"
  .=================================.
  |                                 |
  |         BLOCK #{:<14} |
  |                                 |
  |---------------------------------|
  | Hash: {:<25} |
  | Prev: {:<25} |
  '================================='
    "#, height, hash_short, prev_short);
}
