use crate::blockchain::{Block, Chain};
use crate::consensus::{ProofOfReserve, ProofOfWork, ProofOfStake};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Node {
    chain: Arc<Mutex<Chain>>,
    peers: Vec<String>,
}

impl Node {
    pub fn new() -> Self {
        Node {
            chain: Arc::new(Mutex::new(Chain::new())),
            peers: vec![],
        }
    }

    pub fn add_peer(&mut self, address: String) {
        self.peers.push(address);
    }

    pub fn sync_with_peers(&self) {
        // Implement logic to sync blockchain data with connected peers.
    }

    pub fn mine_block(&self) {
        let mut chain = self.chain.lock().unwrap();
        let prev_block = chain.get_last_block().clone();

        let proof_of_work = ProofOfWork::new();
        let new_block = proof_of_work.mine(&prev_block);

        chain.add_block(new_block);
    }

    pub fn run(&self) {
        loop {
            self.sync_with_peers();
            self.mine_block();
            thread::sleep(std::time::Duration::from_secs(10));
        }
    }
}
