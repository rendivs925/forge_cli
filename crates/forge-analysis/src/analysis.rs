use serde::{Deserialize, Serialize};

use crate::analyzer::ExecutionOutcome;
use crate::finding::Finding;

/// Unified result of one analysis run.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Analysis {
    pub findings: Vec<Finding>,
    pub outcomes: Vec<ExecutionOutcome>,
}

impl Analysis {
    pub fn new(findings: Vec<Finding>, outcomes: Vec<ExecutionOutcome>) -> Self {
        Self { findings, outcomes }
    }

    pub fn empty() -> Self {
        Self::default()
    }

    /// Whether any enabled analyzer failed to execute.
    pub fn has_execution_failures(&self) -> bool {
        self.outcomes.iter().any(ExecutionOutcome::is_failed)
    }
}

#[cfg(test)]
mod tests {
    use super::Analysis;
    use crate::analyzer::{ExecutionOutcome, FailedReason};

    #[test]
    fn execution_failures_detected() {
        let analysis = Analysis::new(
            Vec::new(),
            vec![ExecutionOutcome::failed(
                FailedReason::MissingTool,
                "not found".to_string(),
                std::time::Duration::ZERO,
            )],
        );
        assert!(analysis.has_execution_failures());
    }

    #[test]
    fn no_failures_when_all_succeeded() {
        let analysis = Analysis::new(
            Vec::new(),
            vec![ExecutionOutcome::succeeded(None, std::time::Duration::ZERO)],
        );
        assert!(!analysis.has_execution_failures());
    }
}
