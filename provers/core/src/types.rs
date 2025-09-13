/// Trait for identifying elliptic curves used in zero-knowledge proof systems.
///
/// This trait provides a way to statically identify different elliptic curves
/// through unique numeric identifiers. Each curve type implements this trait
/// to return its specific curve ID, enabling type-safe curve selection and
/// backend specialization.
///
/// Curve types are typically used as type parameters to specify which elliptic
/// curve a particular backend or proof system should use.
///
pub trait CurveId {
    /// Returns the unique numeric identifier for this curve.
    ///
    /// Each curve implementation must return a unique ID that distinguishes
    /// it from all other supported curves.
    fn curve_id() -> u64;
}

macro_rules! define_curve {
    ($name:ident, $id:expr) => {
        pub struct $name;

        impl CurveId for $name {
            fn curve_id() -> u64 {
                $id
            }
        }
    };
}

/// Predefined elliptic curve types for common ZK-SNARK applications.
///
/// This module contains type definitions for widely-used elliptic curves in
/// zero-knowledge proof systems. Each curve is defined with a unique identifier
/// and can be used as a type parameter to specify which curve a backend should use.
///
/// # Available Curves
///
/// - [`curve::BN254`]
/// - [`curve::BLS12_381`]
/// - [`curve::BLS12_377`]
/// - [`curve::BLS24_317`]
/// - [`curve::BLS24_315`]
/// - [`curve::BW6_761`]
/// - [`curve::BW6_633`]
///
pub mod curve {
    use super::*;

    define_curve!(BN254, 1);
    define_curve!(BLS12_381, 2);
    define_curve!(BLS24_317, 3);

    define_curve!(BLS12_377, 4);
    define_curve!(BW6_761, 5);
    define_curve!(BLS24_315, 6);
    define_curve!(BW6_633, 7);
}
