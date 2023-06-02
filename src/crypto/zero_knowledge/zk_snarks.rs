use bellman::{
    groth16::{create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof, Parameters, PreparedVerifyingKey, Proof},
    Circuit, ConstraintSystem, LinearCombination, SynthesisError,
};
use pairing::{bls12_381::Bls12, Engine, Field};
use rand::rngs::OsRng;

pub struct ZkSnarks;

impl ZkSnarks {
    pub fn setup() -> (Parameters<Bls12>, PreparedVerifyingKey<Bls12>) {
        let rng = &mut OsRng;
        let params = {
            let c = ExampleCircuit::<Bls12> {
                x: None,
                y: None,
            };
            generate_random_parameters::<Bls12, _, _>(c, rng).unwrap()
        };
        let pvk = prepare_verifying_key(&params.vk);
        (params, pvk)
    }

    pub fn create_proof(
        params: &Parameters<Bls12>,
        x: Option<Field<Bls12>>,
        y: Option<Field<Bls12>>,
    ) -> Proof<Bls12> {
        let rng = &mut OsRng;
        let circuit = ExampleCircuit::<Bls12> { x, y };
        create_random_proof(circuit, params, rng).unwrap()
    }

    pub fn verify_proof(
        pvk: &PreparedVerifyingKey<Bls12>,
        proof: &Proof<Bls12>,
        x: Field<Bls12>,
        y: Field<Bls12>,
    ) -> bool {
        let public_inputs = vec![x, y];
        verify_proof(pvk, proof, &public_inputs).is_ok()
    }
}

struct ExampleCircuit<E: Engine> {
    pub x: Option<E::Fr>,
    pub y: Option<E::Fr>,
}

impl<E: Engine> Circuit<E> for ExampleCircuit<E> {
    fn synthesize<CS: ConstraintSystem<E>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let x = cs.alloc(|| "x", || self.x.ok_or(SynthesisError::AssignmentMissing))?;
        let y = cs.alloc_input(|| "y", || self.y.ok_or(SynthesisError::AssignmentMissing))?;

        let x_squared = cs.alloc(|| "x^2", || {
            let mut x_val = self.x.ok_or(SynthesisError::AssignmentMissing)?;
            x_val.square();
            Ok(x_val)
        })?;

        cs.enforce(
            || "x * x = x^2",
            |lc| lc + x,
            |lc| lc + x,
            |lc| lc + x_squared,
        );

        cs.enforce(
            || "x^2 + x = y",
            |lc| lc + x_squared + x,
            |lc| lc + CS::one(),
            |lc| lc + y,
        );

        Ok(())
    }
}
