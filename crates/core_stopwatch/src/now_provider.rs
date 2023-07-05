use std::{cell::RefCell, rc::Rc};

use chrono::{DateTime, Utc};

pub trait NowProvider {
    fn now(&self) -> DateTime<Utc>;
}

#[derive(Default)]
pub struct ChronoNow;

impl NowProvider for ChronoNow {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}

pub struct TestProvider {
    current: Rc<RefCell<DateTime<Utc>>>,
}

impl TestProvider {
    pub fn new(current: Rc<RefCell<DateTime<Utc>>>) -> Self {
        Self { current }
    }
}

impl NowProvider for TestProvider {
    fn now(&self) -> DateTime<Utc> {
        self.current.borrow().clone()
    }
}
