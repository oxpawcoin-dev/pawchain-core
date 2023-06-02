use por_zero_knowledge_blockchain::crypto::zero_knowledge::zk_snarks::{generate_proof, generate_random_parameters, verify_proof};

fn main() {
    let (params, vk) = generate_random_parameters();

    let witness = vec![1, 1];
    let public_inputs = vec![2];

    let proof = match generate_proof(&params, &witness, &public_inputs) {
        Ok(p) => p,
        Err(e) => {
            println!("Failed to generate proof: {}", e);
            return;
        }
    };

    let is_valid = verify_proof(&vk, &proof, &public_inputs);
    println!("Is proof valid? {}", is_valid);
}
