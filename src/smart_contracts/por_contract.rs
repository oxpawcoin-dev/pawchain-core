use crate::crypto::address::{generate_keypair, public_key_to_address}; 
use crate::blockchain::Transaction;
use crate::blockchain::Token; // Imported the Token struct
use std::collections::HashMap;

const INITIAL_SUPPLY: u64 = 1_000_000_000; // Added the INITIAL_SUPPLY constant

pub struct PORContract {
    token: Token, // Add a token field
    reserves: HashMap<String, u64>,
    balances: HashMap<String, u64>,
}

impl PORContract {
    pub fn new() -> Self {
        let token = Token {
            name: "Pawcoin".to_string(),
            symbol: "PCO".to_string(),
            decimals: 8,
        };

        let mut contract = PORContract {
            token,
            reserves: HashMap::new(),
            balances: HashMap::new(),
        };

        // Generate the genesis address
        let genesis_keypair = generate_keypair();
        let genesis_address = public_key_to_address(&genesis_keypair.public);


        // Distribute the initial supply to a specific address or among several addresses
        contract.balances.insert("genesis_address".to_string(), INITIAL_SUPPLY);
        contract
    }

    pub fn update_reserves(&mut self, transaction: &Transaction) {
        // Update reserves based on the transaction data
    }

    pub fn distribute_reward(&mut self, validator_address: &str, transaction_fee: u64) {
        let validator_balance = self.balances.entry(validator_address.to_string()).or_insert(0);
        *validator_balance += BLOCK_REWARD + transaction_fee;
    }
    
    // Add a function to update balances
    pub fn update_balances(&mut self, transaction: &Transaction) -> Result<(), String> {
        let sender_balance = self.balances.entry(transaction.from.clone()).or_insert(0);
        let receiver_balance = self.balances.entry(transaction.to.clone()).or_insert(0);

        if *sender_balance < transaction.amount + transaction.fee {
            return Err("Insufficient balance".into());
        }

        *sender_balance -= transaction.amount + transaction.fee;
        *receiver_balance += transaction.amount;

        Ok(())
    }

    pub fn validate_transaction(&self, transaction: &Transaction) -> bool {
        // Validate the transaction based on the current reserve data and balances
    }

    pub fn execute(&mut self, transaction: &Transaction, validator_address: &str) -> Result<(), String> {
        if !self.validate_transaction(transaction) {
            return Err("Invalid transaction".into());
        }
    
        self.update_reserves(transaction)?;
        self.update_balances(transaction)?;
        self.distribute_reward(validator_address, transaction.fee);
        Ok(())
    }
    

    // Add a function to access the token information
    pub fn get_token_info(&self) -> &Token {
        &self.token
    }
}
