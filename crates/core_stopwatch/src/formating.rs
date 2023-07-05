use chrono::{DateTime, Datelike, Duration, TimeZone, Timelike};

pub fn date_to_row<T: TimeZone>(date: DateTime<T>) -> String {
    format!(
        "{:04}.{:02}.{:02} {:02}:{:02}:{:02}",
        date.year(),
        date.month(),
        date.day(),
        date.hour(),
        date.minute(),
        date.second(),
    )
}

pub fn duration_to_row(to_string: Duration) -> String {
    let total_seconds = to_string.num_seconds();
    let (total_seconds, seconds) = (total_seconds / 60, total_seconds % 60);
    let (hours, minutes) = (total_seconds / 60, total_seconds % 60);
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}
