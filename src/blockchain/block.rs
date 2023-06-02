use crate::blockchain::Transaction;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub timestamp: u64,
    pub prev_block_hash: Vec<u8>,
    pub nonce: u64,
    pub merkle_root: Vec<u8>,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(prev_block_hash: Vec<u8>, transactions: Vec<Transaction>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let (merkle_root, _) = Self::calculate_merkle_root(&transactions);

        Block {
            timestamp,
            prev_block_hash,
            nonce: 0,
            merkle_root,
            transactions,
        }
    }

    pub fn calculate_merkle_root(transactions: &[Transaction]) -> (Vec<u8>, bool) {
        if transactions.is_empty() {
            return (vec![], false);
        }

        if transactions.len() == 1 {
            let tx_hash = transactions[0].calculate_hash();
            return (tx_hash, true);
        }

        let mut new_level = Vec::with_capacity((transactions.len() + 1) / 2);
        let mut skip = false;

        for txs in transactions.chunks(2) {
            let left_hash = txs[0].calculate_hash();
            let right_hash = if txs.len() == 1 {
                skip = true;
                left_hash.clone()
            } else {
                txs[1].calculate_hash()
            };

            let mut hasher = Sha256::new();
            hasher.update(left_hash);
            hasher.update(right_hash);
            let hash = hasher.finalize().to_vec();
            new_level.push(Transaction::new(vec![], vec![])); // Dummy transaction
            new_level.last_mut().unwrap().signature = Some(hash);
        }

        Self::calculate_merkle_root(&new_level)
    }

    pub fn mine(&mut self, target: &Vec<u8>) {
        while !self.is_valid(target) {
            self.nonce += 1;
        }
    }

    pub fn is_valid(&self, target: &Vec<u8>) -> bool {
        let block_hash = self.calculate_hash();
        block_hash < *target
    }

    pub fn calculate_hash(&self) -> Vec<u8> {
        let serialized_block = bincode::serialize(&self).expect("Cannot serialize block");
        let mut hasher = Sha256::new();
        hasher.update(serialized_block);
        hasher.finalize().to_vec()
    }
}
