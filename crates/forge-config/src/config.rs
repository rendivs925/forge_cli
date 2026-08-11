use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// The only schema version Forge currently understands.
pub const SUPPORTED_SCHEMA_VERSION: u32 = 1;

/// Effective Forge configuration.
///
/// `schema` is required in a `forge.toml`; every other field falls back to the
/// built-in default when absent from a layer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForgeConfig {
    pub schema: u32,
    #[serde(default)]
    pub profile: Option<String>,
    #[serde(default)]
    pub offline: bool,
    #[serde(default)]
    pub no_cache: bool,
    #[serde(default)]
    pub fail_fast: bool,
    #[serde(default)]
    pub gate_policy: Option<String>,
    #[serde(default)]
    pub profiles: HashMap<String, ProfileConfig>,
    #[serde(default)]
    pub tools: HashMap<String, ToolConfig>,
    #[serde(default)]
    pub rules: HashMap<String, RuleConfig>,
    #[serde(default)]
    pub policies: HashMap<String, PolicyConfig>,
}

/// A named analysis profile selecting tools and execution bounds.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileConfig {
    pub tools: Vec<String>,
    #[serde(default = "default_concurrency")]
    pub concurrency: usize,
}

fn default_concurrency() -> usize {
    4
}

/// Configuration for an external tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolConfig {
    pub executable: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub timeout_secs: Option<u64>,
    #[serde(default)]
    pub version_command: Option<Vec<String>>,
    #[serde(default)]
    pub supported_version_range: Option<String>,
    #[serde(default = "default_true")]
    pub enabled: bool,
}

/// Per-rule enablement and severity override.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuleConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default)]
    pub severity: Option<String>,
}

/// A quality policy with severity and category thresholds.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyConfig {
    #[serde(default)]
    pub max_blockers: Option<u64>,
    #[serde(default)]
    pub max_critical: Option<u64>,
    #[serde(default)]
    pub max_major: Option<u64>,
    #[serde(default)]
    pub max_minor: Option<u64>,
    #[serde(default)]
    pub categories: HashMap<String, u64>,
}

fn default_true() -> bool {
    true
}

impl Default for ForgeConfig {
    fn default() -> Self {
        Self {
            schema: SUPPORTED_SCHEMA_VERSION,
            profile: None,
            offline: false,
            no_cache: false,
            fail_fast: false,
            gate_policy: None,
            profiles: HashMap::new(),
            tools: HashMap::new(),
            rules: HashMap::new(),
            policies: HashMap::new(),
        }
    }
}
