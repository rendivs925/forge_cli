use std::path::PathBuf;

use crate::cli::GlobalArgs;

pub fn workspace_root(global: &GlobalArgs) -> PathBuf {
    global
        .workspace
        .clone()
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default())
}
