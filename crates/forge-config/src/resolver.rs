use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::config::{ForgeConfig, SUPPORTED_SCHEMA_VERSION};
use crate::error::ConfigError;

const CONFIG_KEYS: [&str; 5] = ["schema", "profile", "offline", "no_cache", "fail_fast"];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Layer {
    Defaults,
    Global,
    Project,
    Cli,
}

impl Layer {
    pub fn label(self) -> &'static str {
        match self {
            Self::Defaults => "built-in defaults",
            Self::Global => "global user config",
            Self::Project => "project config",
            Self::Cli => "command-line flags",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResolvedConfig {
    pub config: ForgeConfig,
    pub provenance: HashMap<String, Vec<LayerSource>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LayerSource {
    pub layer: Layer,
    pub path: Option<PathBuf>,
}

#[derive(Debug, Clone, Default)]
pub struct CliOverrides {
    pub profile: Option<String>,
    pub offline: Option<bool>,
    pub no_cache: Option<bool>,
    pub fail_fast: Option<bool>,
}

pub struct ConfigResolver {
    workspace: PathBuf,
    config_path: Option<PathBuf>,
    cli: CliOverrides,
}

impl ConfigResolver {
    pub fn new(workspace: PathBuf, config_path: Option<PathBuf>, cli: CliOverrides) -> Self {
        Self {
            workspace,
            config_path,
            cli,
        }
    }

    pub fn resolve(&self) -> Result<ResolvedConfig, ConfigError> {
        let mut config = ForgeConfig::default();
        let mut provenance: HashMap<String, Vec<LayerSource>> = HashMap::new();
        for key in CONFIG_KEYS {
            provenance.insert(
                key.to_string(),
                vec![LayerSource {
                    layer: Layer::Defaults,
                    path: None,
                }],
            );
        }

        if let Some(global) = self.global_path().filter(|p| p.is_file()) {
            let raw = load_file(&global)?;
            apply_layer(
                &raw,
                &mut config,
                &mut provenance,
                LayerSource {
                    layer: Layer::Global,
                    path: Some(global),
                },
            );
        }

        let project = self.project_path();
        if let Some(ref path) = project {
            if path.is_file() {
                let raw = load_file(path)?;
                apply_layer(
                    &raw,
                    &mut config,
                    &mut provenance,
                    LayerSource {
                        layer: Layer::Project,
                        path: Some(path.clone()),
                    },
                );
            } else if self.config_path.is_some() {
                return Err(ConfigError::Io {
                    path: path.clone(),
                    source: std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "configuration file not found",
                    ),
                });
            }
        }

        apply_cli(&self.cli, &mut config, &mut provenance);

        validate(&config)?;
        Ok(ResolvedConfig { config, provenance })
    }

    fn global_path(&self) -> Option<PathBuf> {
        let home = env::var_os("HOME")?;
        Some(
            PathBuf::from(home)
                .join(".config")
                .join("forge")
                .join("forge.toml"),
        )
    }

    fn project_path(&self) -> Option<PathBuf> {
        if let Some(ref path) = self.config_path {
            return Some(path.clone());
        }
        let path = self.workspace.join("forge.toml");
        Some(path)
    }
}

fn load_file(path: &Path) -> Result<RawConfig, ConfigError> {
    let content = fs::read_to_string(path).map_err(|source| ConfigError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    let raw: RawConfig = toml::from_str(&content).map_err(|e| ConfigError::Parse {
        path: path.to_path_buf(),
        message: e.to_string(),
    })?;
    if let Some(schema) = raw.schema {
        if schema != SUPPORTED_SCHEMA_VERSION {
            return Err(ConfigError::UnsupportedSchema {
                found: schema,
                path: path.to_path_buf(),
            });
        }
    } else {
        return Err(ConfigError::Parse {
            path: path.to_path_buf(),
            message: "missing required key 'schema'".to_string(),
        });
    }
    Ok(raw)
}

#[derive(Debug, Clone, Deserialize)]
struct RawConfig {
    schema: Option<u32>,
    #[serde(default)]
    profile: Option<String>,
    #[serde(default)]
    offline: Option<bool>,
    #[serde(default)]
    no_cache: Option<bool>,
    #[serde(default)]
    fail_fast: Option<bool>,
}

fn apply_layer(
    raw: &RawConfig,
    config: &mut ForgeConfig,
    provenance: &mut HashMap<String, Vec<LayerSource>>,
    layer: LayerSource,
) {
    if let Some(schema) = raw.schema {
        config.schema = schema;
        provenance
            .entry("schema".to_string())
            .or_default()
            .push(layer.clone());
    }
    if let Some(profile) = &raw.profile {
        config.profile = Some(profile.clone());
        provenance
            .entry("profile".to_string())
            .or_default()
            .push(layer.clone());
    }
    if let Some(offline) = raw.offline {
        config.offline = offline;
        provenance
            .entry("offline".to_string())
            .or_default()
            .push(layer.clone());
    }
    if let Some(no_cache) = raw.no_cache {
        config.no_cache = no_cache;
        provenance
            .entry("no_cache".to_string())
            .or_default()
            .push(layer.clone());
    }
    if let Some(fail_fast) = raw.fail_fast {
        config.fail_fast = fail_fast;
        provenance
            .entry("fail_fast".to_string())
            .or_default()
            .push(layer.clone());
    }
}

fn apply_cli(
    cli: &CliOverrides,
    config: &mut ForgeConfig,
    provenance: &mut HashMap<String, Vec<LayerSource>>,
) {
    let layer = LayerSource {
        layer: Layer::Cli,
        path: None,
    };
    if let Some(profile) = &cli.profile {
        config.profile = Some(profile.clone());
        provenance
            .entry("profile".to_string())
            .or_default()
            .push(layer.clone());
    }
    if let Some(offline) = cli.offline {
        config.offline = offline;
        provenance
            .entry("offline".to_string())
            .or_default()
            .push(layer.clone());
    }
    if let Some(no_cache) = cli.no_cache {
        config.no_cache = no_cache;
        provenance
            .entry("no_cache".to_string())
            .or_default()
            .push(layer.clone());
    }
    if let Some(fail_fast) = cli.fail_fast {
        config.fail_fast = fail_fast;
        provenance
            .entry("fail_fast".to_string())
            .or_default()
            .push(layer.clone());
    }
}

fn validate(config: &ForgeConfig) -> Result<(), ConfigError> {
    if config.schema != SUPPORTED_SCHEMA_VERSION {
        return Err(ConfigError::Invalid {
            key: "schema".to_string(),
            message: format!(
                "unsupported schema version {} (supported: {})",
                config.schema, SUPPORTED_SCHEMA_VERSION
            ),
        });
    }
    if let Some(profile) = &config.profile && profile.trim().is_empty() {
        return Err(ConfigError::Invalid {
            key: "profile".to_string(),
            message: "profile must not be empty".to_string(),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unique_temp_dir(tag: &str) -> std::io::Result<std::path::PathBuf> {
        let dir =
            std::env::temp_dir().join(format!("forge-config-test-{}-{}", std::process::id(), tag));
        std::fs::create_dir_all(&dir)?;
        Ok(dir)
    }

    fn write_config(path: &Path, content: &str) -> std::io::Result<()> {
        std::fs::write(path, content)?;
        Ok(())
    }

    #[test]
    fn defaults_when_no_files() {
        let resolver = ConfigResolver::new(
            unique_temp_dir("defaults").unwrap(),
            None,
            CliOverrides::default(),
        );
        let resolved = resolver.resolve().unwrap();
        assert_eq!(resolved.config.schema, 1);
        assert_eq!(resolved.config.profile, None);
        assert!(!resolved.config.offline);
        assert!(!resolved.config.no_cache);
        assert!(!resolved.config.fail_fast);
        assert_eq!(
            resolved.provenance["schema"],
            vec![LayerSource {
                layer: Layer::Defaults,
                path: None
            }]
        );
    }

    #[test]
    fn project_config_overrides_defaults() {
        let dir = unique_temp_dir("project").unwrap();
        let path = dir.join("forge.toml");
        write_config(&path, "schema = 1\nprofile = \"custom\"\n").unwrap();
        let resolver = ConfigResolver::new(dir, None, CliOverrides::default());
        let resolved = resolver.resolve().unwrap();
        assert_eq!(resolved.config.profile, Some("custom".to_string()));
        assert_eq!(
            resolved.provenance["profile"],
            vec![
                LayerSource {
                    layer: Layer::Defaults,
                    path: None
                },
                LayerSource {
                    layer: Layer::Project,
                    path: Some(path.clone())
                }
            ]
        );
    }

    #[test]
    fn missing_schema_is_error() {
        let dir = unique_temp_dir("missing-schema").unwrap();
        let path = dir.join("forge.toml");
        write_config(&path, "profile = \"custom\"\n").unwrap();
        let resolver = ConfigResolver::new(dir, None, CliOverrides::default());
        let err = resolver.resolve().unwrap_err();
        assert!(matches!(err, ConfigError::Parse { .. }));
    }

    #[test]
    fn unsupported_schema_is_error() {
        let dir = unique_temp_dir("bad-schema").unwrap();
        let path = dir.join("forge.toml");
        write_config(&path, "schema = 99\n").unwrap();
        let resolver = ConfigResolver::new(dir, None, CliOverrides::default());
        let err = resolver.resolve().unwrap_err();
        assert!(matches!(err, ConfigError::UnsupportedSchema { .. }));
    }

    #[test]
    fn cli_overrides_project() {
        let dir = unique_temp_dir("cli-override").unwrap();
        let path = dir.join("forge.toml");
        write_config(&path, "schema = 1\nprofile = \"custom\"\n").unwrap();
        let cli = CliOverrides {
            profile: Some("cli-profile".to_string()),
            offline: Some(true),
            ..Default::default()
        };
        let resolver = ConfigResolver::new(dir, None, cli);
        let resolved = resolver.resolve().unwrap();
        assert_eq!(resolved.config.profile, Some("cli-profile".to_string()));
        assert_eq!(
            resolved.provenance["profile"],
            vec![
                LayerSource {
                    layer: Layer::Defaults,
                    path: None
                },
                LayerSource {
                    layer: Layer::Project,
                    path: Some(path.clone())
                },
                LayerSource {
                    layer: Layer::Cli,
                    path: None
                }
            ]
        );
    }

    #[test]
    fn explicit_missing_config_is_error() {
        let dir = unique_temp_dir("missing-file").unwrap();
        let resolver = ConfigResolver::new(
            dir.clone(),
            Some(dir.join("nonexistent.toml")),
            CliOverrides::default(),
        );
        let err = resolver.resolve().unwrap_err();
        assert!(matches!(err, ConfigError::Io { .. }));
    }
}
