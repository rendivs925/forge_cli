use forge_core::{ExitCode, ForgeError};

use crate::cli::{Format, GlobalArgs};

use super::analyze;
use super::quality::{QualityResult, report};

pub fn run(global: &GlobalArgs) -> Result<ExitCode, ForgeError> {
    let analysis = analyze::run(global)?;
    match global.format {
        Format::Terminal => {
            let result = QualityResult {
                command: "check",
                status: if analysis.has_execution_failures() {
                    "execution-failed"
                } else {
                    "pass"
                },
                findings: analysis.findings.len(),
                message: format!("{} findings", analysis.findings.len()),
            };
            report(global, &result)?;
        }
        Format::Json => {
            let result = analyze::AnalysisResult::from_run("check", &analysis);
            crate::output::render_json(&result)?;
        }
    }
    Ok(analyze::analysis_exit_code(&analysis, None))
}
