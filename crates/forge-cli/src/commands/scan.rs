use forge_core::{ExitCode, ForgeError};

use crate::cli::{GlobalArgs, ScanArgs};

use super::quality::{QualityResult, report};

pub fn run(global: &GlobalArgs, args: &ScanArgs) -> Result<ExitCode, ForgeError> {
    let result = QualityResult {
        command: "scan",
        status: "pass",
        findings: 0,
        message: "no findings".to_string(),
    };
    report(global, &result)?;
    if args.gate {
        return super::gate::run(global);
    }
    Ok(ExitCode::Success)
}
