/// Error types for Gnark backend operations.
///
/// This enum represents all possible errors that can occur when interacting
/// with the Gnark Go library through FFI bindings. Errors are categorized
/// into serialization/deserialization issues, file I/O problems, and
/// cryptographic operation failures.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("serialize error")]
    SerializeError,

    #[error("deserialize error")]
    DeserializeError,

    #[error("write to file error")]
    WriteToFileError,

    #[error("read from file error")]
    ReadFromFileError,

    #[error("convert compiled circuit to types.CompiledCircuit error")]
    ConvertCompiledCircuitError,

    #[error("convert pk to types.Groth16ProvingKey error")]
    ConvertPkError,

    #[error("convert vk to types.Groth16VerifyingKey error")]
    ConvertVkError,

    #[error("setup error")]
    SetupError,

    #[error("prove error")]
    ProveError,

    #[error("verify error")]
    VerifyError,

    #[error("unknown go error: {0}")]
    UnknownGoError(i64),

    #[error("circuit definition parse error")]
    CircuitDefinitionParseError,

    #[error("compile error")]
    CompileError,

    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("proof length is wrong")]
    ProofLengthWrong,

    #[error("failed to export solidity")]
    ExportSolidityError,

    #[error("prover not found")]
    ProverNotFoundError,

    #[error("object not found")]
    ObjectNotFoundError,

    #[error(transparent)]
    ConvertToStringError(#[from] std::string::FromUtf8Error),
}

impl Error {
    /// Converts a Go error code to the corresponding Rust error type.
    ///
    /// The Gnark Go library returns specific error codes for different failure conditions.
    /// This method maps those numeric codes to appropriate Rust error variants.
    ///
    /// # Arguments
    ///
    /// * `code` - The error code returned from the Go library
    ///
    /// # Returns
    ///
    /// The corresponding Error variant, or `UnknownGoError` if the code is unrecognized.
    pub fn from_go_error(code: i64) -> Self {
        match code {
            -10003 => Self::SerializeError,
            -10001 => Self::DeserializeError,
            -10004 => Self::WriteToFileError,
            -10005 => Self::ConvertVkError,
            -10002 => Self::ReadFromFileError,
            -20001 => Self::CircuitDefinitionParseError,
            -20002 => Self::CompileError,
            -20003 => Self::ConvertCompiledCircuitError,
            -20004 => Self::SetupError,
            -20005 => Self::ConvertPkError,
            -20006 => Self::ConvertCompiledCircuitError,
            -20007 => Self::DeserializeError,
            -20008 => Self::ProveError,
            -20009 => Self::ConvertVkError,
            -20010 => Self::VerifyError,
            -20011 => Self::ProverNotFoundError,
            -20012 => Self::ObjectNotFoundError,
            -10006 => Self::ExportSolidityError,
            _ => Self::UnknownGoError(code),
        }
    }
}

/// Convenience type alias for Results with Gnark Error.
///
/// This type alias provides a shorter way to write `Result<T, Error>` throughout
/// the crate, making error handling more concise and consistent.
pub type Result<T> = std::result::Result<T, Error>;
