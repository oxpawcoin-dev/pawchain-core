use blake3::{Hash, Hasher};
use std::collections::VecDeque;

pub struct MerkleTree {
    root: Hash,
    leaf_hashes: Vec<Hash>,
}

impl MerkleTree {
    pub fn new(data: &[&[u8]]) -> Self {
        let leaf_hashes = data.iter().map(Self::hash_leaf).collect::<Vec<_>>();
        let root = Self::build_tree(leaf_hashes.clone());
        MerkleTree { root, leaf_hashes }
    }

    pub fn root(&self) -> &Hash {
        &self.root
    }

    pub fn leaf_hashes(&self) -> &[Hash] {
        &self.leaf_hashes
    }

    fn build_tree(leaf_hashes: Vec<Hash>) -> Hash {
        let mut node_queue = VecDeque::from(leaf_hashes);

        while node_queue.len() > 1 {
            let left = node_queue.pop_front().unwrap();
            let right = node_queue.pop_front().unwrap();
            let parent = Self::hash_nodes(&left, &right);
            node_queue.push_back(parent);
        }

        node_queue.pop_front().unwrap()
    }

    fn hash_leaf(data: &&[u8]) -> Hash {
        let mut hasher = Hasher::new();
        hasher.update(b"Leaf:");
        hasher.update(data);
        hasher.finalize()
    }

    fn hash_nodes(left: &Hash, right: &Hash) -> Hash {
        let mut hasher = Hasher::new();
        hasher.update(b"Node:");
        hasher.update(left.as_bytes());
        hasher.update(right.as_bytes());
        hasher.finalize()
    }
}
