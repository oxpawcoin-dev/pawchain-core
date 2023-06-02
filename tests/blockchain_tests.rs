use por_zero_knowledge_blockchain::blockchain::{Block, Chain, Transaction};

#[test]
fn test_add_block() {
    let mut chain = Chain::new();
    let transactions = vec![Transaction::new("from".into(), "to".into(), 10)];
    let block = Block::new(transactions);

    chain.add_block(block);
    assert_eq!(chain.blocks.len(), 2);
}
