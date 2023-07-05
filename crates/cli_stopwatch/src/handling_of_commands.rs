use core_stopwatch::{
    data_source::{self, DataErrorTimers},
    Stopwatch,
};

use crate::{
    cli_parse::{CliCommands, CliSubCommands},
    file_access, tables, AppResult,
};

pub fn handle_command(args: CliCommands) -> AppResult<String> {
    let path = file_access::get_path_to_data()?;
    let mut loaded_timers = data_source::load_timers_from(&path).or_else(|error| match error {
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
            data_source::save_timers_to(&path, loaded_timers)?;
            Ok("Stopwatch created".to_string())
        }
        CliSubCommands::Clear => {
            loaded_timers.clear();
            data_source::save_timers_to(&path, loaded_timers)?;
            Ok("All stopwatches removed".to_string())
        }
        CliSubCommands::List => {
            let table = tables::create_table_from_stopwatches(loaded_timers.to_stopwatches());
            Ok(table.to_string())
        }
    }
}
