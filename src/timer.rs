use std::time::{Duration, Instant};
use tokio::{self, timer::{Delay}};

use crate::options::Options;


pub enum TimerEvent {
    Tick { time: f64 }
}

pub struct Timer {
    time: f64,
    countdown: bool,
    start_time: Instant,
}

impl Timer {
    pub fn new(options: &Options) -> Self {
        Timer {
            time: options.time,
            countdown: options.countdown,
            start_time: Instant::now(),
        }
    }

    pub async fn get_event(&mut self) -> TimerEvent {
        // complete when we need to render the next frame
        // aka, at the next whole second
        let now = Instant::now();
        let wait_time = 1000 - ((now - self.start_time).as_millis() % 1000);
        Delay::new(now + Duration::from_millis(wait_time as u64)).await;

        TimerEvent::Tick {
            time: (Instant::now() - self.start_time).as_secs_f64()
        }
    }
}
