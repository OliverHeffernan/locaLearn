use std::path::PathBuf;

/// Result type used across the study application.
pub type Result<T> = std::result::Result<T, StudyError>;

/// Errors produced by CLI, filesystem, provider, and terminal workflows.
#[derive(Debug, thiserror::Error)]
pub enum StudyError {
    /// A filesystem operation failed.
    #[error("filesystem operation failed: {0}")]
    Io(#[from] std::io::Error),

    /// TOML serialization or deserialization failed.
    #[error("toml operation failed: {0}")]
    TomlSerialize(#[from] toml::ser::Error),

    /// TOML deserialization failed.
    #[error("toml parsing failed: {0}")]
    TomlDeserialize(#[from] toml::de::Error),

    /// The current directory is not a valid study set.
    #[error("not a study set directory: {0}")]
    NotAStudySet(PathBuf),

    /// A provider named in configuration was not registered.
    #[error("generation provider is not registered: {0}")]
    ProviderNotFound(String),

    /// A provider model selector could not be parsed or resolved.
    #[error("invalid provider model selector: {0}")]
    InvalidProviderModel(String),

    /// A configured artifact is not registered.
    #[error("study artifact is not registered: {0}")]
    ArtifactNotFound(String),

    /// A provider failed to complete generation.
    #[error("generation provider failed: {0}")]
    ProviderFailed(String),

    /// User input could not be handled.
    #[error("invalid input: {0}")]
    InvalidInput(String),
}
