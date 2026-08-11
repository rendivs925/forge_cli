use std::fmt;

/// Deterministic exit codes Forge SHALL expose so CI systems can distinguish
/// quality failure from execution failure and configuration errors.
///
/// 0  success
/// 1  quality gate failed
/// 2  usage/configuration error
/// 3  tool execution error
/// 4  internal Forge error
/// 5  interrupted
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitCode {
    Success,
    QualityGateFailed,
    Usage,
    ToolExecution,
    Internal,
    Interrupted,
}

impl ExitCode {
    pub const fn as_i32(self) -> i32 {
        match self {
            Self::Success => 0,
            Self::QualityGateFailed => 1,
            Self::Usage => 2,
            Self::ToolExecution => 3,
            Self::Internal => 4,
            Self::Interrupted => 5,
        }
    }
}

impl fmt::Display for ExitCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_i32())
    }
}

#[cfg(test)]
mod tests {
    use super::ExitCode;

    #[test]
    fn exit_codes_match_contract() {
        assert_eq!(ExitCode::Success.as_i32(), 0);
        assert_eq!(ExitCode::QualityGateFailed.as_i32(), 1);
        assert_eq!(ExitCode::Usage.as_i32(), 2);
        assert_eq!(ExitCode::ToolExecution.as_i32(), 3);
        assert_eq!(ExitCode::Internal.as_i32(), 4);
        assert_eq!(ExitCode::Interrupted.as_i32(), 5);
    }
}
