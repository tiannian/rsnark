use std::marker::PhantomData;

use rsnark_core::{
    CircuitElement,
    types::{PublicWitness, Witness},
};

use crate::{Backend, Proof, Prover};

pub struct CircuitProver<B, C>
where
    B: Backend,
{
    pub(crate) prover: Prover<B>,
    pub(crate) constraint: B::CircuitConstraint,
    pub(crate) marker: PhantomData<C>,
}

impl<B, C> CircuitProver<B, C>
where
    B: Backend,
    C: CircuitElement,
{
    pub fn setup(&self) -> (B::ProvingKey, B::VerifyingKey) {
        self.prover.backend.setup(&self.constraint)
    }

    // TODO: use C::witness type
    pub fn prove(&self, proving_key: &B::ProvingKey, circuit_witness: &C) -> (Proof, Witness) {
        let mut witness = Witness::new();

        circuit_witness.append_public(witness.public_mut());
        circuit_witness.append_private(witness.private_mut());

        let proof = self
            .prover
            .backend
            .prove(&self.constraint, proving_key, &witness);

        (proof, witness)
    }

    // TODO: use C::public_witness type
    pub fn verify(
        &self,
        verifying_key: &B::VerifyingKey,
        proof: &Proof,
        public_witness: &PublicWitness,
    ) -> bool {
        self.prover
            .backend
            .verify(verifying_key, proof, public_witness)
    }
}
