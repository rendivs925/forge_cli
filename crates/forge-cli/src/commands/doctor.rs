use std::path::{Path, PathBuf};

use forge_core::{ExitCode, ForgeError};
use serde::Serialize;

use crate::cli::{Format, GlobalArgs};
use crate::output;

#[derive(Debug, Serialize)]
pub struct CheckResult {
    pub name: &'static str,
    pub status: &'static str,
    pub detail: String,
}

pub fn run(global: &GlobalArgs) -> Result<ExitCode, ForgeError> {
    let root = workspace_root(global);
    let checks = [
        git_repository(&root),
        cargo_manifest(&root),
        forge_config(&root),
        cache_writable(),
    ];

    match global.format {
        Format::Terminal => {
            for check in &checks {
                output::write_terminal(&format!(
                    "[{}] {}: {}",
                    check.status, check.name, check.detail
                ))?;
            }
        }
        Format::Json => output::render_json(&checks)?,
    }

    let failed = checks.iter().any(|check| check.status == "fail");
    if failed {
        Ok(ExitCode::ToolExecution)
    } else {
        Ok(ExitCode::Success)
    }
}

fn workspace_root(global: &GlobalArgs) -> PathBuf {
    global
        .workspace
        .clone()
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default())
}

fn git_repository(root: &Path) -> CheckResult {
    let detail = if root.join(".git").is_dir() {
        "git repository detected".to_string()
    } else {
        "no .git directory found".to_string()
    };
    CheckResult {
        name: "repository",
        status: if detail.starts_with("no") {
            "warn"
        } else {
            "ok"
        },
        detail,
    }
}

fn cargo_manifest(root: &Path) -> CheckResult {
    let detail = if root.join("Cargo.toml").is_file() {
        "Cargo.toml found".to_string()
    } else {
        "no Cargo.toml found".to_string()
    };
    CheckResult {
        name: "project",
        status: if detail.starts_with("no") {
            "warn"
        } else {
            "ok"
        },
        detail,
    }
}

fn forge_config(root: &Path) -> CheckResult {
    let detail = if root.join("forge.toml").is_file() {
        "forge.toml found".to_string()
    } else {
        "no forge.toml found".to_string()
    };
    CheckResult {
        name: "config",
        status: if detail.starts_with("no") {
            "warn"
        } else {
            "ok"
        },
        detail,
    }
}

fn cache_writable() -> CheckResult {
    let cache_dir = std::env::temp_dir().join("forge-cache");
    let detail = match std::fs::create_dir_all(&cache_dir) {
        Ok(()) => "cache directory writable".to_string(),
        Err(error) => format!("cache directory not writable: {error}"),
    };
    CheckResult {
        name: "cache",
        status: if detail.starts_with("cache directory not") {
            "fail"
        } else {
            "ok"
        },
        detail,
    }
}
