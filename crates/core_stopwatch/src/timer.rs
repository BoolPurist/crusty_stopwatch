use chrono::{DateTime, Duration, Utc};

pub trait Timer {
    fn elapsed(&self) -> Duration;
    fn started(&self) -> DateTime<Utc>;
    fn resume(&mut self);
    fn pause(&mut self);
    fn is_paused(&self) -> bool;
    fn reset(&mut self);

    fn current_time(&self) -> DateTime<Utc> {
        self.started() + self.elapsed()
    }
}
