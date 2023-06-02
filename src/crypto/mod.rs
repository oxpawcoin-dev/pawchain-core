mod hash;
mod signature;
mod zero_knowledge;

pub use self::hash::Hash;
pub use self::signature::{PrivateKey, PublicKey, Signature};
pub use self::zero_knowledge::{ZkSnarks, ZkStarks, Bulletproofs};

pub enum CryptoAlgorithm {
    Hash(Hash),
    Signature(Signature),
    ZeroKnowledge {
        zk_snarks: ZkSnarks,
        zk_starks: ZkStarks,
        bulletproofs: Bulletproofs,
    },
}

impl CryptoAlgorithm {
    pub fn new() -> Self {
        CryptoAlgorithm::ZeroKnowledge {
            zk_snarks: ZkSnarks::new(),
            zk_starks: ZkStarks::new(),
            bulletproofs: Bulletproofs::new(),
        }
    }
}
