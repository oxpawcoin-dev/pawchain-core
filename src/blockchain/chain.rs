use crate::blockchain::{Block, Transaction};
use sha2::{Sha256, Digest};

pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(vec![], vec![]);
        Blockchain {
            chain: vec![genesis_block],
        }
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let prev_block_hash = self.chain.last().unwrap().calculate_hash();
        let mut block = Block::new(prev_block_hash, transactions);
        // Define your mining target here. This is an example of a simple target.
        let target = vec![0; 32];

        block.mine(&target);
        self.chain.push(block);
    }

    pub fn is_valid(&self) -> bool {
        let mut prev_hash: Vec<u8> = vec![];
        let target = vec![0; 32]; // Use the same target as in the add_block function

        for block in &self.chain {
            let block_hash = block.calculate_hash();

            // Validate Proof of Work
            if block_hash >= target {
                return false;
            }

            // Validate block hash and previous block hash
            if prev_hash.len() > 0 && block.prev_block_hash != prev_hash {
                return false;
            }

            // Validate Merkle root
            let (merkle_root, is_valid) = Block::calculate_merkle_root(&block.transactions);
            if !is_valid || merkle_root != block.merkle_root {
                return false;
            }

            prev_hash = block_hash;
        }

        true
    }

    pub fn get_last_block(&self) -> &Block {
        self.chain.last().unwrap()
    }
}
