use distaff::{hash::Hash, stark::StarkProof, Program, Prover, Verifier};
use rand::Rng;

pub struct ZkStarks;

impl ZkStarks {
    pub fn example_circuit() -> Program {
        // Implement your specific zk-STARKs circuit here.
        Program::new(vec![])
    }

    pub fn create_proof(program: &Program, inputs: &[u128], outputs: &[u128]) -> StarkProof {
        let mut rng = rand::thread_rng();
        let secret_seed = rng.gen::<[u8; 32]>();
        Prover::prove(program, Hash(secret_seed), inputs, outputs).unwrap()
    }

    pub fn verify_proof(
        program: &Program,
        inputs: &[u128],
        outputs: &[u128],
        proof: &StarkProof,
    ) -> bool {
        Verifier::verify(program, inputs, outputs, proof).is_ok()
    }
}
