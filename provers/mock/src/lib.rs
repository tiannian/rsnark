use rsnark_core::{
    U256,
    types::{CircuitDefinition, PublicWitness, Witness},
};
use rsnark_provers_core::{Backend, Curve, Proof};
use sha3::{Digest, Sha3_256};

#[derive(Clone)]
pub struct MockProverBackend;

impl Backend for MockProverBackend {
    type CircuitConstraint = CircuitDefinition;
    type ProvingKey = ();
    type VerifyingKey = ();

    fn new() -> Self {
        Self
    }

    fn compile(&self, _curve: &Curve, circuit: &CircuitDefinition) -> Self::CircuitConstraint {
        circuit.clone()
    }

    fn setup(&self, _cs: &Self::CircuitConstraint) -> (Self::ProvingKey, Self::VerifyingKey) {
        ((), ())
    }

    fn prove(
        &self,
        _cs: &Self::CircuitConstraint,
        _pk: &Self::ProvingKey,
        witness: &Witness,
    ) -> Proof {
        let res_hash = hash_public_witness(witness.public());
        Proof(vec![res_hash])
    }

    fn verify(
        &self,
        _vk: &Self::VerifyingKey,
        proof: &Proof,
        public_witness: &PublicWitness,
    ) -> bool {
        let res_hash = hash_public_witness(&public_witness.public);
        res_hash == proof.0[0]
    }
}

fn hash_public_witness(public_witness: &[U256]) -> U256 {
    let mut hasher = Sha3_256::new();

    for value in public_witness {
        let bytes: [u8; 32] = value.to_be_bytes();
        hasher.update(&bytes);
    }

    let res_bytes = hasher.finalize();
    U256::from_be_slice(&res_bytes)
}
