pub mod prelude {
    pub use anyhow;
    pub use env_logger;
    pub use log::{error, info};
    pub type AppError = anyhow::Error;
    pub type AppResult<T = ()> = Result<T, anyhow::Error>;
}

pub mod data_source;
pub mod file_access;
pub mod formating;
pub mod now_provider;

pub use now_provider::{ChronoNow, NowProvider};
pub use stopwatch::Stopwatch;
pub use timer::Timer;
pub use timers::Timers;

mod chrono_duration_serde;
mod stopwatch;
mod timer;
mod timers;
