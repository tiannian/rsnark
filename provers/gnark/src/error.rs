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

    #[error("unknown go error: {0}")]
    UnknownGoError(i64),
}

pub type Result<T> = std::result::Result<T, Error>;
