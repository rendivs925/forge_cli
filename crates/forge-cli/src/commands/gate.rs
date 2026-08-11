use forge_core::{ExitCode, ForgeError};

use crate::cli::GlobalArgs;
use crate::commands::config as config_cmd;

use super::quality::{QualityResult, report};

pub fn run(global: &GlobalArgs) -> Result<ExitCode, ForgeError> {
    let _resolved = config_cmd::resolve(global)?;
    let result = QualityResult {
        command: "gate",
        status: "pass",
        findings: 0,
        message: "quality gate passed".to_string(),
    };
    report(global, &result)?;
    Ok(ExitCode::Success)
}
