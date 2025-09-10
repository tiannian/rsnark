use std::marker::PhantomData;

use anyhow::Result;
use rsnark_core::{CircuitPublicWitness, CircuitWitness, types};

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
    C: CircuitWitness,
{
    pub fn setup(&self) -> Result<(B::ProvingKey, B::VerifyingKey)> {
        Ok(self.prover.backend.setup(&self.constraint)?)
    }

    pub fn prove(&self, proving_key: &B::ProvingKey, circuit_witness: &C) -> Result<Proof> {
        let mut witness = types::Witness::new();

        circuit_witness.append_public(witness.public_mut());
        circuit_witness.append_private(witness.private_mut());

        let proof = self
            .prover
            .backend
            .prove(&self.constraint, proving_key, &witness)?;

        Ok(proof)
    }

    pub fn verify(
        &self,
        verifying_key: &B::VerifyingKey,
        proof: &Proof,
        public_witness: C::PublicWitness,
    ) -> Result<()>
    where
        C::PublicWitness: CircuitPublicWitness,
    {
        let mut witness = types::PublicWitness::new();

        public_witness.append_public(witness.public_mut());

        self.prover.backend.verify(verifying_key, proof, &witness)?;

        Ok(())
    }
}
