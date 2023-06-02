# PawChain Network 

A singularity chain order Proof of reserve blockchain architecture utilizing zero-knowledge proof for seamless multi blockchain network interaction and Interoperability.

`NOTE: This are Core Codes not fullyimplemented for test or production ready. This code in development and might not work 
as intended  for developer use only. `

`Pawchain  is a blockchain implementation with a Proof of Reserve consensus mechanism and zero-knowledge proof integration, designed to be secure and scalable. It provides a robust foundation for building decentralized applications that require transparency, privacy, and a high degree of security.`

# Table of Contents

- Features
- Installation
- Usage 
  - Running the Node
  - Examples
- Testing
- Structure
- Contributing 
- License

## Features

- Proof of Reserve consensus mechanism
- Integration of various zero-knowledge proof scheme:
  - zk-SNARKs
  - zk-STARKs
  - Bulletproof

- Cryptographic primitives for hashing and signing 
- P2P networking with node discovery and communication
- Simple Smart Contract system with PORContract
- Merkle tree implementation for efficient storage and 
  verification
- Utility function for serialization and deserialization

# Installation

To install the project, you need to have Rust and Cargo installed on your machine.

Clone the repository 
```
$ git clone https://github.com/zootclassic-dev/pawchain.git
$ cd pawchain
```

Build the project: 
```
$ cargo build --release
```

# Usage

Running The Node
To run the node, use the following command: 

```
$ cargo run
```

This will start a node with new blockchain and the Proof of Reserve consensus mechanism.

# Example 

The    `/examples` folder contains example code demonstrating how to use various components of pawchain network. (dev uses only).

To run an example , uses the ` cargo run --example` command follwoing by the name of the example: 

```
$ cargo run --example par_example
```

# Testing

To run the tests for the pawchain network, use the `cargo test` command in the project root

```
$ cargo test
```

This command will compile and run all the tests in the `/tests` directory.

# Structure 

The project is organized into the following modules
- `src/blockchain`: Contains the core blockchain components, such 
   asblocks, transactions, and the chain.
- `src/consensus`: Contains the consensus mechansim, inlduing Proof of
   Reserve, Proof of Work, and Proof of Stake.
- `src/network`: Contains networking components, including nodde discover, 
   P2P communication, and RPC.
- `src/smart_contracts`: Contains the smart contract system, including the 
  PORContract.
- `src/utils`: Contains utility functions, such as Merkle tree implementation
  and serialization. 

# Contributing 
This high level strucutred code for pawchain network architecture is provided for 
developers and open source use case by `coomunity dao` as an attempt to provide an initial building blocks for contributors and developers assisting in the 
development Pawchain blockchain. 

# License
This project is opened source for public use and contribution. 

# Code Structure
```lua 
pawchain_structure/
│
├── src/
│   ├── blockchain/
│   │   ├── mod.rs
│   │   ├── block.rs
│   │   ├── chain.rs
│   │   └── transaction.rs
│   │
│   ├── consensus/
│   │   ├── mod.rs
│   │   ├── por.rs
│   │   ├── pow.rs
│   │   └── pos.rs
│   │
│   ├── crypto/
│   │   ├── mod.rs
│   │   ├── hash.rs
│   │   ├── signature.rs
│   │   └── zero_knowledge/
│   │       ├── mod.rs
│   │       ├── zk_snarks.rs
│   │       ├── zk_starks.rs
│   │       └── bulletproofs.rs
│   │
│   ├── network/
│   │   ├── mod.rs
│   │   ├── node.rs
│   │   ├── p2p.rs
│   │   └── rpc.rs
│   │
│   ├── smart_contracts/
│   │   ├── mod.rs
│   │   └── por_contract.rs
│   │
│   ├── utils/
│   │   ├── mod.rs
│   │   ├── merkle_tree.rs
│   │   └── serialization.rs
│   │
│   └── main.rs
│
├── tests/
│   ├── blockchain_tests.rs
│   ├── consensus_tests.rs
│   ├── crypto_tests.rs
│   ├── network_tests.rs
│   └── smart_contracts_tests.rs
│
├── examples/
│   ├── por_example.rs
│   └── zero_knowledge_example.rs
│
├── Cargo.toml
└── README.md
```
