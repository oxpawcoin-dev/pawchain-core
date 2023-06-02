use secp256k1::{PublicKey, SecretKey, Signature, Message, sign, verify};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Transaction {
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
    pub signature: Option<Signature>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransactionInput {
    pub prev_tx_hash: Vec<u8>,
    pub output_index: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransactionOutput {
    pub recipient: PublicKey,
    pub amount: u64,
}

impl Transaction {
    pub fn new(inputs: Vec<TransactionInput>, outputs: Vec<TransactionOutput>) -> Self {
        Transaction {
            inputs,
            outputs,
            signature: None,
        }
    }

    pub fn sign(&mut self, secret_key: &SecretKey) {
        let message = self.calculate_hash();
        let msg = Message::parse_slice(&message).expect("32-byte message");
        let (signature, _) = sign(&msg, secret_key);
        self.signature = Some(signature);
    }

    pub fn verify_signature(&self) -> Result<(), String> {
        if let Some(signature) = &self.signature {
            let message = self.calculate_hash();
            let msg = Message::parse_slice(&message).expect("32-byte message");
            let public_key = &self.inputs[0].get_address();
            verify(&msg, &signature, public_key)
                .map_err(|_| "Invalid signature".to_string())
        } else {
            Err("No signature provided".to_string())
        }
    }

    fn calculate_hash(&self) -> Vec<u8> {
        let mut tx_clone = self.clone();
        tx_clone.signature = None;
        let serialized_tx = bincode::serialize(&tx_clone).expect("Cannot serialize transaction");
        let mut hasher = Sha256::new();
        hasher.update(serialized_tx);
        hasher.finalize().to_vec()
    }
}

impl TransactionInput {
    pub fn new(prev_tx_hash: Vec<u8>, output_index: u32) -> Self {
        TransactionInput {
            prev_tx_hash,
            output_index,
        }
    }

    fn get_address(&self) -> PublicKey {
        // Implement logic for retrieving the public key associated with the transaction input.
    }
}

impl TransactionOutput {
    pub fn new(recipient: PublicKey, amount: u64) -> Self {
        TransactionOutput { recipient, amount }
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Inputs: {:?}\nOutputs: {:?}\nSignature: {:?}",
            self.inputs, self.outputs, self.signature
        )
    }
}
