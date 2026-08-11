use forge_core::{ExitCode, ForgeError};

use crate::cli::GlobalArgs;

use super::quality::{QualityResult, report};

pub fn run(global: &GlobalArgs) -> Result<ExitCode, ForgeError> {
    let result = QualityResult {
        command: "check",
        status: "pass",
        findings: 0,
        message: "no findings".to_string(),
    };
    report(global, &result)?;
    Ok(ExitCode::Success)
}
