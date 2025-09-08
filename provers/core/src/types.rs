use rsnark_core::U256;

pub struct Proof(pub Vec<U256>);

#[derive(Debug, Clone)]
pub enum Curve {
    BN254,
    BLS12_381,
}
