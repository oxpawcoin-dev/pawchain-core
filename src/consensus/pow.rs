use crate::blockchain::{Blockchain, Block, Transaction};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

pub struct ProofOfWork {
    pub blockchain: Blockchain,
    pub mining_target: Vec<u8>,
    pub mining_threads: usize,
}

impl ProofOfWork {
    pub fn new(blockchain: Blockchain, mining_target: Vec<u8>, mining_threads: usize) -> Self {
        ProofOfWork {
            blockchain,
            mining_target,
            mining_threads,
        }
    }

    pub fn mine_block(&mut self, transactions: Vec<Transaction>) -> Block {
        let prev_block_hash = self.blockchain.get_last_block().calculate_hash();
        let mut new_block = Block::new(prev_block_hash, transactions);

        let found_block = Arc::new(AtomicBool::new(false));
        let mined_block = Arc::new(Mutex::new(None));
        let mining_target = Arc::new(self.mining_target.clone());

        let mut mining_threads = Vec::with_capacity(self.mining_threads);

        for _ in 0..self.mining_threads {
            let found_block = found_block.clone();
            let mined_block = mined_block.clone();
            let mining_target = mining_target.clone();
            let mut new_block = new_block.clone();

            let handle = thread::spawn(move || {
                while !found_block.load(Ordering::Relaxed) {
                    new_block.nonce += 1;
                    if new_block.is_valid(&mining_target) {
                        found_block.store(true, Ordering::Relaxed);
                        *mined_block.lock().unwrap() = Some(new_block);
                    }
                }
            });

            mining_threads.push(handle);
        }

        for handle in mining_threads {
            handle.join().unwrap();
        }

        let mined_block = Arc::try_unwrap(mined_block)
            .unwrap()
            .into_inner()
            .unwrap()
            .unwrap();

        self.blockchain.add_block(mined_block.transactions.clone());
        mined_block
    }
}
