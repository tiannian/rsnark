use rsnark_core::types::{CircuitDefinition, Witness};

use crate::{
    Error, Result,
    ffi::{self, Groth16Prover},
    types::{CompiledCircuit, CurveType, GoInnerRef, Groth16ProvingKey, Groth16VerifyingKey},
};

pub struct Groth16Backend(u64);

impl Groth16Backend {
    pub fn new(curve: CurveType) -> Self {
        let prover = ffi::Groth16ProverImpl::new(curve.to_curve_id());

        Self(prover)
    }

    pub fn curve_id(&self) -> CurveType {
        let curve = ffi::Groth16ProverImpl::curve_id(self.0);

        CurveType::from_curve_id(curve)
    }

    pub fn compile(&self, circuit: &CircuitDefinition) -> Result<CompiledCircuit> {
        let circuit = serde_json::to_vec(circuit)?;

        let res = ffi::Groth16ProverImpl::compile(self.0, circuit);

        if res >= 0 {
            Ok(CompiledCircuit::from_go_inner_ref(res))
        } else {
            Err(Error::from_go_error(res))
        }
    }

    pub fn setup(
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

    pub fn prove(
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
            Ok(res[8..].to_vec())
        }
    }

    pub fn verify(
        &self,
        vk: &Groth16VerifyingKey,
        proof: Vec<u8>,
        public_witness: &Witness,
    ) -> Result<bool> {
        // TODO: Compect go side public witness
        let public_witness_bytes = serde_json::to_vec(public_witness)?;

        let res =
            ffi::Groth16ProverImpl::verify(self.0, vk.go_inner_ref(), proof, public_witness_bytes);

        if res == 0 {
            Ok(true)
        } else {
            Err(Error::from_go_error(res))
        }
    }
}
