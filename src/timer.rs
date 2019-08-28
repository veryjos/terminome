use std::time::{Duration, Instant};
use tokio::{self, timer::{Delay}};

use crate::options::Options;

pub enum TimerEvent {
    Tick { time: Duration },
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
            duration: options.time,
            countdown: options.countdown,
            start_time: Instant::now(),
        }
    }

    pub async fn get_next_event(&self) -> TimerEvent {
        // immediately resolve as completed if time is greater than start + duration
        if self.countdown && Instant::now() >= self.start_time + self.duration {
            return TimerEvent::Complete;
        }

        // resolve when we need to tick
        // aka, at the next whole second
        let now = Instant::now();
        let clock_time = now - self.start_time;
        let next_tick_time = now + Duration::from_secs_f64(1.0 - (clock_time.as_secs_f64() % 1.0));
        Delay::new(next_tick_time).await;

        let time_elapsed = next_tick_time - self.start_time;

        TimerEvent::Tick {
            time: match self.countdown {
                true => self.duration.checked_sub(time_elapsed)
                            .unwrap_or(Duration::from_secs(0)),

                false => time_elapsed,
            },
        }
    }
}
