use std::time::{Duration, Instant};
use tokio::{self, timer::{Delay}};

use crate::options::Options;

pub struct TimeEvent {
    pub time: f64,
}

pub struct Timer {
    time: f64,
    countdown: bool,
}

impl Timer {
    pub fn new(options: &Options) -> Self {
        Timer {
            time: options.time,
            countdown: options.countdown,
        }
    }

    pub async fn get_event(&self) -> TimeEvent {
        Delay::new(Instant::now() + Duration::from_millis(1000)).await;

        TimeEvent { time: 123.123 }
    }
}
