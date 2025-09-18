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

/// Enumeration of supported elliptic curve types.
///
/// This enum represents all the elliptic curves that can be used with rSnark.
/// Each curve type corresponds to a specific elliptic curve with unique mathematical
/// properties and field characteristics.
///
/// # Curve Categories
///
/// ## Production Curves
/// - [`BN254`](CurveType::BN254): Barreto-Naehrig curve with 254-bit prime, widely used in Ethereum
/// - [`BLS12_381`](CurveType::BLS12_381): BLS12 curve with 381-bit prime, used in Ethereum 2.0
/// - [`BLS12_377`](CurveType::BLS12_377): BLS12 curve with 377-bit prime
/// - [`BLS24_317`](CurveType::BLS24_317): BLS24 curve with 317-bit prime
/// - [`BLS24_315`](CurveType::BLS24_315): BLS24 curve with 315-bit prime
/// - [`BW6_761`](CurveType::BW6_761): BW6 curve with 761-bit prime
/// - [`BW6_633`](CurveType::BW6_633): BW6 curve with 633-bit prime
///
/// ## Testing Curves
/// - [`Mock`](CurveType::Mock): Mock curve for testing purposes
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

/// Enumeration of supported zero-knowledge proving systems.
///
/// This enum represents the different proving systems that can be used to generate
/// and verify zero-knowledge proofs. Each proving system has different characteristics
/// in terms of proof size, verification time, and trusted setup requirements.
///
/// # Proving Systems
///
/// ## Production Systems
/// - [`Groth16`](ProvingSystem::Groth16): Efficient zk-SNARK with constant-size proofs and fast verification
/// - [`Plonk`](ProvingSystem::Plonk): Universal zk-SNARK with universal trusted setup
///
/// ## Testing Systems
/// - [`Mock`](ProvingSystem::Mock): Mock proving system for testing without cryptographic operations
///
#[derive(Debug, Clone)]
pub enum ProvingSystem {
    Mock,
    Groth16,
    Plonk,
}

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct MetadataInfo {
    pub field: BigInt,
    pub curve: CurveType,
    pub proving_system: ProvingSystem,
}

/// Trait for accessing metadata information from proof system configurations.
///
/// This trait provides a standardized interface for retrieving metadata from
/// various backend configurations. It enables generic code to inspect the
/// properties of different proving systems without knowing their specific types.
///
/// # Methods
///
/// - [`field`](Metadata::field): Returns the prime field modulus
/// - [`curve`](Metadata::curve): Returns the elliptic curve type
/// - [`proving_system`](Metadata::proving_system): Returns the proving system type
///
/// # Usage
///
/// This trait is typically implemented by backend types to provide introspection
/// capabilities and enable metadata-aware generic programming.
///
/// ```rust,ignore
/// fn inspect_backend<T: Metadata>(backend: &T) {
///     println!("Curve: {:?}", backend.curve());
///     println!("Proving system: {:?}", backend.proving_system());
///     println!("Field size: {} bits", backend.field().bits());
/// }
/// ```
pub trait Metadata {
    /// Returns a reference to the prime field modulus used by this configuration.
    fn field(&self) -> &BigInt;

    /// Returns a reference to the elliptic curve type used by this configuration.
    fn curve(&self) -> &CurveType;

    /// Returns a reference to the proving system type used by this configuration.
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
