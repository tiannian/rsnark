//! # rSnark
//!
//! rSnark is a Rust library for writing zero-knowledge circuits and generating proofs.
//!
//! It provides a core library to write circuits and a provers library to generate proofs using
//! various backend implementations like Gnark.
//!
//! ## Writing Circuits
//!
//! Defining a circuit requires two simple steps:
//!
//! 1. Define the circuit's inputs and outputs using the `#[derive(Circuit)]` macro.
//! 2. Implement the `Circuit` trait to define the circuit's constraint rules.
//!
//! ```rust
//! use rsnark::{
//!     Groth16BN254GnarkProver,
//!     core::{API, Circuit, CircuitDefine, CircuitWitness},
//! };
//!
//! #[derive(Circuit)]
//! pub struct TestCircuit {
//!     a: u32,        // private input
//!     b: u32,        // private input  
//!     pub c: u32,    // public input
//! }
//!
//! impl Circuit for CircuitDefine<TestCircuit> {
//!     fn define(&self, api: &mut impl API) {
//!         let c = api.add(&self.a, &self.b);
//!         api.assert_is_equal(&c, &self.c);
//!     }
//! }
//! ```
//!
//! ### Circuit Macro
//!
//! The `#[derive(Circuit)]` macro treats Rust's visibility modifiers as indicators of whether
//! a field is a public input or private input, automatically generating the corresponding structures.
//!
//! - Fields without `pub` are treated as **private inputs**
//! - Fields with `pub` are treated as **public inputs**
//!
//! The macro also automatically generates a Public Witness structure that can be accessed
//! using the [`PublicWitness`](crate::core::PublicWitness).
//!
//! ### Defining Circuit Constraints
//!
//! Use the `Circuit` trait to define the circuit's constraint rules. The `define` method
//! receives an API object that provides various operations for building constraints.
//!
//! ```rust
//! use rsnark::core::{API, Circuit, CircuitDefine};
//!
//! #[derive(Circuit)]
//! pub struct MultiplyCircuit {
//!     x: u32,
//!     y: u32,
//!     pub result: u32,
//! }
//!
//! impl Circuit for CircuitDefine<MultiplyCircuit> {
//!     fn define(&self, api: &mut impl API) {
//!         let product = api.mul(&self.x, &self.y);
//!         api.assert_is_equal(&product, &self.result);
//!     }
//! }
//! ```
//!
//! ### Nested Circuits
//!
//! Circuits can be easily nested by embedding other circuits as fields in the struct.
//! Sub-circuits can also implement the Circuit trait, but you must manually call the
//! sub-circuit's `define` function within the parent circuit's `define` function.
//!
//! > Note: Private has higher priority. This means that if you set a sub-circuit (which contains public fields) as private,
//! > the public inputs of that sub-circuit will not be treated as public inputs in the parent circuit.
//! > However, if you set a sub-circuit as public, its private inputs will still be treated as private inputs.
//!
//! ```rust
//! use rsnark::core::{API, Circuit, CircuitDefine};
//!
//! #[derive(Circuit)]
//! pub struct AdderCircuit {
//!     a: u32,
//!     b: u32,
//!     pub sum: u32,
//! }
//!
//! impl Circuit for CircuitDefine<AdderCircuit> {
//!     fn define(&self, api: &mut impl API) {
//!         let result = api.add(&self.a, &self.b);
//!         api.assert_is_equal(&result, &self.sum);
//!     }
//! }
//!
//! #[derive(Circuit)]
//! pub struct MultiplierCircuit {
//!     x: u32,
//!     y: u32,
//!     pub product: u32,
//! }
//!
//! impl Circuit for CircuitDefine<MultiplierCircuit> {
//!     fn define(&self, api: &mut impl API) {
//!         let result = api.mul(&self.x, &self.y);
//!         api.assert_is_equal(&result, &self.product);
//!     }
//! }
//!
//! #[derive(Circuit)]
//! pub struct CompositeCircuit {
//!     adder: AdderCircuit,
//!     multiplier: MultiplierCircuit,
//!     pub final_result: u32,
//! }
//!
//! impl Circuit for CircuitDefine<CompositeCircuit> {
//!     fn define(&self, api: &mut impl API) {
//!         // Execute sub-circuits
//!         self.adder.define(api);
//!         self.multiplier.define(api);
//!
//!         // Main circuit logic
//!         let final_sum = api.add(&self.adder.sum, &self.multiplier.product);
//!         api.assert_is_equal(&final_sum, &self.final_result);
//!     }
//! }
//! ```
//!
//! ## Backend Triple
//!
//! Similar to compiler target triples, rSnark uses backend triples to define which backend,
//! curve, and proving system to use. The format is: `{proving_system}-{curve}-{backend}`.
//!
//! Currently supported backend triples:
//!
//! - [`groth16-bn254-gnark`](Groth16BN254GnarkProver) - Groth16 with BN254 curve using Gnark backend
//! - [`groth16-bls12-381-gnark`](Groth16BLS12_381GnarkProver) - Groth16 with BLS12-381 curve
//! - [`groth16-bls24-317-gnark`](Groth16BLS24_317GnarkProver) - Groth16 with BLS24-317 curve  
//! - [`groth16-bls12-377-gnark`](Groth16BLS12_377GnarkProver) - Groth16 with BLS12-377 curve
//! - [`groth16-bw6-761-gnark`](Groth16BW6_761GnarkProver) - Groth16 with BW6-761 curve
//! - [`groth16-bls24-315-gnark`](Groth16BLS24_315GnarkProver) - Groth16 with BLS24-315 curve
//! - [`groth16-bw6-633-gnark`](Groth16BW6_633GnarkProver) - Groth16 with BW6-633 curve
//!

#[doc(inline)]
pub use rsnark_core as core;

/// Prover to generate
pub mod provers {
    #[doc(inline)]
    pub use rsnark_provers_core::*;

    /// Prover provider by gnark.
    #[doc(inline)]
    pub use rsnark_provers_gnark as gnark;

    /// Mock Prover.
    #[doc(inline)]
    pub use rsnark_provers_mock as mock;
}

/// Provers with backend triple `groth16-bn254-gnark`
pub type Groth16BN254GnarkProver =
    provers::Prover<provers::gnark::Groth16Backend<provers::curve::BN254>>;

/// Provers with backend triple `groth16-bls12-381-gnark`
pub type Groth16BLS12_381GnarkProver =
    provers::Prover<provers::gnark::Groth16Backend<provers::curve::BLS12_381>>;

/// Provers with backend triple `groth16-bls24-317-gnark`
pub type Groth16BLS24_317GnarkProver =
    provers::Prover<provers::gnark::Groth16Backend<provers::curve::BLS24_317>>;

/// Provers with backend triple `groth16-bls12-377-gnark`
pub type Groth16BLS12_377GnarkProver =
    provers::Prover<provers::gnark::Groth16Backend<provers::curve::BLS12_377>>;

/// Provers with backend triple `groth16-bw6-761-gnark`
pub type Groth16BW6_761GnarkProver =
    provers::Prover<provers::gnark::Groth16Backend<provers::curve::BW6_761>>;

/// Provers with backend triple `groth16-bls24-315-gnark`
pub type Groth16BLS24_315GnarkProver =
    provers::Prover<provers::gnark::Groth16Backend<provers::curve::BLS24_315>>;

/// Provers with backend triple `groth16-bw6-633-gnark`
pub type Groth16BW6_633GnarkProver =
    provers::Prover<provers::gnark::Groth16Backend<provers::curve::BW6_633>>;
