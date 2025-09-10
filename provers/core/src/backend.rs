use rsnark_core::types::{CircuitDefinition, PublicWitness, Witness};

use crate::{Curve, Proof};

pub trait Backend: Clone {
    type CircuitConstraint;
    type ProvingKey;
    type VerifyingKey;

    type Error: std::error::Error + Send + Sync + 'static;

    fn new(curve: Curve) -> Self;

    fn compile(&self, circuit: &CircuitDefinition) -> Result<Self::CircuitConstraint, Self::Error>;

    fn setup(
        &self,
        cs: &Self::CircuitConstraint,
    ) -> Result<(Self::ProvingKey, Self::VerifyingKey), Self::Error>;

    fn prove(
        &self,
        cs: &Self::CircuitConstraint,
        pk: &Self::ProvingKey,
        witness: &Witness,
    ) -> Result<Proof, Self::Error>;

    fn verify(
        &self,
        vk: &Self::VerifyingKey,
        proof: &Proof,
        public_witness: &PublicWitness,
    ) -> Result<bool, Self::Error>;
}
