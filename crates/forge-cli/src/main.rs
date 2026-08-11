mod cli;
mod commands;
mod output;

use std::process::ExitCode as ProcessExitCode;

use clap::Parser;

use cli::Cli;

fn main() -> ProcessExitCode {
    let cli = Cli::parse();

    match commands::run(cli) {
        Ok(exit_code) => ProcessExitCode::from(exit_code.as_i32() as u8),
        Err(error) => {
            eprintln!("forge: {error}");
            ProcessExitCode::from(error.exit_code().as_i32() as u8)
        }
    }
}
