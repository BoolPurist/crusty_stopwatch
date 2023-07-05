use core_stopwatch::{formating, Stopwatch, Timer};
use prettytable::{row, Table};

pub fn create_table_from_stopwatches(stop_watches: impl IntoIterator<Item = Stopwatch>) -> Table {
    let mut table = Table::new();
    table.add_row(row!["Title", "Duration", "Started"]);

    for next in stop_watches {
        let title = next.title().clone().unwrap_or_default();
        let elapsed = formating::duration_to_row(next.elapsed());
        let started = formating::date_to_row(next.started());
        table.add_row(row![title, elapsed, started]);
    }

    table
}
