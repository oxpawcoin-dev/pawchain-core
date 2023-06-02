use sha2::{Sha256, Digest};

pub struct Hash {
    pub hash: Vec<u8>,
}

impl Hash {
    pub fn new(input: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(input);
        let hash = hasher.finalize();
        
        Hash {
            hash: hash.to_vec(),
        }
    }

    pub fn from_str(input: &str) -> Self {
        let input_bytes = input.as_bytes();
        Hash::new(input_bytes)
    }

    pub fn to_hex(&self) -> String {
        self.hash
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<Vec<String>>()
            .join("")
    }
}
