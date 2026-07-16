use std::io;

#[derive(Debug, thiserror::Error)]
pub enum CorpusError {
    #[error("policy configuration is invalid: {0}")]
    Policy(String),
    #[error("artifact verification failed: {0}")]
    Verification(String),
    #[error("unsafe archive: {0}")]
    UnsafeArchive(String),
    #[error("archive format error: {0}")]
    Archive(String),
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("serialization error: {0}")]
    Serialize(#[from] serde_json::Error),
    #[error("TOML error: {0}")]
    Toml(#[from] toml::de::Error),
    #[error("network acquisition failed: {0}")]
    Network(String),
}

pub type Result<T> = std::result::Result<T, CorpusError>;
