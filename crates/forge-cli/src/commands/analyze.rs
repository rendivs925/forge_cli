use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;

use forge_analysis::analysis::Analysis;
use forge_analysis::analyzer::{CommandAnalyzer, OutputFormat, RunContext};
use forge_analysis::engine::{AnalysisEngine, EngineConfig, RunUnit};
use forge_analysis::finding::Category;
use forge_analysis::gate::{GateDecision, Policy, PolicyResolver};
use forge_analysis::store::AnalysisStore;
use forge_config::ResolvedConfig;
use forge_config::config::ToolConfig;
use forge_core::{ExitCode, ForgeError};
use serde::Serialize;

use crate::cli::GlobalArgs;
use crate::workspace;

use super::config as config_cmd;

/// Runs the configured analyzers and persists the result.
pub fn run(global: &GlobalArgs) -> Result<Analysis, ForgeError> {
    let resolved = config_cmd::resolve(global)?;
    let root = workspace::workspace_root(global);
    let (concurrency, timeout) = engine_bounds(&resolved);

    let units = build_units(&resolved, &root, timeout)?;
    let engine = AnalysisEngine::new(EngineConfig {
        concurrency,
        default_timeout: timeout,
    });
    let analysis = engine.run(&units);
    AnalysisStore::new(root)
        .save(&analysis)
        .map_err(|error| ForgeError::Internal(error.to_string()))?;
    Ok(analysis)
}

/// Loads the persisted analysis for gate evaluation.
pub fn load(global: &GlobalArgs) -> Result<Analysis, ForgeError> {
    let root = workspace::workspace_root(global);
    AnalysisStore::new(root).load().map_err(|error| {
        ForgeError::Usage(format!(
            "no analysis result available (run 'forge scan' first): {error}"
        ))
    })
}

/// Resolves the configured policy for gate evaluation.
pub fn policy(global: &GlobalArgs) -> Result<Policy, ForgeError> {
    let resolved = config_cmd::resolve(global)?;
    Ok(resolve_policy(&resolved))
}

fn build_units(
    resolved: &ResolvedConfig,
    root: &Path,
    timeout: Duration,
) -> Result<Vec<RunUnit>, ForgeError> {
    let mut units = Vec::new();
    for (tool_id, tool) in active_tools(resolved) {
        units.push(RunUnit {
            analyzer: Box::new(analyzer_for(tool_id, tool)),
            context: context_for(root, tool, timeout),
        });
    }
    Ok(units)
}

fn analyzer_for(tool_id: &str, tool: &ToolConfig) -> CommandAnalyzer {
    let mut analyzer = CommandAnalyzer::new(
        tool_id,
        tool_id,
        &format!("configured tool '{tool_id}'"),
        &tool.executable,
        OutputFormat::JsonLines,
    );
    if let Some(range) = &tool.supported_version_range {
        analyzer = analyzer.with_version_range(range);
    }
    analyzer
}

fn context_for(root: &Path, tool: &ToolConfig, timeout: Duration) -> RunContext {
    RunContext {
        workspace_root: root.to_path_buf(),
        executable: tool.executable.clone(),
        args: tool.args.clone(),
        timeout,
        version_command: tool.version_command.clone(),
    }
}

/// Enabled tools of the active profile. Unknown or disabled tools are skipped.
fn active_tools(resolved: &ResolvedConfig) -> Vec<(&str, &ToolConfig)> {
    let Some(profile_name) = &resolved.config.profile else {
        return Vec::new();
    };
    let Some(profile) = resolved.config.profiles.get(profile_name) else {
        return Vec::new();
    };
    profile
        .tools
        .iter()
        .filter_map(|id| {
            resolved
                .config
                .tools
                .get(id)
                .map(|tool| (id.as_str(), tool))
        })
        .filter(|(_, tool)| tool.enabled)
        .collect()
}

fn engine_bounds(resolved: &ResolvedConfig) -> (usize, Duration) {
    let concurrency = resolved
        .config
        .profile
        .as_ref()
        .and_then(|name| resolved.config.profiles.get(name))
        .map(|profile| profile.concurrency)
        .filter(|value| *value > 0)
        .unwrap_or(4);
    let timeout_secs = resolved
        .config
        .tools
        .values()
        .find_map(|tool| tool.timeout_secs)
        .unwrap_or(120);
    (concurrency, Duration::from_secs(timeout_secs))
}

fn resolve_policy(resolved: &ResolvedConfig) -> Policy {
    let policies: HashMap<String, Policy> = resolved
        .config
        .policies
        .iter()
        .map(|(name, config)| (name.clone(), policy_from(config, name)))
        .collect();
    PolicyResolver::resolve(&policies, resolved.config.gate_policy.as_deref())
}

fn policy_from(config: &forge_config::config::PolicyConfig, name: &str) -> Policy {
    let default = Policy::default_named(name);
    let mut policy = Policy {
        name: name.to_string(),
        max_blockers: config.max_blockers.unwrap_or(default.max_blockers),
        max_critical: config.max_critical.unwrap_or(default.max_critical),
        max_major: config.max_major.unwrap_or(default.max_major),
        max_minor: config.max_minor.unwrap_or(default.max_minor),
        categories: default.categories,
    };
    for (category, limit) in &config.categories {
        if let Some(category) = parse_category(category) {
            policy.categories.insert(category, *limit);
        }
    }
    policy
}

fn parse_category(name: &str) -> Option<Category> {
    match name {
        "security" => Some(Category::Security),
        "reliability" => Some(Category::Reliability),
        "maintainability" => Some(Category::Maintainability),
        "architecture" => Some(Category::Architecture),
        "performance" => Some(Category::Performance),
        "supply-chain" => Some(Category::SupplyChain),
        _ => None,
    }
}

/// Result of a gated command, serializable for `--format json`.
#[derive(Debug, Serialize)]
pub struct AnalysisResult {
    pub command: &'static str,
    pub status: String,
    pub findings: usize,
    pub failed_analyzers: usize,
    pub message: String,
}

impl AnalysisResult {
    pub fn from_run(command: &'static str, analysis: &Analysis) -> Self {
        Self {
            command,
            status: if analysis.has_execution_failures() {
                "execution-failed".to_string()
            } else {
                "pass".to_string()
            },
            findings: analysis.findings.len(),
            failed_analyzers: analysis
                .outcomes
                .iter()
                .filter(|outcome| outcome.is_failed())
                .count(),
            message: format!("{} findings", analysis.findings.len()),
        }
    }

    pub fn from_gate(command: &'static str, decision: &GateDecision, analysis: &Analysis) -> Self {
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
        Self {
            command,
            status: if decision.passed() {
                "pass".to_string()
            } else {
                "fail".to_string()
            },
            findings: analysis.findings.len(),
            failed_analyzers: analysis
                .outcomes
                .iter()
                .filter(|outcome| outcome.is_failed())
                .count(),
            message,
        }
    }
}

/// Exit code for a completed analysis: 0 success, 1 gate failure, 3 execution failure.
pub fn analysis_exit_code(analysis: &Analysis, decision: Option<&GateDecision>) -> ExitCode {
    if analysis.has_execution_failures() {
        return ExitCode::ToolExecution;
    }
    if decision.map(GateDecision::passed) == Some(false) {
        return ExitCode::QualityGateFailed;
    }
    ExitCode::Success
}
