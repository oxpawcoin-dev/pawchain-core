use por_zero_knowledge_blockchain::smart_contracts::por_contract::PORContract;
use por_zero_knowledge_blockchain::blockchain::Transaction;

#[test]
fn test_por_contract_validation() {
    let por_contract = PORContract::new();
    let transaction = Transaction::new("from".into(), "to".into(), 10);

    let is_valid = por_contract.validate_transaction(&transaction);
    assert!(is_valid);
}
