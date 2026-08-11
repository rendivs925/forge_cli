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
}

impl Default for ForgeConfig {
    fn default() -> Self {
        Self {
            schema: SUPPORTED_SCHEMA_VERSION,
            profile: None,
            offline: false,
            no_cache: false,
            fail_fast: false,
        }
    }
}
