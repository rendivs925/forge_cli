use forge_core::{ExitCode, ForgeContext, ForgeError};
use serde::Serialize;

use crate::cli::{Format, GlobalArgs};
use crate::output;

#[derive(Debug, Serialize)]
struct VersionInfo<'a> {
    name: &'a str,
    version: &'a str,
}

pub fn run(global: &GlobalArgs) -> Result<ExitCode, ForgeError> {
    let context = ForgeContext::new();
    let info = VersionInfo {
        name: &context.name,
        version: &context.version,
    };
    match global.format {
        Format::Terminal => output::write_terminal(&format!("{} {}", info.name, info.version))?,
        Format::Json => output::render_json(&info)?,
    }
    Ok(ExitCode::Success)
}
