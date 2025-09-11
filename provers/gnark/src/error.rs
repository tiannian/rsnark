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
}

impl Error {
    pub fn from_go_error(code: i64) -> Self {
        match code {
            -10003 => Self::SerializeError,
            -10001 => Self::DeserializeError,
            -10004 => Self::WriteToFileError,
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
            _ => Self::UnknownGoError(code),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
