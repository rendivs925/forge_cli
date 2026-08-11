use serde::{Deserialize, Serialize};

/// Availability and compatibility of an external tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "kebab-case")]
pub enum ToolStatus {
    Available {
        version: Option<String>,
    },
    Missing,
    Incompatible {
        expected: String,
        found: Option<String>,
    },
}

/// A configured external tool and its resolved state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    pub executable: String,
    pub supported_version_range: Option<String>,
    pub status: ToolStatus,
}

/// Discovers whether configured tools are available and compatible.
#[derive(Debug, Clone)]
pub struct ToolResolver;

impl ToolResolver {
    pub fn resolve(executable: &str) -> Option<String> {
        let path = std::env::var_os("PATH")?;
        let candidates: Vec<std::path::PathBuf> = if std::path::Path::new(executable).is_absolute()
        {
            vec![std::path::PathBuf::from(executable)]
        } else {
            std::env::split_paths(&path)
                .map(|dir| dir.join(executable))
                .collect()
        };
        candidates
            .into_iter()
            .find(|candidate| candidate.is_file())
            .map(|path| path.to_string_lossy().to_string())
    }

    /// Determine whether an executable is available and within the supported
    /// version range. Version comparison is coarse: only a plain ">=X" range
    /// prefix is honored; anything else treats the tool as available.
    pub fn check(
        executable: &str,
        supported_range: Option<&str>,
        version: Option<&str>,
    ) -> ToolStatus {
        if Self::resolve(executable).is_none() {
            return ToolStatus::Missing;
        }
        let Some(range) = supported_range else {
            return ToolStatus::Available {
                version: version.map(str::to_string),
            };
        };
        let Some(found) = version else {
            return ToolStatus::Incompatible {
                expected: range.to_string(),
                found: None,
            };
        };
        if range_matches(range, found) {
            ToolStatus::Available {
                version: Some(found.to_string()),
            }
        } else {
            ToolStatus::Incompatible {
                expected: range.to_string(),
                found: Some(found.to_string()),
            }
        }
    }

    pub fn status_label(status: &ToolStatus) -> &'static str {
        match status {
            ToolStatus::Available { .. } => "available",
            ToolStatus::Missing => "missing",
            ToolStatus::Incompatible { .. } => "incompatible",
        }
    }
}

fn range_matches(range: &str, version: &str) -> bool {
    if let Some(min) = range.strip_prefix(">=") {
        return compare_versions(version, min) >= 0;
    }
    true
}

fn compare_versions(a: &str, b: &str) -> i32 {
    let parse = |s: &str| -> Vec<u64> {
        s.split('.')
            .filter_map(|part| {
                part.chars()
                    .take_while(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse::<u64>()
                    .ok()
            })
            .collect()
    };
    let a_parts = parse(a);
    let b_parts = parse(b);
    for (x, y) in a_parts.iter().zip(b_parts.iter()) {
        if x != y {
            return (*x as i32) - (*y as i32);
        }
    }
    (a_parts.len() as i32) - (b_parts.len() as i32)
}

#[cfg(test)]
mod tests {
    use super::{ToolResolver, ToolStatus, compare_versions, range_matches};

    #[test]
    fn compare_versions_orders_correctly() {
        assert!(compare_versions("1.60.0", "1.60.0") == 0);
        assert!(compare_versions("2.0.0", "1.99.0") > 0);
        assert!(compare_versions("1.5", "1.50") < 0);
    }

    #[test]
    fn range_matches_plain_prefix() {
        assert!(range_matches(">=1.60", "1.61.0"));
        assert!(!range_matches(">=1.60", "1.59.0"));
        assert!(range_matches("any", "1.0"));
    }

    #[test]
    fn missing_tool_is_reported() {
        let status = ToolResolver::check("forge-tool-does-not-exist-xyz", None, None);
        assert_eq!(status, ToolStatus::Missing);
    }
}
