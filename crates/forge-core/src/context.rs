/// Application context shared across commands.
#[derive(Debug, Clone)]
pub struct ForgeContext {
    pub name: String,
    pub version: String,
}

impl ForgeContext {
    pub fn new() -> Self {
        Self {
            name: "forge".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

impl Default for ForgeContext {
    fn default() -> Self {
        Self::new()
    }
}
