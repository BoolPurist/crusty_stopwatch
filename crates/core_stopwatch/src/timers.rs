use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::{ChronoNow, Stopwatch};

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
