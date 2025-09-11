//! # rsnark-provers-gnark
//!
//! Gnark-based backend implementation for zero-knowledge proof systems.
//!
//! This crate provides a Rust interface to the Gnark library (implemented in Go)
//! for generating and verifying zero-knowledge proofs using the Groth16 protocol.
//! It bridges the gap between Rust applications and Gnark's high-performance
//! cryptographic operations through FFI bindings.
//!
//! ## Key Components
//!
//! - [`Groth16Backend`]: Backend implementation using Gnark's Groth16 prover
//! - [`Error`]: Error types for Gnark operations and Go FFI interactions
//! - [`types`]: Type definitions for compiled circuits, proving keys, and verifying keys
//!
//! ## Features
//!
//! - **High Performance**: Leverages Gnark's optimized cryptographic implementations
//! - **Multi-Curve Support**: Supports various elliptic curves through curve type parameters
//! - **FFI Integration**: Seamless integration with Go-based Gnark library
//! - **Memory Management**: Automatic cleanup of Go-side resources
//!
//! ## Requirements
//!
//! This crate requires the Gnark Go library to be built and available. The build
//! process is handled automatically through the `build.rs` script using `rust2go`.

mod ffi;

pub mod types;

mod error;
pub use error::*;

mod groth16;
pub use groth16::*;
