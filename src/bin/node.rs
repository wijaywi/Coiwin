use coiwin::blockchain::block::Block;
use coiwin::blockchain::chain::Blockchain;
use coiwin::consensus::pow::ProofOfWork;
use coiwin::network::p2p::P2PNode;
use coiwin::wallet::hybrid_tx::{HybridTransaction, HybridWallet, TransactionPayload};
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use anyhow::Result;

fn main() -> Result<()> {
    println!("=== Coiwin Blockchain Node ===");
    
    // We will load the Miner's wallet from disk or generate a new one if it doesn't exist
    let miner_wallet = if let Ok(json) = std::fs::read_to_string("miner_wallet.json") {
        println!("Loaded existing miner wallet.");
        serde_json::from_str(&json).unwrap_or_else(|_| HybridWallet::generate().unwrap())
    } else {
        println!("Creating new miner wallet...");
        let w = HybridWallet::generate()?;
        if let Ok(json) = serde_json::to_string_pretty(&w) {
            let _ = std::fs::write("miner_wallet.json", json);
        }
        w
    };
    let miner_address = miner_wallet.ecdsa_public.clone();
    println!("Miner Address: {}", miner_address);

    let chain = if let Some(loaded) = Blockchain::load_from_disk() {
        println!("Loaded blockchain from disk ({} blocks).", loaded.blocks.len());
        loaded
    } else {
        println!("No local blockchain found. Creating Genesis block...");
        Blockchain::new()
    };
    
    let shared_chain = Arc::new(Mutex::new(chain));
    let p2p = P2PNode::new(Arc::clone(&shared_chain));
    
    // Start server in background
    p2p.start_server("8000"); // default port

    loop {
        print!("> ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let command = input.trim();

        if command.starts_with("connect ") {
            let parts: Vec<&str> = command.split_whitespace().collect();
            if parts.len() == 2 {
                p2p.connect_to_peer(parts[1]);
            } else {
                println!("Usage: connect <ip:port>");
            }
            continue;
        }

        match command {
            "mine" => {
                println!("Mining a new block...");
                let coinbase_tx = HybridTransaction {
                    payload: TransactionPayload {
                        sender: "COINBASE".to_string(),
                        receiver: miner_address.clone(),
                        amount: 50,
                        nonce: 0,
                    },
                    ecdsa_signature: "00".to_string(),
                    dilithium_signature: "00".to_string(),
                    dilithium_public: "00".to_string(),
                };

                let (prev_hash, current_difficulty) = {
                    let c = shared_chain.lock().unwrap();
                    let latest = c.get_latest_block().unwrap();
                    (latest.hash.clone(), c.get_difficulty())
                };

                println!("Current network difficulty: {}", current_difficulty);
                let mut new_block = Block::new(prev_hash, vec![coinbase_tx], current_difficulty, 0);
                
                // PoW
                ProofOfWork::mine(&mut new_block);
                println!("Block mined! Hash: {}", new_block.hash);

                // Add to chain & broadcast
                let (height, hash, prev) = {
                    let mut c = shared_chain.lock().unwrap();
                    c.update_balances(&new_block.transactions);
                    c.add_block(new_block.clone());
                    c.save_to_disk();
                    
                    let latest = c.get_latest_block().unwrap();
                    (c.blocks.len() - 1, latest.hash.clone(), latest.header.prev_hash.clone())
                };
                
                p2p.broadcast_new_block(&new_block);
                print_ascii_block(height, &hash, &prev);
            }
            "balance" => {
                let c = shared_chain.lock().unwrap();
                let bal = c.get_balance(&miner_address);
                println!("Miner Balance: {} Coiwin", bal);
            }
            "status" => {
                let c = shared_chain.lock().unwrap();
                let height = c.blocks.len() - 1;
                let latest = c.get_latest_block().unwrap();
                println!("Blockchain height: {}", height + 1);
                print_ascii_block(height, &latest.hash, &latest.header.prev_hash);
                println!("Latest block hash: {}", latest.hash);
                println!("Next block difficulty: {}", c.get_difficulty());
                println!("Connected peers: {}", p2p.peers.lock().unwrap().len());
            }
            "exit" => {
                break;
            }
            "" => continue,
            _ => {
                println!("Available commands: mine, balance, status, connect <ip:port>, exit");
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
