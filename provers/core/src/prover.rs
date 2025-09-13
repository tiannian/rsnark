use std::marker::PhantomData;

use anyhow::Result;
use rsnark_core::{Circuit, CircuitBuilder, CircuitWitness};

use crate::{Backend, CircuitProver};

/// High-level prover that orchestrates the zero-knowledge proof generation process.
///
/// The `Prover` serves as the main entry point for creating zero-knowledge proofs using
/// a specific backend implementation. It handles the circuit compilation process and
/// creates circuit-specific provers for actual proof generation.
///
/// # Type Parameters
///
/// * `B` - The backend implementation that defines the underlying cryptographic operations
///
pub struct Prover<B> {
    pub(crate) backend: B,
}

impl<B: Backend> Default for Prover<B> {
    fn default() -> Self {
        Self::new()
    }
}

impl<B> Prover<B>
where
    B: Backend,
{
    /// Creates a new prover instance with the specified backend.
    ///
    /// This initializes the backend and prepares the prover for circuit compilation.
    /// The backend is created using its [`Backend::new()`] method.
    ///
    /// # Returns
    ///
    /// A new `Prover` instance ready for circuit compilation.
    ///
    pub fn new() -> Self {
        let backend = B::new();

        Self { backend }
    }

    /// Compiles a circuit and creates a circuit-specific prover.
    ///
    /// This method takes a circuit witness type and compiles it into a form suitable
    /// for the backend. It creates the circuit definition by instantiating the circuit
    /// with fresh variables and then compiling it using the backend.
    ///
    /// # Type Parameters
    ///
    /// * `C` - The circuit witness type that implements [`CircuitWitness`]
    ///
    /// # Returns
    ///
    /// Returns a [`CircuitProver`] instance that can be used for setup, proving, and verification
    /// operations on the compiled circuit.
    ///
    /// # Errors
    ///
    /// This function may return an error if:
    /// - Circuit compilation fails due to invalid constraints
    /// - The backend cannot represent the circuit
    /// - Internal compilation errors occur
    ///
    pub fn compile_circuit<C>(self) -> Result<CircuitProver<B, C>>
    where
        C: CircuitWitness,
        C::PublicElement: Circuit,
    {
        let metadata = self.backend.metadata();
        let mut builder = CircuitBuilder::new(metadata);
        let circuit = C::create_public(builder.variable_initer_mut(), false);
        circuit.define(&mut builder);

        let define = builder.build();

        let cs = self.backend.compile(&define)?;

        Ok(CircuitProver {
            backend: self.backend,
            constraint: cs,
            marker: PhantomData,
        })
    }

    // pub fn compile_constraints(&self, constraints: &[u8]) -> B::CircuitConstraint {
}
