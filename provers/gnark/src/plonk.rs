use std::marker::PhantomData;

use rsnark_core::types::{CircuitDefinition, PublicWitness, Witness};
use rsnark_provers_core::{Backend, CurveId};

use crate::{
    Error, Result, ffi,
    types::{CompiledCircuit, GoInnerRef, PlonkProof, PlonkProvingKey, PlonkVerifyingKey},
};

pub struct PlonkBackend<C> {
    go_ref_id: u64,
    marker: PhantomData<C>,
}

impl<C> Drop for PlonkBackend<C> {
    fn drop(&mut self) {
        ffi::plonk::remove_prover(self.go_ref_id);
    }
}

impl<C> PlonkBackend<C>
where
    C: CurveId,
{
    fn _new() -> Self {
        let curve = C::curve_id();

        let prover = ffi::plonk::create(curve);

        Self {
            go_ref_id: prover,
            marker: PhantomData,
        }
    }

    fn _compile(&self, circuit: &CircuitDefinition) -> Result<CompiledCircuit<C>> {
        let circuit = serde_json::to_vec(circuit)?;

        let curve = C::curve_id();

        let res = ffi::plonk::compile(curve, circuit);

        if res >= 0 {
            Ok(CompiledCircuit::from_go_inner_ref(res))
        } else {
            Err(Error::from_go_error(res))
        }
    }

    fn _setup(
        &self,
        compiled_circuit: &CompiledCircuit<C>,
    ) -> Result<(PlonkProvingKey<C>, PlonkVerifyingKey<C>)> {
        let res = ffi::plonk::setup(self.go_ref_id, compiled_circuit.go_inner_ref());

        let res0 = i64::from_be_bytes(res[0..8].try_into().unwrap());
        let res1 = i64::from_be_bytes(res[8..16].try_into().unwrap());

        if res0 >= 0 && res1 >= 0 {
            Ok((
                PlonkProvingKey::from_go_inner_ref(res0),
                PlonkVerifyingKey::from_go_inner_ref(res1),
            ))
        } else {
            Err(Error::from_go_error(res0))
        }
    }

    fn _prove(
        &self,
        compiled_circuit: &CompiledCircuit<C>,
        pk: &PlonkProvingKey<C>,
        witness: &Witness,
    ) -> Result<PlonkProof<C>> {
        let witness_bytes = serde_json::to_vec(witness)?;

        let res = ffi::plonk::prove(
            self.go_ref_id,
            compiled_circuit.go_inner_ref(),
            pk.go_inner_ref(),
            witness_bytes,
        );

        if res < 0 {
            Err(Error::from_go_error(res))
        } else {
            Ok(PlonkProof::from_go_inner_ref(res))
        }
    }

    fn _verify(
        &self,
        vk: &PlonkVerifyingKey<C>,
        proof: &PlonkProof<C>,
        public_witness: &PublicWitness,
    ) -> Result<bool> {
        let public_witness_bytes = serde_json::to_vec(public_witness)?;

        let res = ffi::plonk::verify(
            self.go_ref_id,
            vk.go_inner_ref(),
            proof.go_inner_ref(),
            public_witness_bytes,
        );

        if res == 0 {
            Ok(true)
        } else if res == -20010 {
            Ok(false)
        } else {
            Err(Error::from_go_error(res))
        }
    }
}

impl<C> Backend for PlonkBackend<C>
where
    C: CurveId,
{
    type CircuitConstraint = CompiledCircuit<C>;
    type ProvingKey = PlonkProvingKey<C>;
    type VerifyingKey = PlonkVerifyingKey<C>;
    type Proof = PlonkProof<C>;

    type Error = Error;

    fn new() -> Self {
        Self::_new()
    }

    fn compile(&self, circuit: &CircuitDefinition) -> Result<Self::CircuitConstraint> {
        self._compile(circuit)
    }

    fn setup(
        &self,
        cs: &Self::CircuitConstraint,
    ) -> Result<(Self::ProvingKey, Self::VerifyingKey)> {
        self._setup(cs)
    }

    fn prove(
        &self,
        compiled_circuit: &Self::CircuitConstraint,
        pk: &Self::ProvingKey,
        witness: &Witness,
    ) -> Result<Self::Proof> {
        let proof = self._prove(compiled_circuit, pk, witness)?;

        Ok(proof)
    }

    fn verify(
        &self,
        vk: &Self::VerifyingKey,
        proof: &Self::Proof,
        public_witness: &PublicWitness,
    ) -> Result<bool> {
        self._verify(vk, proof, public_witness)
    }
}
