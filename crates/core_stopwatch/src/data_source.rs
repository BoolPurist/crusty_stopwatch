use ron::ser::PrettyConfig;
use std::{fs, io, path::Path};
use thiserror::Error;

use crate::Timers;

#[derive(Debug, Error)]
pub enum DataErrorTimers {
    #[error("Io: {}", 0)]
    Io(#[from] io::Error),
    #[error("Invalid format: {}", 0)]
    Format(String),
}

pub fn load_timers_from(path: &Path) -> Result<Timers, DataErrorTimers> {
    let content = fs::read_to_string(path)?;
    ron::from_str(&content).map_err(|error| DataErrorTimers::Format(error.to_string()))
}

pub fn save_timers_to(path: &Path, to_save: Timers) -> Result<(), DataErrorTimers> {
    let to_save = if cfg!(debug_assertions) {
        ron::ser::to_string_pretty::<Timers>(&to_save, PrettyConfig::default())
    } else {
        ron::to_string::<Timers>(&to_save)
    };

    let to_save = to_save.map_err(|error| DataErrorTimers::Format(error.to_string()))?;
    fs::write(path, to_save)?;
    Ok(())
}

#[cfg(test)]
mod testing {
    use chrono::{Duration, TimeZone, Utc};
    use ron::ser::PrettyConfig;

    use crate::Stopwatch;

    use super::*;

    #[test]
    fn serilize_stopwatches() {
        let start = Utc.with_ymd_and_hms(2023, 2, 1, 10, 10, 10).unwrap();
        let after_one_day = start + Duration::days(1);
        let input = Timers::new(vec![
            Stopwatch::from_on(None, start),
            Stopwatch::from_on(None, after_one_day),
        ]);
        let output = ron::ser::to_string_pretty(&input, PrettyConfig::default()).unwrap();

        insta::assert_display_snapshot!(output);
    }
}
