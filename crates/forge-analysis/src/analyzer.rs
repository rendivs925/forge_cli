use serde::{Deserialize, Serialize};

use crate::finding::Finding;

/// Identity and capabilities an analyzer exposes to the engine.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnalyzerInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub capabilities: Vec<String>,
    pub supported_project_types: Vec<String>,
}

/// Requirements for the external tool an analyzer needs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolRequirement {
    /// Name of the executable on PATH or an absolute path.
    pub executable: String,
    /// Optional supported version range (e.g. ">=1.60").
    pub supported_version_range: Option<String>,
}

/// Context passed to an analyzer for a single run.
#[derive(Debug, Clone)]
pub struct RunContext {
    pub workspace_root: std::path::PathBuf,
    pub executable: String,
    pub args: Vec<String>,
    pub timeout: std::time::Duration,
    pub version_command: Option<Vec<String>>,
}

/// Normalized output of one analyzer execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunOutput {
    pub findings: Vec<Finding>,
    pub tool_version: Option<String>,
    pub duration: std::time::Duration,
}

/// Outcome of one analyzer execution, distinct from any findings produced.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecutionOutcome {
    Succeeded {
        tool_version: Option<String>,
        duration_ms: u64,
    },
    Failed {
        reason: FailedReason,
        message: String,
        duration_ms: u64,
    },
}

impl ExecutionOutcome {
    pub fn succeeded(version: Option<String>, duration: std::time::Duration) -> Self {
        Self::Succeeded {
            tool_version: version,
            duration_ms: duration.as_millis() as u64,
        }
    }

    pub fn failed(reason: FailedReason, message: String, duration: std::time::Duration) -> Self {
        Self::Failed {
            reason,
            message,
            duration_ms: duration.as_millis() as u64,
        }
    }

    pub fn is_failed(&self) -> bool {
        matches!(self, Self::Failed { .. })
    }
}

/// Why an analyzer execution failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FailedReason {
    MissingTool,
    ExitStatus,
    Timeout,
    Internal,
}

/// How an analyzer interprets the tool's stdout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum OutputFormat {
    /// Each stdout line is a JSON-serialized `Finding`.
    JsonLines,
    /// Findings come from exit code alone; stdout is ignored.
    ExitCode,
}

/// An analyzer that executes an external process.
///
/// The engine treats analyzers as opaque: each analyzer returns its own
/// findings and is responsible for its own tool execution.
pub trait Analyzer: Send + Sync {
    fn info(&self) -> AnalyzerInfo;
    fn tool_requirement(&self) -> ToolRequirement;

    /// Execute the analyzer. Failures are returned as errors and recorded by
    /// the engine as execution outcomes, never as findings.
    fn run(&self, ctx: &RunContext) -> Result<RunOutput, String>;
}

/// Generic process-based analyzer driven by configuration.
///
/// Runs the configured executable with the configured arguments, captures
/// stdout, enforces a timeout via process polling, and optionally records the
/// tool version from a version command (FORGE-ENG-040..043).
pub struct CommandAnalyzer {
    info: AnalyzerInfo,
    requirement: ToolRequirement,
    format: OutputFormat,
}

impl CommandAnalyzer {
    pub fn new(
        id: &str,
        name: &str,
        description: &str,
        executable: &str,
        format: OutputFormat,
    ) -> Self {
        Self {
            info: AnalyzerInfo {
                id: id.to_string(),
                name: name.to_string(),
                description: description.to_string(),
                capabilities: Vec::new(),
                supported_project_types: Vec::new(),
            },
            requirement: ToolRequirement {
                executable: executable.to_string(),
                supported_version_range: None,
            },
            format,
        }
    }

    pub fn with_capabilities(mut self, capabilities: Vec<&str>) -> Self {
        self.info.capabilities = capabilities.iter().map(|c| c.to_string()).collect();
        self
    }

    pub fn with_project_types(mut self, project_types: Vec<&str>) -> Self {
        self.info.supported_project_types = project_types.iter().map(|c| c.to_string()).collect();
        self
    }

    pub fn with_version_range(mut self, range: &str) -> Self {
        self.requirement.supported_version_range = Some(range.to_string());
        self
    }
}

impl Analyzer for CommandAnalyzer {
    fn info(&self) -> AnalyzerInfo {
        self.info.clone()
    }

    fn tool_requirement(&self) -> ToolRequirement {
        self.requirement.clone()
    }

