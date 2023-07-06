use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub trait NowProvider {
    fn now(&self) -> DateTime<Utc>;
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct ChronoNow;

impl NowProvider for ChronoNow {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}

#[cfg(test)]
pub mod test_utility {
    use super::*;
    use std::{cell::RefCell, rc::Rc};

    pub struct TestProvider {
        current: Rc<RefCell<DateTime<Utc>>>,
    }

    impl TestProvider {
        pub fn new(current: Rc<RefCell<DateTime<Utc>>>) -> Self {
            Self { current }
        }
    }

    impl From<DateTime<Utc>> for TestProvider {
        fn from(value: DateTime<Utc>) -> Self {
            Self {
                current: Rc::new(RefCell::new(value)),
            }
        }
    }

    impl NowProvider for TestProvider {
        fn now(&self) -> DateTime<Utc> {
            self.current.borrow().clone()
        }
    }
}
