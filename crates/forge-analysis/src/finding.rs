use serde::{Deserialize, Serialize};

/// Normalized finding severity, independent of the source analyzer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Severity {
    Blocker,
    Critical,
    Major,
    Minor,
    Info,
}

impl Severity {
    /// Rank used for severity comparisons; lower rank is more severe.
    pub fn rank(self) -> u8 {
        match self {
            Self::Blocker => 0,
            Self::Critical => 1,
            Self::Major => 2,
            Self::Minor => 3,
            Self::Info => 4,
        }
    }
}

/// Normalized finding category shared across analyzers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Category {
    Security,
    Reliability,
    Maintainability,
    Architecture,
    Performance,
    SupplyChain,
}

impl Category {
    pub fn label(self) -> &'static str {
        match self {
            Self::Security => "security",
            Self::Reliability => "reliability",
            Self::Maintainability => "maintainability",
            Self::Architecture => "architecture",
            Self::Performance => "performance",
            Self::SupplyChain => "supply chain",
        }
    }
}

/// Source location of a finding.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Location {
    /// Path relative to the workspace root.
    pub file: String,
    pub start_line: Option<u32>,
    pub start_column: Option<u32>,
    pub end_line: Option<u32>,
    pub end_column: Option<u32>,
}

/// A normalized finding produced by any analyzer.
///
/// `id` is a stable fingerprint derived from the analyzer, rule, location, and
/// message so that the same logical finding keeps its identity across runs.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Finding {
    pub id: String,
    pub analyzer_id: String,
    pub rule_id: String,
    pub severity: Severity,
    pub category: Category,
    pub location: Location,
    pub message: String,
    pub remediation: Option<String>,
}

impl Finding {
    pub fn new(
        analyzer_id: &str,
        rule_id: &str,
        severity: Severity,
        category: Category,
        location: Location,
        message: &str,
        remediation: Option<&str>,
    ) -> Self {
        let id = format!(
            "{}:{}:{}:{}:{}:{}",
            analyzer_id,
            rule_id,
            location.file,
            location.start_line.unwrap_or(0),
            location.start_column.unwrap_or(0),
            message
        );
        Self {
            id,
            analyzer_id: analyzer_id.to_string(),
            rule_id: rule_id.to_string(),
            severity,
            category,
            location,
            message: message.to_string(),
            remediation: remediation.map(str::to_string),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Category, Finding, Location, Severity};

    fn sample_location() -> Location {
        Location {
            file: "src/main.rs".to_string(),
            start_line: Some(10),
            start_column: Some(5),
            end_line: Some(10),
            end_column: Some(20),
        }
    }

    #[test]
    fn finding_id_is_stable() {
        let a = Finding::new(
            "demo",
            "test.rule",
            Severity::Major,
            Category::Maintainability,
            sample_location(),
            "some message",
            Some("fix it"),
        );
        let b = Finding::new(
            "demo",
            "test.rule",
            Severity::Major,
            Category::Maintainability,
            sample_location(),
            "some message",
            Some("fix it"),
        );
        assert_eq!(a.id, b.id);
    }

    #[test]
    fn finding_id_changes_with_location() {
        let a = Finding::new(
            "demo",
            "test.rule",
            Severity::Major,
            Category::Maintainability,
            sample_location(),
            "some message",
            None,
        );
        let mut moved = sample_location();
        moved.start_line = Some(11);
        let b = Finding::new(
            "demo",
            "test.rule",
            Severity::Major,
            Category::Maintainability,
            moved,
            "some message",
            None,
        );
        assert_ne!(a.id, b.id);
    }
}
