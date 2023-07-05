pub mod now_provider;

mod chrono_duration_serde;
mod stopwatch;
mod timer;

pub use now_provider::{ChronoNow, NowProvider};
pub use stopwatch::Stopwatch;
pub use timer::Timer;

use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(new, Serialize, Deserialize)]
pub struct LoadedTimers {
    stopwatches: Vec<Stopwatch<ChronoNow>>,
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
        let input = LoadedTimers::new(vec![
            Stopwatch::from_on(None, start),
            Stopwatch::from_on(None, after_one_day),
        ]);
        let output = ron::ser::to_string_pretty(&input, PrettyConfig::default()).unwrap();

        insta::assert_display_snapshot!(output);
    }
}
