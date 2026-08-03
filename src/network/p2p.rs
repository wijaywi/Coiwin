use crate::blockchain::block::Block;
use crate::blockchain::chain::Blockchain;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Serialize, Deserialize, Debug)]
pub enum P2PMessage {
    RequestBlocks,
    SendBlocks(Vec<Block>),
    NewBlock(Block),
}

pub struct P2PNode {
    pub chain: Arc<Mutex<Blockchain>>,
    pub peers: Arc<Mutex<Vec<String>>>, // IP:Port strings
}

impl P2PNode {
    pub fn new(chain: Arc<Mutex<Blockchain>>) -> Self {
        P2PNode {
            chain,
            peers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn start_server(&self, port: &str) {
        let bind_addr = format!("0.0.0.0:{}", port);
        let listener = TcpListener::bind(&bind_addr).expect("Could not bind to port");
        println!("P2P Server listening on {}", bind_addr);
        
        let chain_ref = Arc::clone(&self.chain);
        let peers_ref = Arc::clone(&self.peers);

        thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        let chain_clone = Arc::clone(&chain_ref);
                        let _peers_clone = Arc::clone(&peers_ref);
                        thread::spawn(move || {
                            Self::handle_connection(stream, chain_clone);
                        });
                    }
                    Err(e) => {
                        println!("Failed connection: {}", e);
                    }
                }
            }
        });
    }

    fn handle_connection(mut stream: TcpStream, chain: Arc<Mutex<Blockchain>>) {
        let mut buffer = [0; 1024 * 1024]; // 1MB buffer for blocks
        if let Ok(size) = stream.read(&mut buffer) {
            if size > 0 {
                let msg_str = String::from_utf8_lossy(&buffer[..size]);
                if let Ok(msg) = serde_json::from_str::<P2PMessage>(&msg_str) {
                    match msg {
                        P2PMessage::RequestBlocks => {
                            let blocks = chain.lock().unwrap().blocks.clone();
                            let response = P2PMessage::SendBlocks(blocks);
                            if let Ok(response_json) = serde_json::to_string(&response) {
                                let _ = stream.write_all(response_json.as_bytes());
                            }
                        }
                        P2PMessage::SendBlocks(received_blocks) => {
                            let mut local_chain = chain.lock().unwrap();
                            if received_blocks.len() > local_chain.blocks.len() {
                                println!("Discovered a longer chain! Updating local blockchain...");
                                local_chain.blocks = received_blocks;
                                local_chain.recalculate_balances(); 
                                local_chain.save_to_disk();
                            }
                        }
                        P2PMessage::NewBlock(block) => {
                            let mut local_chain = chain.lock().unwrap();
                            let prev_hash = local_chain.get_latest_block().unwrap().hash.clone();
                            if block.header.prev_hash == prev_hash {
                                println!("Received new block from peer!");
                                local_chain.update_balances(&block.transactions);
                                local_chain.add_block(block);
                                local_chain.save_to_disk();
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn connect_to_peer(&self, peer_addr: &str) {
        if let Ok(mut stream) = TcpStream::connect(peer_addr) {
            println!("Connected to peer {}", peer_addr);
            self.peers.lock().unwrap().push(peer_addr.to_string());
            
            // Ask for blocks
            let msg = P2PMessage::RequestBlocks;
            if let Ok(msg_json) = serde_json::to_string(&msg) {
                let _ = stream.write_all(msg_json.as_bytes());
            }
            
            let mut buffer = [0; 1024 * 1024];
            if let Ok(size) = stream.read(&mut buffer) {
                if size > 0 {
                    let msg_str = String::from_utf8_lossy(&buffer[..size]);
                    if let Ok(P2PMessage::SendBlocks(received_blocks)) = serde_json::from_str(&msg_str) {
                        let mut local_chain = self.chain.lock().unwrap();
                        if received_blocks.len() > local_chain.blocks.len() {
                            println!("Discovered a longer chain from peer! Syncing...");
                            local_chain.blocks = received_blocks;
                            local_chain.recalculate_balances();
                            local_chain.save_to_disk();
                        } else {
                            println!("Peer's chain is not longer than ours.");
                        }
                    }
                }
            }
        } else {
            println!("Failed to connect to {}", peer_addr);
        }
    }

    pub fn broadcast_new_block(&self, block: &Block) {
        let peers = self.peers.lock().unwrap().clone();
        for peer in peers {
            if let Ok(mut stream) = TcpStream::connect(&peer) {
                let msg = P2PMessage::NewBlock(block.clone());
                if let Ok(msg_json) = serde_json::to_string(&msg) {
                    let _ = stream.write_all(msg_json.as_bytes());
                }
            }
        }
    }
}
