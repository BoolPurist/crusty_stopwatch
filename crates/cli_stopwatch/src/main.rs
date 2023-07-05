type CliError = anyhow::Error;
type AppResult<T = ()> = Result<T, anyhow::Error>;
use clap::Parser;
use cli_parse::{CliCommands, CliSubCommands};
use core_stopwatch::{DataErrorTimers, Stopwatch};
use log::info;
mod tables;
use std::{fs, path::PathBuf, process::ExitCode};
mod cli_parse;

fn main() -> ExitCode {
    handle_env_file();

    env_logger::init();

    let args = cli_parse::CliCommands::parse();

    match handle_command(args) {
        Ok(result) => println!("{}", result),
        Err(error) => {
            report_error(error);
            return ExitCode::FAILURE;
        }
    };

    ExitCode::SUCCESS
}

fn handle_env_file() {
    if cfg!(debug_assertions) {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(".env");
        dotenv::from_path(path.as_path()).expect("Unable to read .env file");
    }
}

fn get_path_to_data() -> AppResult<PathBuf> {
    if cfg!(debug_assertions) {
        let folder_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(".date_during_dev");
        let data_path = folder_path.join("data.ron");

        if !folder_path.exists() {
            fs::create_dir_all(&folder_path)?;
            info!(
                "Created directory at {:?} for loading data from",
                folder_path
            );
        }
        if !data_path.exists() {
            fs::write(&data_path, "")?;
            info!(
                "Created empty data at {:?} for loading data from",
                data_path
            );
        }

        Ok(data_path)
    } else {
        todo!("Not implemented for production");
    }
}

fn handle_command(args: CliCommands) -> AppResult<String> {
    let path = get_path_to_data()?;
    let mut loaded_timers =
        core_stopwatch::load_timers_from(&path).or_else(|error| match error {
            DataErrorTimers::Io(io_error) => Err(io_error),
            DataErrorTimers::Format(error) => {
                info!(
                    "Format invalid format. Starging with no timer.\n Error: {}",
                    error
                );
                Ok(Default::default())
            }
        })?;

    match args.sub_commands {
        CliSubCommands::Create(create) => {
            let mut new_stop_watch = Stopwatch::now();
            new_stop_watch.with_name(create.title);
            loaded_timers.add_new_stopwatch(new_stop_watch);
            core_stopwatch::save_timers_to(&path, loaded_timers)?;
            Ok("Stopwatch created".to_string())
        }
        CliSubCommands::Clear => {
            loaded_timers.clear();
            core_stopwatch::save_timers_to(&path, loaded_timers)?;
            Ok("All stopwatches removed".to_string())
        }
        CliSubCommands::List => {
            let table = tables::create_table_from_stopwatches(loaded_timers.to_stopwatches());
            Ok(table.to_string())
        }
    }
}

pub fn report_error(error: CliError) {
    eprintln!("{:?}", error);
}
