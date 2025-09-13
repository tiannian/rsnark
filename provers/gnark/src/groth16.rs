use std::marker::PhantomData;

use rsnark_core::{
    CurveId, MetadataInfo, ProvingSystem, U256,
    types::{CircuitDefinition, PublicWitness, Witness},
};
use rsnark_provers_core::Backend;

use crate::{
    Error, Result, ffi,
    types::{CompiledCircuit, GoInnerRef, Groth16Proof, Groth16ProvingKey, Groth16VerifyingKey},
};

/// Groth16 backend implementation using the Gnark library.
///
/// This backend provides a Rust interface to Gnark's Groth16 implementation,
/// offering high-performance zero-knowledge proof generation and verification.
/// The backend manages Go-side resources through FFI and supports multiple
/// elliptic curves.
///
/// # Type Parameters
///
/// * `C` - The elliptic curve type that implements [`CurveId`]
///
/// # Resource Management
///
/// The backend maintains a reference to Go-side objects through `go_ref_id`.
/// These resources are managed by the Gnark library and cleaned up automatically
/// when no longer referenced.
pub struct Groth16Backend<C> {
    go_ref_id: u64,
    marker: PhantomData<C>,
}

impl<C> Drop for Groth16Backend<C> {
    fn drop(&mut self) {
        ffi::groth16::remove_prover(self.go_ref_id);
    }
}

impl<C> Clone for Groth16Backend<C>
where
    C: CurveId,
{
    fn clone(&self) -> Self {
        Self {
            go_ref_id: self.go_ref_id,
            marker: PhantomData,
        }
    }
}

impl<C> Groth16Backend<C>
where
    C: CurveId,
{
    fn _new() -> Self {
        let curve = C::curve_id();

        let prover = ffi::groth16::create(curve);

        Self {
            go_ref_id: prover,
            marker: PhantomData,
        }
    }

    fn _compile(&self, circuit: &CircuitDefinition) -> Result<CompiledCircuit<C>> {
        let circuit = serde_json::to_vec(circuit)?;

        let curve = C::curve_id();

        let res = ffi::groth16::compile(curve, circuit);

        if res >= 0 {
            Ok(CompiledCircuit::from_go_inner_ref(res))
        } else {
            Err(Error::from_go_error(res))
        }
    }

    fn _setup(
        &self,
        compiled_circuit: &CompiledCircuit<C>,
    ) -> Result<(Groth16ProvingKey<C>, Groth16VerifyingKey<C>)> {
        let res = ffi::groth16::setup(self.go_ref_id, compiled_circuit.go_inner_ref());

        let res0 = i64::from_be_bytes(res[0..8].try_into().unwrap());
        let res1 = i64::from_be_bytes(res[8..16].try_into().unwrap());

        if res0 >= 0 && res1 >= 0 {
            Ok((
                Groth16ProvingKey::from_go_inner_ref(res0),
                Groth16VerifyingKey::from_go_inner_ref(res1),
            ))
        } else {
            Err(Error::from_go_error(res0))
        }
    }

    fn _prove(
        &self,
        compiled_circuit: &CompiledCircuit<C>,
        pk: &Groth16ProvingKey<C>,
        witness: &Witness,
    ) -> Result<Groth16Proof<C>> {
        let witness_bytes = serde_json::to_vec(witness)?;

        let res = ffi::groth16::prove(
            self.go_ref_id,
            compiled_circuit.go_inner_ref(),
            pk.go_inner_ref(),
            witness_bytes,
        );

        if res < 0 {
            Err(Error::from_go_error(res))
        } else {
            Ok(Groth16Proof::from_go_inner_ref(res))
        }
    }

    fn _verify(
        &self,
        vk: &Groth16VerifyingKey<C>,
        proof: &Groth16Proof<C>,
        public_witness: &PublicWitness,
    ) -> Result<bool> {
        let public_witness_bytes = serde_json::to_vec(public_witness)?;

        let res = ffi::groth16::verify(
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

/// Implementation of the [`Backend`] trait for Groth16 using Gnark.
///
/// This implementation bridges Rust's type system with Gnark's Go implementation,
/// providing compile-time curve selection and runtime proof operations.
impl<C> Backend for Groth16Backend<C>
where
    C: CurveId,
{
    type CircuitConstraint = CompiledCircuit<C>;
    type ProvingKey = Groth16ProvingKey<C>;
    type VerifyingKey = Groth16VerifyingKey<C>;
    type Proof = Groth16Proof<C>;

    type Error = Error;

    fn new() -> Self {
        Self::_new()
    }

    fn metadata(&self) -> MetadataInfo {
        MetadataInfo {
            field: C::field(),
            curve: C::curve_type(),
            proving_system: ProvingSystem::Groth16,
        }
    }

    fn compile(&self, circuit: &CircuitDefinition) -> Result<Self::CircuitConstraint> {
        self._compile(circuit)
    }

    fn setup(
        &self,
        compiled_circuit: &Self::CircuitConstraint,
    ) -> Result<(Self::ProvingKey, Self::VerifyingKey)> {
        self._setup(compiled_circuit)
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

#[cfg(test)]
mod tests {
    use super::*;
    use rsnark_core::{API, Circuit, CircuitDefine, CircuitWitness, curve::BN254};
    use rsnark_provers_core::Prover;

    #[derive(rsnark_core::Circuit)]
    pub struct TestCircuit {
        a: u32,
        b: u32,
        pub c: u32,
    }

    impl Circuit for CircuitDefine<TestCircuit> {
        fn define(&self, api: &mut impl API) {
            let c = api.add(&self.a, &self.b);
            api.assert_is_equal(&c, &self.c);
        }
    }

    #[test]
    fn test_groth16_with_core_prover() {
        let prover: Prover<Groth16Backend<BN254>> = Prover::new();

        let circuit_prover = prover.compile_circuit::<TestCircuit>().unwrap();

        let (pk, vk) = circuit_prover.setup().unwrap();

        let circuit_witness = TestCircuit {
            a: 3,
            b: 4,
            c: 7, // 3 + 4 = 7
        };

        let proof = circuit_prover.prove(&pk, &circuit_witness).unwrap();

        let public_witness = circuit_witness.into_public_witness();
        circuit_prover.verify(&vk, &proof, public_witness).unwrap();
    }
}
