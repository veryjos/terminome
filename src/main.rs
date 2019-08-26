use tokio;

pub mod options;
pub mod timer;

use crate::options::Options;
use crate::timer::{Timer, TimerEvent};

#[tokio::main]
async fn main() {
    let options = Options::from_args();
    let mut timer = Timer::new(&options);

    loop {
        let event = timer.get_next_event().await;

        match event {
            TimerEvent::Tick { time } => {
                println!("{}", time);
            },

            TimerEvent::Complete => {
                println!("Done!");
                break;
            },
        }
    }
}
