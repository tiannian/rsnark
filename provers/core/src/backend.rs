use rsnark_core::types::{CircuitDefinition, PublicWitness, Witness};

use crate::Proof;

pub enum Curve {
    BN254,
    BLS12_381,
}

pub trait Backend {
    type CircuitConstraint;
    type ProvingKey;
    type VerifyingKey;

    fn new() -> Self;

    fn compile(&self, curve: &Curve, circuit: &CircuitDefinition) -> Self::CircuitConstraint;

    fn setup(&self, cs: &Self::CircuitConstraint) -> (Self::ProvingKey, Self::VerifyingKey);

    fn prove(
        &self,
        cs: &Self::CircuitConstraint,
        pk: &Self::ProvingKey,
        witness: &Witness,
    ) -> Proof;

    fn verify(
        &self,
        vk: &Self::VerifyingKey,
        proof: &Proof,
        public_witness: &PublicWitness,
    ) -> bool;
}
