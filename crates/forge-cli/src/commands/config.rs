use forge_config::{CliOverrides, ConfigResolver};
use forge_core::{ExitCode, ForgeError};
use serde::Serialize;

use crate::cli::{ConfigArgs, ConfigCommand, Format, GlobalArgs};
use crate::output;
use crate::workspace;

pub fn run(global: &GlobalArgs, args: &ConfigArgs) -> Result<ExitCode, ForgeError> {
    match &args.command {
        ConfigCommand::Show => show(global),
        ConfigCommand::Explain { key } => explain(global, key),
    }
}

pub fn resolve(global: &GlobalArgs) -> Result<forge_config::ResolvedConfig, ForgeError> {
    let root = workspace::workspace_root(global);
    let cli = CliOverrides {
        profile: global.profile.clone(),
        offline: global.offline,
        no_cache: global.no_cache,
        fail_fast: global.fail_fast,
    };
    let resolver = ConfigResolver::new(root, global.config.clone(), cli);
    resolver.resolve().map_err(ForgeError::from)
}

fn show(global: &GlobalArgs) -> Result<ExitCode, ForgeError> {
    let resolved = resolve(global)?;
    match global.format {
        Format::Terminal => {
            let profile = resolved.config.profile.as_deref().unwrap_or("<unset>");
            output::write_terminal(&format!(
                "schema = {}\nprofile = {}\noffline = {}\nno_cache = {}\nfail_fast = {}",
                resolved.config.schema,
                profile,
                resolved.config.offline,
                resolved.config.no_cache,
                resolved.config.fail_fast
            ))?;
        }
        Format::Json => {
            output::render_json(&resolved.config)?;
        }
    }
    Ok(ExitCode::Success)
}

#[derive(Serialize)]
struct ExplainOutput {
    key: String,
    layers: Vec<LayerInfo>,
}

#[derive(Serialize)]
struct LayerInfo {
    kind: String,
    path: Option<String>,
}

fn explain(global: &GlobalArgs, key: &str) -> Result<ExitCode, ForgeError> {
    let resolved = resolve(global)?;
    let layers = resolved
        .provenance
        .get(key)
        .ok_or_else(|| ForgeError::Usage(format!("unknown configuration key: {key}")))?;
    match global.format {
        Format::Terminal => {
            let lines: Vec<String> = layers
                .iter()
                .map(|source| {
                    let loc = source
                        .path
                        .as_ref()
                        .map(|p| format!(" ({})", p.display()))
                        .unwrap_or_default();
                    format!("  - {}{}", source.layer.label(), loc)
                })
                .collect();
            output::write_terminal(&format!("{}:\n{}", key, lines.join("\n")))?;
        }
        Format::Json => {
            let output = ExplainOutput {
                key: key.to_string(),
                layers: layers
                    .iter()
                    .map(|source| LayerInfo {
                        kind: source.layer.label().to_string(),
                        path: source.path.as_ref().map(|p| p.display().to_string()),
                    })
                    .collect(),
            };
            output::render_json(&output)?;
        }
    }
    Ok(ExitCode::Success)
}
