use super::*;

#[derive(Debug)]
pub enum ResearchStorageError {
    Io(io::Error),
    Serde(serde_json::Error),
    Encryption(String),
    InvalidPath(String),
    InvalidSnapshot(String),
    UnsupportedSchema(u32),
}

impl std::fmt::Display for ResearchStorageError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "storage I/O error: {error}"),
            Self::Serde(error) => write!(formatter, "storage serialization error: {error}"),
            Self::Encryption(message) => {
                write!(formatter, "research storage encryption error: {message}")
            }
            Self::InvalidPath(message) => write!(formatter, "{message}"),
            Self::InvalidSnapshot(message) => write!(formatter, "{message}"),
            Self::UnsupportedSchema(version) => {
                write!(
                    formatter,
                    "unsupported research library schema version {version}"
                )
            }
        }
    }
}

impl std::error::Error for ResearchStorageError {}

impl From<io::Error> for ResearchStorageError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_json::Error> for ResearchStorageError {
    fn from(error: serde_json::Error) -> Self {
        Self::Serde(error)
    }
}