    fn run(&self, ctx: &RunContext) -> Result<RunOutput, String> {
        let started = std::time::Instant::now();
        let version = resolve_version(ctx)
            .map_err(|message| {
                format!(
                    "could not determine tool version for '{}': {message}",
                    self.info.id
                )
            })
            .ok()
            .flatten();

        let output = run_process(&ctx.executable, &ctx.args, ctx.timeout)?;
        if !output.status.success() {
            return Err(format!(
                "'{}' exited with status {}",
                ctx.executable,
                output.status.code().unwrap_or(-1)
            ));
        }
        let findings = parse_findings(&self.format, &output.stdout)?;
        Ok(RunOutput {
            findings,
            tool_version: version,
            duration: started.elapsed(),
        })
    }
}

/// Spawn the process, enforce the deadline via polling, and collect output.
fn run_process(
    executable: &str,
    args: &[String],
    timeout: std::time::Duration,
) -> Result<std::process::Output, String> {
    let mut child = std::process::Command::new(executable)
        .args(args)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|error| format!("could not start '{executable}': {error}"))?;

    let deadline = std::time::Instant::now() + timeout;
    loop {
        match child.try_wait() {
            Ok(Some(_)) => {
                return child
                    .wait_with_output()
                    .map_err(|error| format!("could not collect output: {error}"));
            }
            Ok(None) => {
                if std::time::Instant::now() >= deadline {
                    let _ = child.kill();
                    return Err(format!("'{executable}' exceeded timeout of {timeout:?}"));
                }
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
            Err(error) => {
                return Err(format!("could not wait for '{executable}': {error}"));
            }
        }
    }
}

fn resolve_version(ctx: &RunContext) -> Result<Option<String>, String> {
    let Some(args) = &ctx.version_command else {
        return Ok(None);
    };
    let Some((first, rest)) = args.split_first() else {
        return Ok(None);
    };
    let output = std::process::Command::new(first)
        .args(rest)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .output()
        .map_err(|error| error.to_string())?;
    if !output.status.success() {
        return Ok(None);
    }
    let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(if text.is_empty() { None } else { Some(text) })
}

fn parse_findings(format: &OutputFormat, stdout: &[u8]) -> Result<Vec<Finding>, String> {
    match format {
        OutputFormat::ExitCode => Ok(Vec::new()),
        OutputFormat::JsonLines => {
            let text = String::from_utf8_lossy(stdout);
            let mut findings = Vec::new();
            for line in text.lines() {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                let finding: Finding = serde_json::from_str(line).map_err(|error| {
                    format!("invalid finding line '{}': {error}", truncate(line, 80))
                })?;
                findings.push(finding);
            }
            Ok(findings)
        }
    }
}

fn truncate(text: &str, max: usize) -> String {
    if text.len() <= max {
        text.to_string()
    } else {
        let end = text.floor_char_boundary(max);
        format!("{}…", &text[..end])
    }
}

#[cfg(test)]
mod tests {
    use super::{ExecutionOutcome, FailedReason, OutputFormat, parse_findings};
    use crate::finding::{Category, Finding, Location, Severity};

    #[test]
    fn json_lines_format_parses_findings() {
        let finding = Finding::new(
            "demo",
            "demo.rule",
            Severity::Major,
            Category::Maintainability,
            Location {
                file: "src/main.rs".to_string(),
                start_line: Some(3),
                start_column: None,
                end_line: None,
                end_column: None,
            },
            "a message",
            None,
        );
        let json = serde_json::to_string(&finding).unwrap();
        let parsed = parse_findings(&OutputFormat::JsonLines, json.as_bytes()).unwrap();
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].id, finding.id);
        assert_eq!(parsed[0].message, "a message");
    }

    #[test]
    fn exit_code_format_has_no_findings() {
        let parsed = parse_findings(&OutputFormat::ExitCode, b"ignored output").unwrap();
        assert!(parsed.is_empty());
    }

    #[test]
    fn invalid_json_line_is_an_error() {
        let err = parse_findings(&OutputFormat::JsonLines, b"not-json").unwrap_err();
        assert!(err.contains("invalid finding line"));
    }

    #[test]
    fn succeeded_is_not_failed() {
        let outcome = ExecutionOutcome::succeeded(None, std::time::Duration::from_secs(1));
        assert!(!outcome.is_failed());
    }

    #[test]
    fn failed_is_failed() {
        let outcome = ExecutionOutcome::failed(
            FailedReason::Timeout,
            "tool exceeded timeout".to_string(),
            std::time::Duration::from_secs(5),
        );
        assert!(outcome.is_failed());
    }
}
