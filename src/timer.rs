use std::time::{Duration, Instant};
use tokio::{self, timer::{Delay}};

use crate::options::Options;

pub enum TimerEvent {
    Tick { time: u64 },
    Complete,
}

pub struct Timer {
    duration: Duration,
    countdown: bool,
    start_time: Instant,
}

impl Timer {
    pub fn new(options: &Options) -> Self {
        Timer {
            duration: Duration::from_secs_f64(options.time),
            countdown: options.countdown,
            start_time: Instant::now(),
        }
    }

    pub async fn get_next_event(&mut self) -> TimerEvent {
        // immediately complete if time is greater than start + duration
        if self.countdown && Instant::now() >= self.start_time + self.duration {
            return TimerEvent::Complete;
        }

        // complete when we need to render the next frame
        // aka, at the next whole second
        let now = Instant::now();
        let wait_time = 1000 - ((now - self.start_time).as_millis() % 1000);
        let expected_time = now + Duration::from_millis(wait_time as u64);
        Delay::new(expected_time).await;

        let time_elapsed = expected_time - self.start_time;

        TimerEvent::Tick {
            time: match self.countdown {
                true => self.duration.checked_sub(time_elapsed)
                            .unwrap_or(Duration::from_millis(0)),

                false => time_elapsed,
            }.as_secs_f64().round() as u64,
        }
    }
}
