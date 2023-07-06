use crate::chrono_duration_serde;
use crate::{now_provider::NowProvider, ChronoNow, Timer};
use chrono::{DateTime, Duration, Utc};
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Getters, Clone)]
pub struct Stopwatch<T = ChronoNow> {
    #[getter(skip)]
    title: Option<String>,
    #[getter(skip)]
    started: DateTime<Utc>,
    #[serde(with = "chrono_duration_serde")]
    elasped_since_last_pause: chrono::Duration,
    is_paused: bool,
    #[getter(skip)]
    provider_for_now: T,
}

impl Stopwatch<ChronoNow> {
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    pub fn now() -> Self {
        let now: ChronoNow = Default::default();
        Self {
            title: None,
            started: now.now(),
            elasped_since_last_pause: Duration::zero(),
            is_paused: true,
            provider_for_now: now,
        }
    }

    pub fn from_on(title: Option<String>, started: DateTime<Utc>) -> Self {
        let now: ChronoNow = Default::default();
        Self {
            title,
            started,
            elasped_since_last_pause: Duration::zero(),
            is_paused: true,
            provider_for_now: now,
        }
    }
}

#[cfg(test)]
use crate::now_provider::test_utility::TestProvider;

#[cfg(test)]
impl Stopwatch<TestProvider> {
    pub(crate) fn started_from(provider: TestProvider, started: DateTime<Utc>) -> Self {
        Self {
            title: None,
            started,
            elasped_since_last_pause: Duration::zero(),
            is_paused: true,
            provider_for_now: provider,
        }
    }
}

impl<T> Stopwatch<T> {
    pub fn with_name(&mut self, name: Option<String>) -> &mut Self {
        self.title = name;
        self
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
    use crate::now_provider::test_utility::TestProvider;
    use crate::Stopwatch;
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
        let actual = Stopwatch::started_from(mock, given_time.borrow().clone());

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
