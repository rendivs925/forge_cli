use std::process::{Command, Output};

fn forge() -> Command {
    Command::new(env!("CARGO_BIN_EXE_forge"))
}

fn run_with_home(home: &std::path::Path, args: &[&str]) -> std::io::Result<Output> {
    forge().env("HOME", home).args(args).output()
}

fn run(args: &[&str]) -> std::io::Result<Output> {
    run_with_home(&std::path::PathBuf::from("/nonexistent"), args)
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn exit_code(output: &Output) -> i32 {
    output.status.code().unwrap_or(-1)
}

fn unique_dir(tag: &str) -> std::io::Result<std::path::PathBuf> {
    let dir = std::env::temp_dir().join(format!("forge-cli-test-{}-{}", std::process::id(), tag));
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

fn write_config(path: &std::path::Path, content: &str) -> std::io::Result<()> {
    std::fs::write(path, content)?;
    Ok(())
}

#[test]
fn help_exits_zero_and_lists_commands() -> std::io::Result<()> {
    let output = run(&["--help"])?;
    let help = stdout(&output);
    assert_eq!(exit_code(&output), 0);
    for command in ["check", "scan", "gate", "doctor", "version"] {
        assert!(help.contains(command), "help missing {command}");
    }
    Ok(())
}

#[test]
fn version_prints_name_and_version() -> std::io::Result<()> {
    let output = run(&["version"])?;
    let stdout_text = stdout(&output);
    assert_eq!(exit_code(&output), 0);
    assert!(stdout_text.starts_with("forge "));
    Ok(())
}

#[test]
fn check_exits_success() -> std::io::Result<()> {
    let output = run(&["check"])?;
    assert_eq!(exit_code(&output), 0);
    Ok(())
}

#[test]
fn scan_exits_success() -> std::io::Result<()> {
    let output = run(&["scan"])?;
    assert_eq!(exit_code(&output), 0);
    Ok(())
}

#[test]
fn gate_exits_success() -> std::io::Result<()> {
    let output = run(&["gate"])?;
    assert_eq!(exit_code(&output), 0);
    Ok(())
}

#[test]
fn doctor_exits_success() -> std::io::Result<()> {
    let output = run(&["doctor"])?;
    assert_eq!(exit_code(&output), 0);
    Ok(())
}

#[test]
fn json_output_is_valid_json() -> std::io::Result<()> {
    let output = run(&["check", "--format", "json"])?;
    let stdout_text = stdout(&output);
    assert_eq!(exit_code(&output), 0);
    let value: serde_json::Value =
        serde_json::from_str(&stdout_text).map_err(std::io::Error::other)?;
    assert_eq!(value["command"], "check");
    Ok(())
}

#[test]
fn invalid_command_exits_with_usage_code() -> std::io::Result<()> {
    let output = run(&["no-such-command"])?;
    assert_eq!(exit_code(&output), 2);
    Ok(())
}

#[test]
fn missing_subcommand_exits_with_usage_code() -> std::io::Result<()> {
    let output = run(&[])?;
    assert_eq!(exit_code(&output), 2);
    Ok(())
}

#[test]
fn config_show_exits_zero() -> std::io::Result<()> {
    let dir = unique_dir("show-zero")?;
    let output = run_with_home(&dir, &["config", "show"])?;
    assert_eq!(exit_code(&output), 0);
    let text = stdout(&output);
    assert!(text.contains("schema = 1"), "expected schema = 1 in output");
    Ok(())
}

#[test]
fn config_show_json_is_valid_json() -> std::io::Result<()> {
    let dir = unique_dir("show-json")?;
    let output = run_with_home(&dir, &["config", "show", "--format", "json"])?;
    assert_eq!(exit_code(&output), 0);
    let value: serde_json::Value =
        serde_json::from_str(&stdout(&output)).map_err(std::io::Error::other)?;
    assert_eq!(value.get("schema"), Some(&1.into()));
    Ok(())
}

#[test]
fn config_show_loads_project_config() -> std::io::Result<()> {
    let dir = unique_dir("show-project")?;
    let config_path = dir.join("forge.toml");
    write_config(&config_path, "schema = 1\nprofile = \"custom\"\n")?;
    let output = run(&[
        "config",
        "show",
        "--config",
        config_path
            .to_str()
            .ok_or_else(|| std::io::Error::other("non-utf8 path"))?,
        "--workspace",
        dir.to_str()
            .ok_or_else(|| std::io::Error::other("non-utf8 path"))?,
    ])?;
    assert_eq!(exit_code(&output), 0);
    let text = stdout(&output);
    assert!(
        text.contains("custom"),
        "expected 'custom' profile in output"
    );
    Ok(())
}

#[test]
fn config_explain_reports_contributing_layers() -> std::io::Result<()> {
    let dir = unique_dir("explain-layers")?;
    let config_path = dir.join("forge.toml");
    write_config(&config_path, "schema = 1\nprofile = \"custom\"\n")?;
    let output = run(&[
        "config",
        "explain",
        "profile",
        "--config",
        config_path
            .to_str()
            .ok_or_else(|| std::io::Error::other("non-utf8 path"))?,
    ])?;
    assert_eq!(exit_code(&output), 0);
    let text = stdout(&output);
    assert!(
        text.contains("built-in defaults"),
        "expected defaults layer"
    );
    assert!(text.contains("project config"), "expected project layer");
    Ok(())
}

#[test]
fn config_explain_unknown_key_exits_usage() -> std::io::Result<()> {
    let output = run(&["config", "explain", "nonexistent-key"])?;
    assert_eq!(exit_code(&output), 2);
    Ok(())
}

#[test]
fn invalid_config_schema_exits_config_error() -> std::io::Result<()> {
    let dir = unique_dir("bad-schema")?;
    let config_path = dir.join("forge.toml");
    write_config(&config_path, "schema = 99\n")?;
    let output = run(&[
        "config",
        "show",
        "--config",
        config_path
            .to_str()
            .ok_or_else(|| std::io::Error::other("non-utf8 path"))?,
    ])?;
    assert_eq!(exit_code(&output), 2);
    Ok(())
}

#[test]
fn missing_explicit_config_exits_config_error() -> std::io::Result<()> {
    let dir = unique_dir("missing-file")?;
    let missing = dir.join("nonexistent.toml");
    let output = run(&[
        "config",
        "show",
        "--config",
        missing
            .to_str()
            .ok_or_else(|| std::io::Error::other("non-utf8 path"))?,
    ])?;
    assert_eq!(exit_code(&output), 2);
    Ok(())
}
