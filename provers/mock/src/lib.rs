//! # rsnark-provers-mock
//!
//! Mock backend implementation for testing and development purposes.
//!
//! This crate provides a simple, non-cryptographic backend that can be used
//! for testing circuit compilation and proof generation workflows without
//! the computational overhead of real zero-knowledge proofs. The mock backend
//! uses simple hashing to simulate proof generation and verification.
//!
//! ## Features
//!
//! - **Fast execution**: No complex cryptographic operations
//! - **Deterministic behavior**: Proofs are based on SHA3 hashing
//! - **Simple setup**: No key generation required
//! - **Testing friendly**: Ideal for unit tests and development
//!
//! ## Limitations
//!
//! This mock implementation provides **no cryptographic security** and should
//! never be used in production environments. It's designed solely for testing
//! and development purposes.

use rsnark_core::{
    CurveType, MetadataInfo, ProvingSystem, U256,
    types::{CircuitDefinition, PublicWitness, Witness},
};
use rsnark_provers_core::Backend;
use sha3::{Digest, Sha3_256};

/// Error type for mock backend operations.
///
/// Currently, the mock backend implementation does not produce any errors,
/// but this enum is provided for future extensibility and API consistency.
#[derive(Debug, thiserror::Error)]
pub enum Error {}

/// Mock backend implementation for testing and development.
///
/// This backend provides a simple, non-cryptographic implementation of the
/// [`Backend`] trait that can be used for testing circuit compilation and
/// proof workflows. Instead of generating real zero-knowledge proofs, it
/// creates deterministic "proofs" based on SHA3 hashing of public inputs.
///
/// # Security Warning
///
/// This implementation provides **no cryptographic security** and should
/// never be used in production. It's designed only for testing and development.
#[derive(Clone)]
pub struct MockProverBackend;

impl Backend for MockProverBackend {
    type CircuitConstraint = CircuitDefinition;
    type ProvingKey = ();
    type VerifyingKey = ();
    type Proof = U256;

    type Error = Error;

    fn new() -> Self {
        Self
    }

    fn metadata(&self) -> MetadataInfo {
        MetadataInfo {
            field: U256::MAX,
            curve: CurveType::Mock,
            proving_system: ProvingSystem::Mock,
        }
    }

    /// Returns a copy of the circuit definition as the "compiled" constraint.
    ///
    /// In the mock implementation, no actual compilation is performed.
    /// The circuit definition is simply cloned and returned.
    fn compile(&self, circuit: &CircuitDefinition) -> Result<Self::CircuitConstraint, Self::Error> {
        Ok(circuit.clone())
    }

    /// Returns unit values as mock proving and verifying keys.
    ///
    /// The mock backend doesn't require actual key generation, so it
    /// returns empty unit values for both keys.
    fn setup(
        &self,
        _cs: &Self::CircuitConstraint,
    ) -> Result<(Self::ProvingKey, Self::VerifyingKey), Self::Error> {
        Ok(((), ()))
    }

    /// Generates a mock proof by hashing the public witness.
    ///
    /// The mock proof is simply the SHA3-256 hash of the public witness values.
    /// This provides deterministic behavior for testing while avoiding complex
    /// cryptographic operations.
    fn prove(
        &self,
        _cs: &Self::CircuitConstraint,
        _pk: &Self::ProvingKey,
        witness: &Witness,
    ) -> Result<Self::Proof, Self::Error> {
        let res_hash = hash_public_witness(witness.public());
        Ok(res_hash)
    }

    /// Verifies a mock proof by comparing hashes.
    ///
    /// Verification simply checks if the proof hash matches the hash of the
    /// provided public witness. Returns `true` if they match, `false` otherwise.
    fn verify(
        &self,
        _vk: &Self::VerifyingKey,
        proof: &Self::Proof,
        public_witness: &PublicWitness,
    ) -> Result<bool, Self::Error> {
        let res_hash = hash_public_witness(&public_witness.public);
        if res_hash == *proof {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

fn hash_public_witness(public_witness: &[U256]) -> U256 {
    let mut hasher = Sha3_256::new();

    for value in public_witness {
        let bytes: [u8; 32] = value.to_be_bytes();
        hasher.update(bytes);
    }

    let res_bytes = hasher.finalize();
    U256::from_be_slice(&res_bytes)
}
