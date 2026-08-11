use forge_analysis::analysis::Analysis;
use forge_analysis::gate::{GateDecision, GateEvaluator};
use forge_core::{ExitCode, ForgeError};

use crate::cli::{Format, GlobalArgs, ScanArgs};
use crate::output;

use super::analyze;
use super::quality::{QualityResult, report};

pub fn run(global: &GlobalArgs, args: &ScanArgs) -> Result<ExitCode, ForgeError> {
    let analysis = analyze::run(global)?;
    let decision = if args.gate {
        Some(GateEvaluator::evaluate(
            &analysis,
            &analyze::policy(global)?,
        ))
    } else {
        None
    };
    report_scan(global, &analysis, decision.as_ref())?;
    Ok(analyze::analysis_exit_code(&analysis, decision.as_ref()))
}

fn report_scan(
    global: &GlobalArgs,
    analysis: &Analysis,
    decision: Option<&GateDecision>,
) -> Result<(), ForgeError> {
    match global.format {
        Format::Terminal => {
            let result = QualityResult {
                command: "scan",
                status: status(decision),
                findings: analysis.findings.len(),
                message: message(analysis, decision),
            };
            report(global, &result)
        }
        Format::Json => {
            let result = match decision {
                Some(decision) => analyze::AnalysisResult::from_gate("scan", decision, analysis),
                None => analyze::AnalysisResult::from_run("scan", analysis),
            };
            output::render_json(&result)
        }
    }
}

fn status(decision: Option<&GateDecision>) -> &'static str {
    match decision {
        Some(decision) if decision.passed() => "pass",
        Some(_) => "fail",
        None => "pass",
    }
}

fn message(analysis: &Analysis, decision: Option<&GateDecision>) -> String {
    match decision {
        Some(GateDecision::Pass) => "quality gate passed".to_string(),
        Some(GateDecision::Fail { policy, violations }) => {
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
        None => format!("{} findings", analysis.findings.len()),
    }
}
