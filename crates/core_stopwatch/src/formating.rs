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

#[cfg(test)]
mod testing {
    use chrono::Utc;

    use super::*;

    #[test]
    fn date_to_right_row() {
        let date = Utc.with_ymd_and_hms(2022, 10, 1, 11, 2, 9).unwrap();
        let actual = date_to_row(date);
        assert_eq!("2022.10.01 11:02:09", actual.as_str());
    }

    #[test]
    fn duration_to_right_row() {
        let date = Duration::hours(2) + Duration::minutes(24) + Duration::seconds(4);
        let actual = duration_to_row(date);
        assert_eq!("02:24:04", actual.as_str());
    }
}
