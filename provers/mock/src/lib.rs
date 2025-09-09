use rsnark_core::types::{CircuitDefinition, PublicWitness, Witness};
use rsnark_provers_core::{Backend, Curve, Proof};

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
        _witness: &Witness,
    ) -> Proof {
        Proof(vec![])
    }

    fn verify(
        &self,
        _vk: &Self::VerifyingKey,
        _proof: &Proof,
        _public_witness: &PublicWitness,
    ) -> bool {
        true
    }
}
