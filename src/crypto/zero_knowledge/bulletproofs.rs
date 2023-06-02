use bulletproofs::{
    r1cs::{ConstraintSystem, LinearCombination, Prover, R1CSError, Variable, Verifier},
    BulletproofGens, PedersenGens, Proof, R1CSProof,
};
use curve25519_dalek::scalar::Scalar;

pub struct Bulletproofs {
    pub pc_gens: PedersenGens,
    pub bp_gens: BulletproofGens,
}

impl Bulletproofs {
    pub fn new() -> Self {
        Bulletproofs {
            pc_gens: PedersenGens::default(),
            bp_gens: BulletproofGens::new(64, 1),
        }
    }

    pub fn create_proof(
        &self,
        secret: u64,
        value: u64,
    ) -> Result<(Proof, R1CSProof), R1CSError> {
        let mut prover = Prover::new(&self.pc_gens);

        let (com_secret, var_secret) = prover.commit(Scalar::from(secret), Scalar::random(&mut rand::thread_rng()));
        let (com_value, var_value) = prover.commit(Scalar::from(value), Scalar::random(&mut rand::thread_rng()));

        prover.constrain(com_secret - com_value);

        let cs = prover.constraint_system();
        cs.multiply(LinearCombination::from(var_secret), LinearCombination::from(var_value), LinearCombination::zero());

        let proof = prover.prove(&self.bp_gens)?;

        Ok((proof, prover.r1cs_proof()))
    }

    pub fn verify_proof(
        &self,
        proof: &Proof,
        r1cs_proof: &R1CSProof,
        commitment: &Scalar,
    ) -> Result<(), R1CSError> {
        let mut verifier = Verifier::new(&self.pc_gens);
        let var = verifier.commit(commitment.clone());

        verifier.constrain(*commitment);

        let cs = verifier.constraint_system();
        cs.multiply(LinearCombination::from(var), LinearCombination::from(var), LinearCombination::zero());

        verifier.verify(&proof, &self.bp_gens, &r1cs_proof)?;

        Ok(())
    }
}
