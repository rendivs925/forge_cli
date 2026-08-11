use std::process::{Command, Output};

fn forge() -> Command {
    Command::new(env!("CARGO_BIN_EXE_forge"))
}

fn run(args: &[&str]) -> std::io::Result<Output> {
    forge().args(args).output()
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn exit_code(output: &Output) -> i32 {
    output.status.code().unwrap_or(-1)
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
