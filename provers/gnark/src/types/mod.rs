//! Type definitions for Gnark backend operations.
//!
//! This module contains Rust type wrappers around Go-side Gnark objects.
//! These types manage references to compiled circuits, proving keys, and
//! verifying keys stored in the Go runtime, providing a safe Rust interface
//! for cryptographic operations.

mod groth16;
pub use groth16::*;

mod traits;
pub use traits::*;
