use std::time::Duration;

use crate::analysis::Analysis;
use crate::analyzer::{Analyzer, ExecutionOutcome, FailedReason, RunContext, RunOutput};

/// Engine configuration bounds for execution.
#[derive(Debug, Clone, Copy)]
pub struct EngineConfig {
    pub concurrency: usize,
    pub default_timeout: Duration,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            concurrency: 4,
            default_timeout: Duration::from_secs(120),
        }
    }
}

/// One analyzer execution: the analyzer plus its dedicated context.
pub struct RunUnit {
    pub analyzer: Box<dyn Analyzer>,
    pub context: RunContext,
}

/// Orchestrates analyzers and combines their results into one `Analysis`.
///
/// Execution is bounded by `concurrency`, isolated per analyzer, and
/// deterministic in how results are merged regardless of execution order.
pub struct AnalysisEngine {
    config: EngineConfig,
}

impl AnalysisEngine {
    pub fn new(config: EngineConfig) -> Self {
        Self { config }
    }

    pub fn run(&self, units: &[RunUnit]) -> Analysis {
        let mut findings = Vec::new();
        let mut outcomes = Vec::new();

        for batch in units.chunks(self.config.concurrency.max(1)) {
            std::thread::scope(|scope| {
                let mut handles = Vec::new();
                for unit in batch {
                    handles.push(scope.spawn(move || execute(&*unit.analyzer, &unit.context)));
                }
                for handle in handles {
                    match handle.join() {
                        Ok((result, outcome)) => {
                            findings.extend(result);
                            outcomes.push(outcome);
                        }
                        Err(panic) => {
                            let message = panic_message(panic);
                            outcomes.push(ExecutionOutcome::failed(
                                FailedReason::Internal,
                                message,
                                Duration::ZERO,
                            ));
                        }
                    }
                }
            });
        }

        findings.sort_by(|a, b| {
            a.rule_id
                .cmp(&b.rule_id)
                .then_with(|| a.location.file.cmp(&b.location.file))
                .then_with(|| a.location.start_line.cmp(&b.location.start_line))
                .then_with(|| a.location.start_column.cmp(&b.location.start_column))
        });
        outcomes.sort_by_key(label);

        Analysis::new(findings, outcomes)
    }
}

fn execute(
    analyzer: &dyn Analyzer,
    ctx: &RunContext,
) -> (Vec<crate::finding::Finding>, ExecutionOutcome) {
    let started = std::time::Instant::now();
    match analyzer.run(ctx) {
        Ok(RunOutput {
            findings,
            tool_version,
            duration,
        }) => (
            findings,
            ExecutionOutcome::succeeded(tool_version, duration_or(started, duration)),
        ),
        Err(message) => {
            let reason = classify_failure(&message);
            (
                Vec::new(),
                ExecutionOutcome::failed(reason, message, started.elapsed()),
            )
        }
    }
}

fn duration_or(started: std::time::Instant, reported: Duration) -> Duration {
    if reported.is_zero() {
        started.elapsed()
    } else {
        reported
    }
}

fn classify_failure(message: &str) -> FailedReason {
    let lower = message.to_ascii_lowercase();
    if lower.contains("timeout") {
        FailedReason::Timeout
    } else if lower.contains("could not start") || lower.contains("no such file") {
        FailedReason::MissingTool
    } else if lower.contains("exited with status") {
        FailedReason::ExitStatus
    } else {
        FailedReason::Internal
    }
}

fn panic_message(panic: Box<dyn std::any::Any + Send>) -> String {
    if let Some(message) = panic.downcast_ref::<&str>() {
        format!("analyzer panicked: {message}")
    } else if let Some(message) = panic.downcast_ref::<String>() {
        format!("analyzer panicked: {message}")
    } else {
        "analyzer panicked".to_string()
    }
}

