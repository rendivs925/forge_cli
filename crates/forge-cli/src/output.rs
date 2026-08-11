use std::io::Write;

use forge_core::ForgeError;
use serde::Serialize;

pub fn render_json<T: Serialize>(value: &T) -> Result<(), ForgeError> {
    let json = serde_json::to_string(value)
        .map_err(|error| ForgeError::Internal(format!("failed to serialize output: {error}")))?;
    println!("{json}");
    Ok(())
}

pub fn write_terminal(message: &str) -> Result<(), ForgeError> {
    let mut stdout = std::io::stdout();
    stdout
        .write_all(message.as_bytes())
        .and_then(|_| stdout.write_all(b"\n"))
        .and_then(|_| stdout.flush())
        .map_err(ForgeError::from)
}
