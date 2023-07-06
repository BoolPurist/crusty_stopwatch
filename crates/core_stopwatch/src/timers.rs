use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::{ChronoNow, Stopwatch};

#[derive(Debug, Default, new, Serialize, Deserialize, Clone)]
pub struct Timers {
    stopwatches: Vec<Stopwatch>,
}

impl Timers {
    pub fn add_new_stopwatch(&mut self, new_stop_watch: Stopwatch<ChronoNow>) {
        self.stopwatches.push(new_stop_watch);
    }
    pub fn iter_mut_stopwatches(&mut self) -> impl Iterator<Item = &mut Stopwatch> {
        self.stopwatches.iter_mut()
    }
    pub fn iter_stopwatches(&self) -> impl Iterator<Item = &Stopwatch> {
        self.stopwatches.iter()
    }

    pub fn to_stopwatches(self) -> Vec<Stopwatch<ChronoNow>> {
        self.stopwatches
    }

    pub fn clear(&mut self) {
        self.stopwatches.clear();
    }
}
