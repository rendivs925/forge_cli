use std::fmt;

use crate::exit_code::ExitCode;

/// Errors that map to a specific exit code category so CI can distinguish
/// quality failure from execution failure and configuration errors.
#[derive(Debug)]
pub enum ForgeError {
    Usage(String),
    Config(String),
    ToolExecution(String),
    Internal(String),
}

impl ForgeError {
    pub fn exit_code(&self) -> ExitCode {
        match self {
            Self::Usage(_) | Self::Config(_) => ExitCode::Usage,
            Self::ToolExecution(_) => ExitCode::ToolExecution,
            Self::Internal(_) => ExitCode::Internal,
        }
    }
}

impl fmt::Display for ForgeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Usage(message) => write!(f, "{message}"),
            Self::Config(message) => write!(f, "configuration error: {message}"),
            Self::ToolExecution(message) => write!(f, "tool execution error: {message}"),
            Self::Internal(message) => write!(f, "internal error: {message}"),
        }
    }
}

impl std::error::Error for ForgeError {}

impl From<std::io::Error> for ForgeError {
    fn from(error: std::io::Error) -> Self {
        Self::Internal(error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::ForgeError;
    use crate::exit_code::ExitCode;

    #[test]
    fn error_exit_codes_match_contract() {
        assert_eq!(ForgeError::Usage("x".into()).exit_code(), ExitCode::Usage);
        assert_eq!(ForgeError::Config("x".into()).exit_code(), ExitCode::Usage);
        assert_eq!(
            ForgeError::ToolExecution("x".into()).exit_code(),
            ExitCode::ToolExecution
        );
        assert_eq!(
            ForgeError::Internal("x".into()).exit_code(),
            ExitCode::Internal
        );
    }
}
