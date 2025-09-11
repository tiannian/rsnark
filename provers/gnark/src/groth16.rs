use rsnark_core::{
    U256,
    types::{CircuitDefinition, PublicWitness, Witness},
};
use rsnark_provers_core::{Backend, Curve, Proof};

use crate::{
    Error, Result,
    ffi::{self, Groth16Prover},
    types::{CompiledCircuit, CurveType, GoInnerRef, Groth16ProvingKey, Groth16VerifyingKey},
};

#[derive(Clone)]
pub struct Groth16Backend(u64);

impl Groth16Backend {
    fn _new(curve: CurveType) -> Self {
        let prover = ffi::Groth16ProverImpl::create(curve.to_curve_id());

        Self(prover)
    }

    pub fn curve_id(&self) -> CurveType {
        let curve = ffi::Groth16ProverImpl::curve_id(self.0);

        CurveType::from_curve_id(curve)
    }

    fn _compile(&self, circuit: &CircuitDefinition) -> Result<CompiledCircuit> {
        let circuit = serde_json::to_vec(circuit)?;

        let res = ffi::Groth16ProverImpl::compile(self.0, circuit);

        if res >= 0 {
            Ok(CompiledCircuit::from_go_inner_ref(res))
        } else {
            Err(Error::from_go_error(res))
        }
    }

    fn _setup(
        &self,
        compiled_circuit: &CompiledCircuit,
    ) -> Result<(Groth16ProvingKey, Groth16VerifyingKey)> {
        let res = ffi::Groth16ProverImpl::setup(self.0, compiled_circuit.go_inner_ref());

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
        compiled_circuit: &CompiledCircuit,
        pk: &Groth16ProvingKey,
        witness: &Witness,
    ) -> Result<Vec<u8>> {
        // TODO: Compect go side witness
        let witness_bytes = serde_json::to_vec(witness)?;

        let res = ffi::Groth16ProverImpl::prove(
            self.0,
            compiled_circuit.go_inner_ref(),
            pk.go_inner_ref(),
            witness_bytes,
        );

        let code = i64::from_be_bytes(res[0..8].try_into().unwrap());

        if code != 0 {
            Err(Error::from_go_error(code))
        } else {
            Ok(res)
        }
    }

    fn _verify(
        &self,
        vk: &Groth16VerifyingKey,
        proof: Vec<u8>,
        public_witness: &PublicWitness,
    ) -> Result<bool> {
        // TODO: Compect go side public witness
        let public_witness_bytes = serde_json::to_vec(public_witness)?;

        let res =
            ffi::Groth16ProverImpl::verify(self.0, vk.go_inner_ref(), proof, public_witness_bytes);

        if res == 0 {
            Ok(true)
        } else if res == -20010 {
            Ok(false)
        } else {
            Err(Error::from_go_error(res))
        }
    }
}

impl Backend for Groth16Backend {
    type CircuitConstraint = CompiledCircuit;
    type ProvingKey = Groth16ProvingKey;
    type VerifyingKey = Groth16VerifyingKey;

    type Error = Error;

    fn new(curve: Curve) -> Self {
        Self::_new(curve.into())
    }

    fn compile(&self, circuit: &CircuitDefinition) -> Result<CompiledCircuit> {
        self._compile(circuit)
    }

    fn setup(
        &self,
        compiled_circuit: &CompiledCircuit,
    ) -> Result<(Groth16ProvingKey, Groth16VerifyingKey)> {
        self._setup(compiled_circuit)
    }

    fn prove(
        &self,
        compiled_circuit: &CompiledCircuit,
        pk: &Groth16ProvingKey,
        witness: &Witness,
    ) -> Result<Proof> {
        let proof = self._prove(compiled_circuit, pk, witness)?;

        let proof_len = proof.len() - 8;

        if proof_len < 256 {
            return Err(Error::ProofLengthWrong);
        }

        let proof = &proof[8..];

        let mut res = Vec::with_capacity(8);

        for i in 0..8 {
            let mut bytes = [0u8; 32];
            bytes.copy_from_slice(&proof[i * 32..(i + 1) * 32]);
            res.push(U256::from_be_bytes(bytes));
        }

        Ok(Proof(res))
    }

    fn verify(
        &self,
        vk: &Groth16VerifyingKey,
        proof: &Proof,
        public_witness: &PublicWitness,
    ) -> Result<bool> {
        let proof_len = proof.0.len() * 32 + 68;

        let mut proof_bytes = Vec::with_capacity(proof_len);
        for value in &proof.0 {
            let bytes: [u8; 32] = value.to_be_bytes();
            proof_bytes.extend_from_slice(&bytes);
        }

        proof_bytes.resize(proof_len, 0u8);

        self._verify(vk, proof_bytes, public_witness)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rsnark_core::{API, Circuit, CircuitDefine, CircuitWitness};
    use rsnark_provers_core::{Curve, Prover};

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
        let prover: Prover<Groth16Backend> = Prover::new(Curve::BN254);

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
