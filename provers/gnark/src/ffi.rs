pub mod binding {
    #![allow(warnings)]
    rust2go::r2g_include_binding!();
}

#[rust2go::r2g]
pub trait Groth16Prover {
    fn create(curve_id: u64) -> u64;

    fn curve_id(prover: u64) -> u64;

    fn compile(prover: u64, circuit: Vec<u8>) -> i64;

    fn setup(prover: u64, compiled_circuit: i64) -> Vec<u8>;

    fn prove(prover: u64, compiled_circuit: i64, pk: i64, witness: Vec<u8>) -> Vec<u8>;

    fn verify(prover: u64, vk: i64, proof: Vec<u8>, public_witness: Vec<u8>) -> i64;
}

#[rust2go::r2g]
pub trait Object {
    fn serialize(object_id: i64) -> Vec<u8>;

    fn deserialize(ty: u64, curve_id: u64, data: Vec<u8>) -> i64;

    fn write_to_file(object_id: i64, path: String) -> i64;

    fn read_from_file(ty: u64, curve_id: u64, path: String) -> i64;
}
