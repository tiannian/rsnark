//! # rsnark-provers-core
//!
//! Core abstractions and traits for zero-knowledge proof systems in the rsnark ecosystem.
//!
//! This crate provides the fundamental building blocks for implementing various ZK-SNARK backends
//! and proof systems. It defines generic interfaces that can be implemented by different
//! cryptographic backends (such as Groth16, PLONK, etc.) while maintaining a consistent API.
//!
//! ## Key Components
//!
//! - [`Backend`]: The core trait that defines the interface for ZK-SNARK backends
//! - [`Prover`]: High-level prover that orchestrates the proof generation process
//! - [`CircuitProver`]: Circuit-specific prover for generating and verifying proofs
//!

mod backend;
pub use backend::*;

mod prover;
pub use prover::*;

mod circuit_prover;
pub use circuit_prover::*;
