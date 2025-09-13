use num::{BigInt, Num};

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

    fn curve_type() -> CurveType;

    fn field() -> BigInt;
}

macro_rules! define_curve {
    ($name:ident, $id:expr, $field:expr) => {
        pub struct $name;

        impl CurveId for $name {
            fn curve_id() -> u64 {
                $id
            }

            fn curve_type() -> CurveType {
                CurveType::$name
            }

            fn field() -> BigInt {
                BigInt::from_str_radix($field, 10).unwrap()
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

    define_curve!(
        BN254,
        1,
        "21888242871839275222246405745257275088548364400416034343698204186575808495617"
    );
    define_curve!(
        BLS12_381,
        2,
        "52435875175126190479447740508185965837690552500527637822603658699938581184513"
    );
    define_curve!(
        BLS24_317,
        3,
        "30869589236456844204538189757527902584594726589286811523515204428962673459201"
    );

    define_curve!(
        BLS12_377,
        4,
        "8444461749428370424248824938781546531375899335154063827935233455917409239041"
    );
    define_curve!(
        BW6_761,
        5,
        "258664426012969094010652733694893533536393512754914660539884262666720468348340822774968888139573360124440321458177"
    );
    define_curve!(
        BLS24_315,
        6,
        "11502027791375260645628074404575422495959608200132055716665986169834464870401"
    );
    define_curve!(
        BW6_633,
        7,
        "39705142709513438335025689890408969744933502416914749335064285505637884093126342347073617133569"
    );
}

#[derive(Debug, Clone)]
pub enum CurveType {
    Mock,
    BN254,
    BLS12_381,
    BLS24_317,
    BLS12_377,
    BW6_761,
    BLS24_315,
    BW6_633,
}

#[derive(Debug, Clone)]
pub enum ProvingSystem {
    Mock,
    Groth16,
    Plonk,
}

#[derive(Debug, Clone)]
pub struct MetadataInfo {
    pub field: BigInt,
    pub curve: CurveType,
    pub proving_system: ProvingSystem,
}

pub trait Metadata {
    fn field(&self) -> &BigInt;

    fn curve(&self) -> &CurveType;

    fn proving_system(&self) -> &ProvingSystem;
}

impl Metadata for MetadataInfo {
    fn field(&self) -> &BigInt {
        &self.field
    }

    fn curve(&self) -> &CurveType {
        &self.curve
    }

    fn proving_system(&self) -> &ProvingSystem {
        &self.proving_system
    }
}
