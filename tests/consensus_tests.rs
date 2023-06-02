use por_zero_knowledge_blockchain::consensus::por::{PORContract, ProofOfReserve};
use por_zero_knowledge_blockchain::blockchain::Transaction;

#[test]
fn test_proof_of_reserve_execution() {
    let mut por_contract = PORContract::new();
    let transaction = Transaction::new("from".into(), "to".into(), 10);

    let result = por_contract.execute(&transaction);
    assert!(result.is_ok());
}
