use ed25519_dalek::{Keypair, PublicKey as DalekPublicKey, SecretKey as DalekSecretKey, Signature as DalekSignature};
use rand::rngs::OsRng;

pub struct PrivateKey(pub DalekSecretKey);

pub struct PublicKey(pub DalekPublicKey);

pub struct Signature(pub DalekSignature);

impl PrivateKey {
    pub fn generate() -> Self {
        let mut csprng = OsRng;
        let secret_key = DalekSecretKey::generate(&mut csprng);
        PrivateKey(secret_key)
    }
}

impl PublicKey {
    pub fn from_private_key(private_key: &PrivateKey) -> Self {
        let public_key = DalekPublicKey::from(&private_key.0);
        PublicKey(public_key)
    }
}

impl Signature {
    pub fn sign(private_key: &PrivateKey, message: &[u8]) -> Self {
        let keypair = Keypair {
            secret: private_key.0.clone(),
            public: PublicKey::from_private_key(&private_key).0,
        };

        let signature = keypair.sign(message);
        Signature(signature)
    }

    pub fn verify(&self, public_key: &PublicKey, message: &[u8]) -> bool {
        match public_key.0.verify(message, &self.0) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
