use forge_core::ForgeError;
use serde::Serialize;

use crate::cli::{Format, GlobalArgs};
use crate::output;

#[derive(Debug, Serialize)]
pub struct QualityResult {
    pub command: &'static str,
    pub status: &'static str,
    pub findings: usize,
    pub message: String,
}

pub fn report(global: &GlobalArgs, result: &QualityResult) -> Result<(), ForgeError> {
    match global.format {
        Format::Terminal => output::write_terminal(&format!(
            "{}: {}\n{}",
            result.command, result.status, result.message
        )),
        Format::Json => output::render_json(result),
    }
}
