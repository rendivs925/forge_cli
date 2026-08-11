use forge_analysis::gate::{GateDecision, GateEvaluator};
use forge_core::{ExitCode, ForgeError};

use crate::cli::{Format, GlobalArgs};
use crate::output;

use super::analyze;
use super::quality::{QualityResult, report};

pub fn run(global: &GlobalArgs) -> Result<ExitCode, ForgeError> {
    let analysis = match analyze::load(global) {
        Ok(analysis) => analysis,
        Err(error) => {
            report(
                global,
                &QualityResult {
                    command: "gate",
                    status: "error",
                    findings: 0,
                    message: error.to_string(),
                },
            )?;
            return Ok(ExitCode::Usage);
        }
    };
    let policy = analyze::policy(global)?;
    let decision = GateEvaluator::evaluate(&analysis, &policy);
    report_gate(global, &decision)?;
    Ok(if decision.passed() {
        ExitCode::Success
    } else {
        ExitCode::QualityGateFailed
    })
}

fn report_gate(global: &GlobalArgs, decision: &GateDecision) -> Result<(), ForgeError> {
    match global.format {
        Format::Terminal => {
            let message = match decision {
                GateDecision::Pass => "quality gate passed".to_string(),
                GateDecision::Fail { policy, violations } => {
                    let reasons: Vec<String> = violations
                        .iter()
                        .map(|violation| {
                            format!(
                                "{} ({}/{} limit)",
                                violation.condition, violation.actual, violation.limit
                            )
                        })
                        .collect();
                    format!(
                        "quality gate failed: policy '{policy}': {}",
                        reasons.join(", ")
                    )
                }
            };
            let result = QualityResult {
                command: "gate",
                status: if decision.passed() { "pass" } else { "fail" },
                findings: 0,
                message,
            };
            report(global, &result)
        }
        Format::Json => output::render_json(decision),
    }
}
