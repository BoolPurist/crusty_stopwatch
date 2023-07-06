static USER_DATA: Lazy<Mutex<Timers>> = Lazy::new(|| Mutex::new(Timers::default()));

use core_stopwatch::Timers;
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub fn get_data(on_read: impl FnOnce(&Timers)) {
    let data = &USER_DATA;
    let data = data.lock().unwrap();
    on_read(&data);
}

pub fn set_data(timers: Timers) {
    let data = &USER_DATA;
    let mut data = data.lock().unwrap();
    *data = timers;
}
