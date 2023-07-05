#[macro_use]
extern crate log;

type CliError = anyhow::Error;
type AppResult<T = ()> = Result<T, anyhow::Error>;

mod file_access;
mod handling_of_commands;

mod cli_parse;
mod tables;

use clap::Parser;
use std::process::ExitCode;

fn main() -> ExitCode {
    file_access::handle_env_file();

    env_logger::init();

    let args = cli_parse::CliCommands::parse();

    match handling_of_commands::handle_command(args) {
        Ok(result) => println!("{}", result),
        Err(error) => {
            report_error(error);
            return ExitCode::FAILURE;
        }
    };

    ExitCode::SUCCESS
}

pub fn report_error(error: CliError) {
    eprintln!("{:?}", error);
}
