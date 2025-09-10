mod groth16;
pub use groth16::*;

mod traits;
pub use traits::*;

pub enum CurveType {
    BN254,
    BLS12_381,
    BLS12_377,
    BW6_761,
}

impl CurveType {
    pub(crate) fn to_curve_id(&self) -> u64 {
        match self {
            CurveType::BN254 => 1,
            CurveType::BLS12_381 => 2,
            CurveType::BLS12_377 => 3,
            CurveType::BW6_761 => 4,
        }
    }
}
