mod pow;
mod pos;
mod por;

pub use self::pow::ProofOfWork;
pub use self::pos::ProofOfStake;
pub use self::por::ProofOfReserve;

pub enum ConsensusAlgorithm {
    ProofOfWork(ProofOfWork),
    ProofOfStake(ProofOfStake),
    ProofOfReserve(ProofOfReserve),
}

impl ConsensusAlgorithm {
    pub fn mine_block(&mut self, transactions: Vec<crate::blockchain::Transaction>) -> crate::blockchain::Block {
        match self {
            ConsensusAlgorithm::ProofOfWork(pow) => pow.mine_block(transactions),
            ConsensusAlgorithm::ProofOfStake(pos) => pos.mine_block(transactions),
            ConsensusAlgorithm::ProofOfReserve(por) => {
                // You may need to modify this part according to your Proof of Reserve implementation
                unimplemented!("Mining with Proof of Reserve is not implemented yet")
            }
        }
    }
}
