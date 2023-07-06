mod cli_parse;
mod handling_of_commands;
mod tables;

use clap::Parser;
use core_stopwatch::file_access;
use core_stopwatch::prelude::*;
use std::process::ExitCode;

pub const PROJECT_ROOT: &str = env!("CARGO_MANIFEST_DIR");

fn main() -> ExitCode {
    file_access::handle_env_file(PROJECT_ROOT);

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

pub fn report_error(error: AppError) {
    eprintln!("{:?}", error);
}
