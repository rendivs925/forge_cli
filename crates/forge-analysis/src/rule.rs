use serde::{Deserialize, Serialize};

use crate::finding::{Category, Severity};

/// A rule with metadata and runtime override configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rule {
    pub id: String,
    pub name: String,
    pub category: Category,
    pub default_severity: Severity,
    pub description: String,
    pub applicability: String,
    pub enabled: bool,
}

impl Rule {
    pub fn new(
        id: &str,
        name: &str,
        category: Category,
        default_severity: Severity,
        description: &str,
        applicability: &str,
    ) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            category,
            default_severity,
            description: description.to_string(),
            applicability: applicability.to_string(),
            enabled: true,
        }
    }
}

/// Per-rule override applied from configuration before analysis.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct RuleOverride {
    pub enabled: Option<bool>,
    pub severity: Option<Severity>,
}

/// Registry of available rules with deterministic resolution.
///
/// Rules are independent units: enablement and severity overrides never
/// couple one rule's evaluation to another.
#[derive(Debug, Clone, Default)]
pub struct RuleRegistry {
    rules: Vec<Rule>,
}

impl RuleRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, rule: Rule) -> &mut Self {
        self.rules.push(rule);
        self
    }

    pub fn register_all(&mut self, rules: Vec<Rule>) -> &mut Self {
        self.rules.extend(rules);
        self
    }

    pub fn all(&self) -> &[Rule] {
        &self.rules
    }

    pub fn get(&self, id: &str) -> Option<&Rule> {
        self.rules.iter().find(|rule| rule.id == id)
    }

    /// Return the rules enabled after applying the given overrides.
    ///
    /// Deterministic: registry order is preserved.
    pub fn enabled_rules(
        &self,
        overrides: &std::collections::HashMap<String, RuleOverride>,
    ) -> Vec<Rule> {
        self.rules
            .iter()
            .map(|rule| {
                let mut resolved = rule.clone();
                if let Some(over) = overrides.get(&rule.id) {
                    if let Some(enabled) = over.enabled {
                        resolved.enabled = enabled;
                    }
                    if let Some(severity) = over.severity {
                        resolved.default_severity = severity;
                    }
                }
                resolved
            })
            .filter(|rule| rule.enabled)
            .collect()
    }
}

/// A reusable collection of rules selected as a unit.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RulePack {
    pub id: String,
    pub rule_ids: Vec<String>,
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{Rule, RuleOverride, RuleRegistry};
    use crate::finding::{Category, Severity};

    fn sample_rules() -> Vec<Rule> {
        vec![
            Rule::new(
                "security.sql-injection",
                "SQL injection",
                Category::Security,
                Severity::Critical,
                "Prevent SQL injection",
                "databases",
            ),
            Rule::new(
                "maintainability.long-function",
                "Long function",
                Category::Maintainability,
                Severity::Minor,
                "Keep functions short",
                "all",
            ),
        ]
    }

    #[test]
    fn registry_resolves_rules_deterministically() {
        let mut registry = RuleRegistry::new();
        registry.register_all(sample_rules());
        let overrides = HashMap::new();
        let enabled = registry.enabled_rules(&overrides);
        assert_eq!(enabled.len(), 2);
        assert_eq!(enabled[0].id, "security.sql-injection");
        assert_eq!(enabled[1].id, "maintainability.long-function");
    }

    #[test]
    fn disabled_rule_is_excluded() {
        let mut registry = RuleRegistry::new();
        registry.register_all(sample_rules());
        let mut overrides = HashMap::new();
        overrides.insert(
            "security.sql-injection".to_string(),
            RuleOverride {
                enabled: Some(false),
                severity: None,
            },
        );
        let enabled = registry.enabled_rules(&overrides);
        assert_eq!(enabled.len(), 1);
        assert_eq!(enabled[0].id, "maintainability.long-function");
    }

    #[test]
    fn severity_override_applies() {
        let mut registry = RuleRegistry::new();
        registry.register_all(sample_rules());
        let mut overrides = HashMap::new();
        overrides.insert(
            "maintainability.long-function".to_string(),
            RuleOverride {
                enabled: None,
                severity: Some(Severity::Critical),
            },
        );
        let enabled = registry.enabled_rules(&overrides);
        assert_eq!(enabled[0].default_severity, Severity::Critical);
    }
}
