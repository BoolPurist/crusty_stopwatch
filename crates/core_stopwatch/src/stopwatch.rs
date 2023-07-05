use crate::{now_provider::NowProvider, ChronoNow, TestProvider, Timer};
use chrono::{DateTime, Duration, Utc};
pub struct Stopwatch<T> {
    title: Option<String>,
    started: DateTime<Utc>,
    elasped_since_last_pause: Duration,
    is_paused: bool,
    provider_for_now: T,
}

impl Stopwatch<ChronoNow> {
    pub fn now(title: Option<String>) -> Self {
        let now: ChronoNow = Default::default();
        Self {
            title,
            started: now.now(),
            elasped_since_last_pause: Duration::zero(),
            is_paused: true,
            provider_for_now: now,
        }
    }
}

impl Stopwatch<TestProvider> {
    fn started_from(provider: TestProvider, title: Option<String>, started: DateTime<Utc>) -> Self {
        let now: ChronoNow = Default::default();
        Self {
            title,
            started,
            elasped_since_last_pause: Duration::zero(),
            is_paused: true,
            provider_for_now: provider,
        }
    }
}

impl<T> Timer for Stopwatch<T>
where
    T: NowProvider,
{
    fn pause(&mut self) {
        todo!()
    }
    fn resume(&mut self) {
        todo!()
    }

    fn started(&self) -> DateTime<Utc> {
        self.started
    }

    fn elapsed(&self) -> Duration {
        let now = self.provider_for_now.now();
        now - self.started
    }

    fn reset(&mut self) {
        todo!()
    }

    fn is_paused(&self) -> bool {
        todo!()
    }
}
#[cfg(test)]
mod testing {
    use crate::{Stopwatch, Timer};
    use chrono::TimeZone;
    use std::{cell::RefCell, rc::Rc};

    use super::*;
    #[test]
    fn elapsed_after_2_hour() {
        // Set up
        let given_time = Rc::new(RefCell::new(
            Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap(),
        ));
        let mock = TestProvider::new(given_time.clone());

        // Act
        let actual = Stopwatch::started_from(
            mock,
            Some("After_2_hours".to_owned()),
            given_time.borrow().clone(),
        );

        // Assert
        assert_eq!(
            Duration::zero(),
            actual.elapsed(),
            "No time should be passed right now"
        );
        let given_time_mom = *given_time.borrow();
        assert_eq!(
            given_time_mom,
            actual.started(),
            "Did not remember whe it was started"
        );

        // Assert after 2 hours
        let to_add = chrono::Duration::hours(2);
        let new_time = given_time_mom + to_add;
        *given_time.borrow_mut() = new_time;

        assert_eq!(to_add, actual.elapsed(), "2 hours shoulb been passed !");
        assert_eq!(
            new_time,
            actual.current_time(),
            "Current time should be started time plus 2 hours."
        );
    }
}
