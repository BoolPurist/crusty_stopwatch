pub mod formating;
pub mod now_provider;

mod chrono_duration_serde;
mod stopwatch;
mod timer;

use std::{fs, io, path::Path};

pub use now_provider::{ChronoNow, NowProvider};
use ron::ser::PrettyConfig;
pub use stopwatch::Stopwatch;
pub use timer::Timer;

use derive_new::new;
use serde::{Deserialize, Serialize};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DataErrorTimers {
    #[error("Io: {}", 0)]
    Io(#[from] io::Error),
    #[error("Invalid format: {}", 0)]
    Format(String),
}

#[derive(Default, new, Serialize, Deserialize)]
pub struct Timers {
    stopwatches: Vec<Stopwatch<ChronoNow>>,
}

impl Timers {
    pub fn add_new_stopwatch(&mut self, new_stop_watch: Stopwatch<ChronoNow>) {
        self.stopwatches.push(new_stop_watch);
    }
    pub fn to_stopwatches(self) -> Vec<Stopwatch<ChronoNow>> {
        self.stopwatches
    }

    pub fn clear(&mut self) {
        self.stopwatches.clear();
    }
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
