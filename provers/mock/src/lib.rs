use rsnark_core::{
    U256,
    types::{CircuitDefinition, PublicWitness, Witness},
};
use rsnark_provers_core::{Backend, Curve, Proof};
use sha3::{Digest, Sha3_256};

#[derive(Debug, thiserror::Error)]
pub enum Error {}

#[derive(Clone)]
pub struct MockProverBackend;

impl Backend for MockProverBackend {
    type CircuitConstraint = CircuitDefinition;
    type ProvingKey = ();
    type VerifyingKey = ();

    type Error = Error;

    fn new(_curve: Curve) -> Self {
        Self
    }

    fn compile(&self, circuit: &CircuitDefinition) -> Result<Self::CircuitConstraint, Self::Error> {
        Ok(circuit.clone())
    }

    fn setup(
        &self,
        _cs: &Self::CircuitConstraint,
    ) -> Result<(Self::ProvingKey, Self::VerifyingKey), Self::Error> {
        Ok(((), ()))
    }

    fn prove(
        &self,
        _cs: &Self::CircuitConstraint,
        _pk: &Self::ProvingKey,
        witness: &Witness,
    ) -> Result<Proof, Self::Error> {
        let res_hash = hash_public_witness(witness.public());
        Ok(Proof(vec![res_hash]))
    }

    fn verify(
        &self,
        _vk: &Self::VerifyingKey,
        proof: &Proof,
        public_witness: &PublicWitness,
    ) -> Result<bool, Self::Error> {
        let res_hash = hash_public_witness(&public_witness.public);
        if res_hash == proof.0[0] {
            Ok(true)
        } else {
            Ok(false)
        }
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
