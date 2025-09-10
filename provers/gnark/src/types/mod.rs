mod groth16;
pub use groth16::*;

mod traits;
use rsnark_provers_core::Curve;
pub use traits::*;

pub enum CurveType {
    BN254,
    BLS12_381,
    BLS12_377,
    BW6_761,
}

impl From<Curve> for CurveType {
    fn from(curve: Curve) -> Self {
        match curve {
            Curve::BN254 => CurveType::BN254,
            Curve::BLS12_381 => CurveType::BLS12_381,
        }
    }
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

    pub(crate) fn from_curve_id(curve_id: u64) -> Self {
        match curve_id {
            1 => CurveType::BN254,
            2 => CurveType::BLS12_381,
            3 => CurveType::BLS12_377,
            4 => CurveType::BW6_761,
            _ => unreachable!(),
        }
    }
}
