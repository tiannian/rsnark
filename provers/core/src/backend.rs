use rsnark_core::{
    MetadataInfo,
    types::{CircuitDefinition, PublicWitness, Witness},
};

/// Core trait defining the interface for zero-knowledge proof backends.
///
/// This trait abstracts over different ZK-SNARK implementations (such as Groth16, PLONK, Marlin, etc.)
/// and provides a unified interface for circuit compilation, trusted setup, proof generation, and verification.
///
/// ## Type Parameters
///
/// The trait defines several associated types that must be implemented by concrete backends:
/// - `CircuitConstraint`: The internal representation of compiled circuit constraints
/// - `ProvingKey`: The proving key generated during the trusted setup phase
/// - `VerifyingKey`: The verifying key generated during the trusted setup phase  
/// - `Error`: The error type for backend-specific operations
///
/// ## Workflow
///
/// The typical workflow when using a backend implementation:
/// 1. Create a new backend instance with [`Backend::new()`]
/// 2. Compile the circuit definition with [`Backend::compile()`]
/// 3. Perform trusted setup with [`Backend::setup()`] to generate keys
/// 4. Generate proofs with [`Backend::prove()`]
/// 5. Verify proofs with [`Backend::verify()`]
///
pub trait Backend {
    type CircuitConstraint;
    type ProvingKey;
    type VerifyingKey;
    type Proof;

    /// Error type for backend operations that must implement standard error traits.
    type Error: std::error::Error + Send + Sync + 'static;

    /// Creates a new instance of the backend.
    ///
    /// This is typically used to initialize any backend-specific state or configuration.
    fn new() -> Self;

    fn metadata(&self) -> MetadataInfo;

    /// Compiles a circuit definition into the backend's internal constraint representation.
    ///
    /// This method takes a high-level circuit definition and converts it into the specific
    /// constraint system format required by the backend implementation.
    ///
    /// # Arguments
    ///
    /// * `circuit` - The circuit definition to compile
    ///
    /// # Returns
    ///
    /// Returns the compiled circuit constraints on success, or an error if compilation fails.
    ///
    /// # Errors
    ///
    /// This function may return an error if:
    /// - The circuit definition is malformed or invalid
    /// - The backend cannot represent the given circuit constraints
    /// - Internal compilation errors occur
    fn compile(&self, circuit: &CircuitDefinition) -> Result<Self::CircuitConstraint, Self::Error>;

    /// Performs the trusted setup phase to generate proving and verifying keys.
    ///
    /// This is a critical phase in ZK-SNARK systems that generates the cryptographic keys
    /// required for proof generation and verification. The setup is specific to the compiled
    /// circuit constraints.
    ///
    /// # Arguments
    ///
    /// * `cs` - The compiled circuit constraints from [`Backend::compile()`]
    ///
    /// # Returns
    ///
    /// Returns a tuple containing the proving key and verifying key on success.
    ///
    /// # Security Note
    ///
    /// The security of the entire proving system depends on this setup phase being performed
    /// correctly and the setup randomness being properly discarded ("toxic waste").
    fn setup(
        &self,
        cs: &Self::CircuitConstraint,
    ) -> Result<(Self::ProvingKey, Self::VerifyingKey), Self::Error>;

    /// Generates a zero-knowledge proof for the given witness.
    ///
    /// Creates a cryptographic proof that demonstrates knowledge of a valid witness
    /// satisfying the circuit constraints, without revealing the witness itself.
    ///
    /// # Arguments
    ///
    /// * `cs` - The compiled circuit constraints
    /// * `pk` - The proving key from the trusted setup
    /// * `witness` - The complete witness (both public and private inputs)
    ///
    /// # Returns
    ///
    /// Returns the generated proof on success.
    ///
    /// # Errors
    ///
    /// This function may return an error if:
    /// - The witness does not satisfy the circuit constraints
    /// - The proving key is incompatible with the circuit constraints
    /// - Cryptographic operations fail during proof generation
    fn prove(
        &self,
        cs: &Self::CircuitConstraint,
        pk: &Self::ProvingKey,
        witness: &Witness,
    ) -> Result<Self::Proof, Self::Error>;

    /// Verifies a zero-knowledge proof against public inputs.
    ///
    /// Checks whether a given proof is valid for the specified public witness,
    /// without requiring knowledge of the private witness components.
    ///
    /// # Arguments
    ///
    /// * `vk` - The verifying key from the trusted setup
    /// * `proof` - The proof to verify
    /// * `public_witness` - The public inputs and outputs
    ///
    /// # Returns
    ///
    /// Returns `true` if the proof is valid, `false` otherwise.
    ///
    /// # Errors
    ///
    /// This function may return an error if:
    /// - The verifying key is malformed or incompatible
    /// * The proof format is invalid
    /// - Cryptographic verification operations fail
    fn verify(
        &self,
        vk: &Self::VerifyingKey,
        proof: &Self::Proof,
        public_witness: &PublicWitness,
    ) -> Result<bool, Self::Error>;
}
