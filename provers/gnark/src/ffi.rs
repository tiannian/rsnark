pub mod binding {
    #![allow(warnings)]
    rust2go::r2g_include_binding!();
}

#[cfg(not(docsrs))]
#[rust2go::r2g]
pub trait Groth16Prover {
    fn groth16_create(curve_id: u64) -> u64;

    fn groth16_compile(curve_id: u64, circuit: Vec<u8>) -> i64;

    fn groth16_setup(prover: u64, compiled_circuit: i64) -> Vec<u8>;

    fn groth16_prove(prover: u64, compiled_circuit: i64, pk: i64, witness: Vec<u8>) -> i64;

    fn groth16_verify(prover: u64, vk: i64, proof: i64, public_witness: Vec<u8>) -> i64;

    fn groth16_remove_prover(prover: u64);
}

#[cfg(not(docsrs))]
#[rust2go::r2g]
pub trait PlonkProver {
    fn plonk_create(curve_id: u64) -> u64;

    fn plonk_compile(curve_id: u64, circuit: Vec<u8>) -> i64;

    fn plonk_setup(prover: u64, compiled_circuit: i64) -> Vec<u8>;

    fn plonk_prove(prover: u64, compiled_circuit: i64, pk: i64, witness: Vec<u8>) -> i64;

    fn plonk_verify(prover: u64, vk: i64, proof: i64, public_witness: Vec<u8>) -> i64;

    fn plonk_remove_prover(prover: u64);
}

#[cfg(not(docsrs))]
#[rust2go::r2g]
pub trait Object {
    fn serialize(object_id: i64) -> Vec<u8>;

    fn deserialize(ty: u64, curve_id: u64, data: Vec<u8>) -> i64;

    fn write_to_file(object_id: i64, path: String) -> i64;

    fn read_from_file(ty: u64, curve_id: u64, path: String) -> i64;

    fn remove_object(object_id: i64);

    fn export_solidity(object_id: i64, type_id: u64) -> Vec<u8>;
}

#[cfg(docsrs)]
pub mod groth16 {
    pub fn create(_curve_id: u64) -> u64 {
        unimplemented!()
    }

    pub fn compile(_curve_id: u64, _circuit: Vec<u8>) -> i64 {
        unimplemented!()
    }

    pub fn setup(_prover: u64, _compiled_circuit: i64) -> Vec<u8> {
        unimplemented!()
    }

    pub fn prove(_prover: u64, _compiled_circuit: i64, _pk: i64, _witness: Vec<u8>) -> i64 {
        unimplemented!()
    }

    pub fn verify(_prover: u64, _vk: i64, _proof: i64, _public_witness: Vec<u8>) -> i64 {
        unimplemented!()
    }

    pub fn remove_prover(_prover: u64) {
        unimplemented!()
    }
}

#[cfg(docsrs)]
pub mod object {
    pub fn serialize(_object_id: i64) -> Vec<u8> {
        unimplemented!()
    }

    pub fn deserialize(_ty: u64, _curve_id: u64, _data: Vec<u8>) -> i64 {
        unimplemented!()
    }

    pub fn write_to_file(_object_id: i64, _path: String) -> i64 {
        unimplemented!()
    }

    pub fn read_from_file(_ty: u64, _curve_id: u64, _path: String) -> i64 {
        unimplemented!()
    }

    pub fn export_solidity(_object_id: i64, _type_id: u64) -> Vec<u8> {
        unimplemented!()
    }

    pub fn remove_object(_object_id: i64) {
        unimplemented!()
    }
}

#[cfg(docsrs)]
pub mod plonk {
    pub fn create(_curve_id: u64) -> u64 {
        unimplemented!()
    }

    pub fn compile(_curve_id: u64, _circuit: Vec<u8>) -> i64 {
        unimplemented!()
    }

    pub fn setup(_prover: u64, _compiled_circuit: i64) -> Vec<u8> {
        unimplemented!()
    }

    pub fn prove(_prover: u64, _compiled_circuit: i64, _pk: i64, _witness: Vec<u8>) -> i64 {
        unimplemented!()
    }

    pub fn verify(_prover: u64, _vk: i64, _proof: i64, _public_witness: Vec<u8>) -> i64 {
        unimplemented!()
    }

    pub fn remove_prover(_prover: u64) {
        unimplemented!()
    }
}

#[cfg(not(docsrs))]
pub mod groth16 {
    use crate::ffi::Groth16Prover;

    pub fn create(curve_id: u64) -> u64 {
        super::Groth16ProverImpl::groth16_create(curve_id)
    }

    pub fn compile(curve_id: u64, circuit: Vec<u8>) -> i64 {
        super::Groth16ProverImpl::groth16_compile(curve_id, circuit)
    }

    pub fn setup(prover: u64, compiled_circuit: i64) -> Vec<u8> {
        super::Groth16ProverImpl::groth16_setup(prover, compiled_circuit)
    }

    pub fn prove(prover: u64, compiled_circuit: i64, pk: i64, witness: Vec<u8>) -> i64 {
        super::Groth16ProverImpl::groth16_prove(prover, compiled_circuit, pk, witness)
    }

    pub fn verify(prover: u64, vk: i64, proof: i64, public_witness: Vec<u8>) -> i64 {
        super::Groth16ProverImpl::groth16_verify(prover, vk, proof, public_witness)
    }

    pub fn remove_prover(prover: u64) {
        super::Groth16ProverImpl::groth16_remove_prover(prover)
    }
}

#[cfg(not(docsrs))]
pub mod object {
    use crate::ffi::Object;

    pub fn serialize(object_id: i64) -> Vec<u8> {
        super::ObjectImpl::serialize(object_id)
    }

    pub fn deserialize(ty: u64, curve_id: u64, data: Vec<u8>) -> i64 {
        super::ObjectImpl::deserialize(ty, curve_id, data)
    }

    pub fn write_to_file(object_id: i64, path: String) -> i64 {
        super::ObjectImpl::write_to_file(object_id, path)
    }

    pub fn read_from_file(ty: u64, curve_id: u64, path: String) -> i64 {
        super::ObjectImpl::read_from_file(ty, curve_id, path)
    }

    pub fn export_solidity(object_id: i64, type_id: u64) -> Vec<u8> {
        super::ObjectImpl::export_solidity(object_id, type_id)
    }

    pub fn remove_object(object_id: i64) {
        super::ObjectImpl::remove_object(object_id)
    }
}

#[cfg(not(docsrs))]
pub mod plonk {
    use crate::ffi::PlonkProver;

    pub fn create(curve_id: u64) -> u64 {
        super::PlonkProverImpl::plonk_create(curve_id)
    }

    pub fn compile(curve_id: u64, circuit: Vec<u8>) -> i64 {
        super::PlonkProverImpl::plonk_compile(curve_id, circuit)
    }

    pub fn setup(prover: u64, compiled_circuit: i64) -> Vec<u8> {
        super::PlonkProverImpl::plonk_setup(prover, compiled_circuit)
    }

    pub fn prove(prover: u64, compiled_circuit: i64, pk: i64, witness: Vec<u8>) -> i64 {
        super::PlonkProverImpl::plonk_prove(prover, compiled_circuit, pk, witness)
    }

    pub fn verify(prover: u64, vk: i64, proof: i64, public_witness: Vec<u8>) -> i64 {
        super::PlonkProverImpl::plonk_verify(prover, vk, proof, public_witness)
    }

    pub fn remove_prover(prover: u64) {
        super::PlonkProverImpl::plonk_remove_prover(prover)
    }
}
