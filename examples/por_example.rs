use por_zero_knowledge_blockchain::blockchain::{Block, Chain, Transaction};
use por_zero_knowledge_blockchain::consensus::por::{PORContract, ProofOfReserve};

fn main() {
    let por_contract = PORContract::new();
    let por = ProofOfReserve::new(por_contract);
    let mut chain = Chain::new();

    let transaction = Transaction::new("from".into(), "to".into(), 10);

    let block = Block::new(vec![transaction.clone()]);
    let is_valid = por.validate_block(&block, &chain);
    println!("Is block valid? {}", is_valid);

    if is_valid {
        chain.add_block(block);
        println!("Block added to the chain");
    } else {
        println!("Block rejected");
    }
}
