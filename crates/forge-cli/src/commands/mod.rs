mod check;
mod config;
mod doctor;
mod gate;
mod init;
mod quality;
mod scan;
mod version;

use forge_core::{ExitCode, ForgeError};

use crate::cli::{Cli, Command};

pub fn run(cli: Cli) -> Result<ExitCode, ForgeError> {
    match cli.command {
        Command::Check => check::run(&cli.global),
        Command::Scan(args) => scan::run(&cli.global, &args),
        Command::Gate => gate::run(&cli.global),
        Command::Doctor => doctor::run(&cli.global),
        Command::Version => version::run(&cli.global),
        Command::Init => init::run(&cli.global),
        Command::Config(args) => config::run(&cli.global, &args),
        other => Err(ForgeError::Usage(format!(
            "command '{}' is not yet implemented",
            command_name(&other)
        ))),
    }
}

fn command_name(command: &Command) -> String {
    format!("{:?}", command)
        .split('(')
        .next()
        .unwrap_or("")
        .to_lowercase()
}
