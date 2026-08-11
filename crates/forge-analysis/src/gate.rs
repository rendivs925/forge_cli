use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::analysis::Analysis;
use crate::finding::{Category, Finding, Severity};

/// A quality policy composed of severity and category thresholds.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Policy {
    pub name: String,
    pub max_blockers: u64,
    pub max_critical: u64,
    pub max_major: u64,
    pub max_minor: u64,
    #[serde(default)]
    pub categories: HashMap<Category, u64>,
}

impl Policy {
    pub fn default_named(name: &str) -> Self {
        Self {
            name: name.to_string(),
            max_blockers: 0,
            max_critical: 0,
            max_major: 10,
            max_minor: 100,
            categories: HashMap::new(),
        }
    }
}

/// One violated condition within a gate decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConditionViolation {
    pub condition: String,
    pub limit: u64,
    pub actual: u64,
    pub responsible_findings: Vec<String>,
}

/// Result of evaluating an analysis against a policy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GateDecision {
    Pass,
    Fail {
        policy: String,
        violations: Vec<ConditionViolation>,
    },
}

impl GateDecision {
    pub fn passed(&self) -> bool {
        matches!(self, Self::Pass)
    }
}

/// Deterministic, explainable evaluation of findings against a policy.
pub struct GateEvaluator;

impl GateEvaluator {
    /// Evaluate `analysis` against `policy`.
    ///
    /// Until baselines exist, every finding counts as new. The decision is
    /// deterministic for identical inputs and names the policy and findings
    /// responsible for any failure.
    pub fn evaluate(analysis: &Analysis, policy: &Policy) -> GateDecision {
        let mut violations = Vec::new();
        for (condition, limit, actual, responsible) in severity_violations(analysis, policy) {
            if actual > limit {
                violations.push(ConditionViolation {
                    condition,
                    limit,
                    actual,
                    responsible_findings: responsible,
                });
            }
        }
        for (category, limit) in &policy.categories {
            let (actual, responsible) = count_by_category(analysis, *category);
            if actual > *limit {
                violations.push(ConditionViolation {
                    condition: format!("category:{}", category.label()),
                    limit: *limit,
                    actual,
                    responsible_findings: responsible,
                });
            }
        }
        if violations.is_empty() {
            GateDecision::Pass
        } else {
            GateDecision::Fail {
                policy: policy.name.clone(),
                violations,
            }
        }
    }
}

fn severity_violations(
    analysis: &Analysis,
    policy: &Policy,
) -> Vec<(String, u64, u64, Vec<String>)> {
    let thresholds = [
        (Severity::Blocker, policy.max_blockers, "blockers"),
        (Severity::Critical, policy.max_critical, "critical"),
        (Severity::Major, policy.max_major, "major"),
        (Severity::Minor, policy.max_minor, "minor"),
    ];
    let mut violations = Vec::new();
    for (severity, limit, label) in thresholds {
        let (actual, responsible) = count_by_severity(analysis, severity);
        if actual > limit {
            violations.push((label.to_string(), limit, actual, responsible));
        }
    }
    violations
}

fn count_by_severity(analysis: &Analysis, severity: Severity) -> (u64, Vec<String>) {
    let findings: Vec<&Finding> = analysis
        .findings
        .iter()
        .filter(|finding| finding.severity == severity)
        .collect();
    (
        findings.len() as u64,
        findings.iter().map(|finding| finding.id.clone()).collect(),
    )
}

fn count_by_category(analysis: &Analysis, category: Category) -> (u64, Vec<String>) {
    let findings: Vec<&Finding> = analysis
        .findings
        .iter()
        .filter(|finding| finding.category == category)
        .collect();
    (
        findings.len() as u64,
        findings.iter().map(|finding| finding.id.clone()).collect(),
    )
}

/// Resolves the active policy from a named set.
#[derive(Debug, Clone)]
pub struct PolicyResolver;

impl PolicyResolver {
    /// Select the named policy, falling back to the built-in default.
    pub fn resolve(policies: &HashMap<String, Policy>, name: Option<&str>) -> Policy {
        let Some(name) = name else {
            return Policy::default_named("default");
        };
        policies
            .get(name)
            .cloned()
            .unwrap_or_else(|| Policy::default_named(name))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{GateDecision, GateEvaluator, Policy, PolicyResolver};
    use crate::analysis::Analysis;
    use crate::finding::{Category, Finding, Location, Severity};

    fn finding(rule: &str, severity: Severity, category: Category) -> Finding {
        Finding::new(
            "demo",
            rule,
            severity,
            category,
            Location {
                file: "src/main.rs".to_string(),
                start_line: Some(1),
                start_column: None,
                end_line: None,
                end_column: None,
            },
            "message",
            None,
        )
    }

    #[test]
    fn zero_blocker_threshold_fails_on_one_blocker() {
        let analysis = Analysis::new(
            vec![finding("a", Severity::Blocker, Category::Security)],
            Vec::new(),
        );
        let policy = Policy::default_named("strict");
        let decision = GateEvaluator::evaluate(&analysis, &policy);
        assert!(!decision.passed());
        let GateDecision::Fail { policy, violations } = decision else {
            panic!("expected failure");
        };
        assert_eq!(policy, "strict");
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].condition, "blockers");
        assert_eq!(violations[0].actual, 1);
        assert_eq!(violations[0].responsible_findings.len(), 1);
    }

    #[test]
    fn category_threshold_fails() {
        let analysis = Analysis::new(
            vec![
                finding("a", Severity::Minor, Category::Security),
                finding("b", Severity::Minor, Category::Security),
            ],
            Vec::new(),
        );
        let mut categories = HashMap::new();
        categories.insert(Category::Security, 1u64);
        let policy = Policy {
            name: "cat".to_string(),
            categories,
            ..Policy::default_named("cat")
        };
        let decision = GateEvaluator::evaluate(&analysis, &policy);
        let GateDecision::Fail { violations, .. } = decision else {
            panic!("expected failure");
        };
        assert_eq!(violations[0].condition, "category:security");
        assert_eq!(violations[0].actual, 2);
    }

    #[test]
    fn within_thresholds_passes() {
        let analysis = Analysis::new(
            vec![finding("a", Severity::Minor, Category::Maintainability)],
            Vec::new(),
        );
        let policy = Policy::default_named("lenient");
        assert!(GateEvaluator::evaluate(&analysis, &policy).passed());
    }

    #[test]
    fn decision_is_deterministic() {
        let analysis = Analysis::new(
            vec![
                finding("a", Severity::Critical, Category::Security),
                finding("b", Severity::Major, Category::Reliability),
            ],
            Vec::new(),
        );
        let policy = Policy::default_named("d");
        let first = GateEvaluator::evaluate(&analysis, &policy);
        let second = GateEvaluator::evaluate(&analysis, &policy);
        assert_eq!(first, second);
    }

    #[test]
    fn resolver_falls_back_to_default() {
        let policies = HashMap::new();
        let resolved = PolicyResolver::resolve(&policies, None);
        assert_eq!(resolved.name, "default");
        let named = PolicyResolver::resolve(&policies, Some("missing"));
        assert_eq!(named.name, "missing");
        let mut policies = HashMap::new();
        policies.insert("strict".to_string(), Policy::default_named("strict"));
        let resolved = PolicyResolver::resolve(&policies, Some("strict"));
        assert_eq!(resolved.name, "strict");
    }
}
