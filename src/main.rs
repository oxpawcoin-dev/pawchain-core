use crate::blockchain::{Block, Chain, Transaction};
use crate::consensus::por::ProofOfReserve;
use crate::network::node::Node;
use crate::smart_contracts::por_contract::PORContract;
use crate::utils::{deserialize, serialize};
use std::env;

mod blockchain;
mod consensus;
mod crypto;
mod network;
mod smart_contracts;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: por_zero_knowledge_blockchain <command>");
        println!("Commands:");
        println!("  run-node: Start a new node");
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "run-node" => run_node(),
        _ => println!("Unknown command: {}", command),
    }
}

fn run_node() {
    let por_contract = PORContract::new();
    let por = ProofOfReserve::new(por_contract);
    let chain = Chain::new();
    let mut node = Node::new(chain, por);

    node.run();
}
