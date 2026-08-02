use crate::blockchain::block::Block;

pub struct ProofOfWork;

impl ProofOfWork {
    pub fn mine(block: &mut Block) {
        let prefix = "0".repeat(block.header.difficulty as usize);
        
        loop {
            block.hash = block.calculate_hash();
            if block.hash.starts_with(&prefix) {
                break;
            }
            block.header.nonce += 1;
        }
    }

    pub fn is_valid(block: &Block) -> bool {
        let prefix = "0".repeat(block.header.difficulty as usize);
        block.hash.starts_with(&prefix) && block.hash == block.calculate_hash()
    }
}
