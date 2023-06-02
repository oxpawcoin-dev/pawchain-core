use por_zero_knowledge_blockchain::network::node::Node;
use por_zero_knowledge_blockchain::blockchain::Chain;
use por_zero_knowledge_blockchain::consensus::por::{PORContract, ProofOfReserve};

#[test]
fn test_node_creation() {
    let por_contract = PORContract::new();
    let por = ProofOfReserve::new(por_contract);
    let chain = Chain::new();

    let node = Node::new(chain, por);
    assert!(node.is_connected());
}
