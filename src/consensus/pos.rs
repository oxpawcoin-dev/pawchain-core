use crate::blockchain::{Blockchain, Block, Transaction};
use rand::Rng;
use std::collections::HashMap;

pub struct ProofOfStake {
    pub blockchain: Blockchain,
    pub validator_weights: HashMap<String, u64>,
}

impl ProofOfStake {
    pub fn new(blockchain: Blockchain) -> Self {
        ProofOfStake {
            blockchain,
            validator_weights: HashMap::new(),
        }
    }

    pub fn add_validator(&mut self, public_key: String, weight: u64) {
        self.validator_weights.insert(public_key, weight);
    }

    pub fn mine_block(&mut self, transactions: Vec<Transaction>) -> Block {
        let prev_block_hash = self.blockchain.get_last_block().calculate_hash();
        let validator_public_key = self.select_validator();
        let mut new_block = Block::new(prev_block_hash, transactions);

        // In PoS, the selected validator signs the block.
        // For simplicity, we will just set the nonce to the validator's weight.
        new_block.nonce = self.validator_weights[&validator_public_key];

        self.blockchain.add_block(new_block.transactions.clone());
        new_block
    }

    fn select_validator(&self) -> String {
        let total_weight: u64 = self.validator_weights.values().sum();
        let mut rng = rand::thread_rng();
        let target = rng.gen_range(0..total_weight);

        let mut current_sum = 0;
        for (public_key, weight) in &self.validator_weights {
            current_sum += weight;
            if current_sum >= target {
                return public_key.to_string();
            }
        }

        panic!("Failed to select a validator");
    }
}
