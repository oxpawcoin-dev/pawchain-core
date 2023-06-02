use por_zero_knowledge_blockchain::crypto::{hash::sha256, signature::generate_keypair};

#[test]
fn test_sha256_hash() {
    let data = "test data".as_bytes();
    let hash = sha256(data);

    assert_eq!(hash.len(), 32);
}

#[test]
fn test_keypair_generation() {
    let (private_key, public_key) = generate_keypair();

    assert_eq!(private_key.len(), 32);
    assert_eq!(public_key.len(), 64);
}