fn label(outcome: &ExecutionOutcome) -> String {
    match outcome {
        ExecutionOutcome::Succeeded { .. } => "succeeded".to_string(),
        ExecutionOutcome::Failed { reason, .. } => format!("failed-{reason:?}"),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::{AnalysisEngine, EngineConfig, RunUnit};
    use crate::analyzer::{Analyzer, AnalyzerInfo, RunContext, RunOutput, ToolRequirement};
    use crate::finding::{Category, Finding, Location, Severity};

    struct FakeAnalyzer {
        id: &'static str,
        findings: Vec<Finding>,
        fail: bool,
    }

    impl Analyzer for FakeAnalyzer {
        fn info(&self) -> AnalyzerInfo {
            AnalyzerInfo {
                id: self.id.to_string(),
                name: self.id.to_string(),
                description: "fake".to_string(),
                capabilities: Vec::new(),
                supported_project_types: Vec::new(),
            }
        }

        fn tool_requirement(&self) -> ToolRequirement {
            ToolRequirement {
                executable: "fake".to_string(),
                supported_version_range: None,
            }
        }

        fn run(&self, _ctx: &RunContext) -> Result<RunOutput, String> {
            if self.fail {
                return Err("fake failure".to_string());
            }
            Ok(RunOutput {
                findings: self.findings.clone(),
                tool_version: Some("1.0".to_string()),
                duration: std::time::Duration::from_millis(1),
            })
        }
    }

    fn finding(rule: &str, file: &str, line: u32) -> Finding {
        Finding::new(
            "fake",
            rule,
            Severity::Major,
            Category::Maintainability,
            Location {
                file: file.to_string(),
                start_line: Some(line),
                start_column: None,
                end_line: None,
                end_column: None,
            },
            "message",
            None,
        )
    }

    fn ctx() -> RunContext {
        RunContext {
            workspace_root: std::path::PathBuf::from("/tmp"),
            executable: "fake".to_string(),
            args: Vec::new(),
            timeout: std::time::Duration::from_secs(1),
            version_command: None,
        }
    }

    #[test]
    fn merges_results_from_multiple_analyzers() {
        let engine = AnalysisEngine::new(EngineConfig {
            concurrency: 2,
            ..Default::default()
        });
        let a = FakeAnalyzer {
            id: "a",
            findings: vec![finding("r1", "f.rs", 1)],
            fail: false,
        };
        let b = FakeAnalyzer {
            id: "b",
            findings: vec![finding("r0", "f.rs", 2)],
            fail: false,
        };
        let units = vec![
            RunUnit {
                analyzer: Box::new(a),
                context: ctx(),
            },
            RunUnit {
                analyzer: Box::new(b),
                context: ctx(),
            },
        ];
        let analysis = engine.run(&units);
        assert_eq!(analysis.findings.len(), 2);
        assert_eq!(analysis.findings[0].rule_id, "r0");
        assert_eq!(analysis.findings[1].rule_id, "r1");
        assert_eq!(analysis.outcomes.len(), 2);
        assert!(!analysis.has_execution_failures());
    }

    #[test]
    fn failing_analyzer_is_isolated() {
        let engine = AnalysisEngine::new(EngineConfig::default());
        let good = FakeAnalyzer {
            id: "good",
            findings: vec![finding("r1", "f.rs", 1)],
            fail: false,
        };
        let bad = FakeAnalyzer {
            id: "bad",
            findings: Vec::new(),
            fail: true,
        };
        let units = vec![
            RunUnit {
                analyzer: Box::new(good),
                context: ctx(),
            },
            RunUnit {
                analyzer: Box::new(bad),
                context: ctx(),
            },
        ];
        let analysis = engine.run(&units);
        assert_eq!(analysis.findings.len(), 1);
        assert_eq!(analysis.findings[0].rule_id, "r1");
        assert!(analysis.has_execution_failures());
    }

    #[test]
    fn panicking_analyzer_is_recorded_as_failure() {
        struct PanicAnalyzer;
        impl Analyzer for PanicAnalyzer {
            fn info(&self) -> AnalyzerInfo {
                AnalyzerInfo {
                    id: "panic".to_string(),
                    name: "panic".to_string(),
                    description: String::new(),
                    capabilities: Vec::new(),
                    supported_project_types: Vec::new(),
                }
            }
            fn tool_requirement(&self) -> ToolRequirement {
                ToolRequirement {
                    executable: "fake".to_string(),
                    supported_version_range: None,
                }
            }
            fn run(&self, _ctx: &RunContext) -> Result<RunOutput, String> {
                panic!("boom");
            }
        }

        let engine = AnalysisEngine::new(EngineConfig::default());
        let units = vec![RunUnit {
            analyzer: Box::new(PanicAnalyzer),
            context: ctx(),
        }];
        let analysis = engine.run(&units);
        assert!(analysis.findings.is_empty());
        assert!(analysis.has_execution_failures());
    }

    #[test]
    fn concurrency_is_bounded() {
        static ACTIVE: AtomicUsize = AtomicUsize::new(0);
        static MAX_ACTIVE: AtomicUsize = AtomicUsize::new(0);
        struct Tracked;
        impl Analyzer for Tracked {
            fn info(&self) -> AnalyzerInfo {
                AnalyzerInfo {
                    id: "tracked".to_string(),
                    name: "tracked".to_string(),
                    description: String::new(),
                    capabilities: Vec::new(),
                    supported_project_types: Vec::new(),
                }
            }
            fn tool_requirement(&self) -> ToolRequirement {
                ToolRequirement {
                    executable: "fake".to_string(),
                    supported_version_range: None,
                }
            }
            fn run(&self, _ctx: &RunContext) -> Result<RunOutput, String> {
                let active = ACTIVE.fetch_add(1, Ordering::SeqCst) + 1;
                MAX_ACTIVE.fetch_max(active, Ordering::SeqCst);
                std::thread::sleep(std::time::Duration::from_millis(20));
                ACTIVE.fetch_sub(1, Ordering::SeqCst);
                Ok(RunOutput {
                    findings: Vec::new(),
                    tool_version: None,
                    duration: std::time::Duration::from_millis(20),
                })
            }
        }

        let engine = AnalysisEngine::new(EngineConfig {
            concurrency: 2,
            ..Default::default()
        });
        let units: Vec<RunUnit> = (0..4)
            .map(|_| RunUnit {
                analyzer: Box::new(Tracked),
                context: ctx(),
            })
            .collect();
        let _ = engine.run(&units);
        assert!(MAX_ACTIVE.load(Ordering::SeqCst) <= 2);
    }
}
