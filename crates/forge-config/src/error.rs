use std::fmt;
use std::path::PathBuf;

use forge_core::ForgeError;

/// Errors loading, resolving, or validating Forge configuration.
#[derive(Debug)]
pub enum ConfigError {
    /// The file could not be read.
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    /// The file is not valid TOML or does not conform to the schema.
    Parse { path: PathBuf, message: String },
    /// The declared schema version is not supported.
    UnsupportedSchema { found: u32, path: PathBuf },
    /// The merged configuration failed validation.
    Invalid { key: String, message: String },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { path, source } => write!(f, "cannot read {}: {source}", path.display()),
            Self::Parse { path, message } => {
                write!(f, "invalid configuration in {}: {message}", path.display())
            }
            Self::UnsupportedSchema { found, path } => write!(
                f,
                "unsupported schema version {found} in {} (supported: {})",
                path.display(),
                crate::config::SUPPORTED_SCHEMA_VERSION
            ),
            Self::Invalid { key, message } => {
                write!(f, "invalid configuration key '{key}': {message}")
            }
        }
    }
}

impl std::error::Error for ConfigError {}

impl From<ConfigError> for ForgeError {
    fn from(error: ConfigError) -> Self {
        ForgeError::Config(error.to_string())
    }
}
