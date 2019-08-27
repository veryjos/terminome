use tokio;

mod glyphs;
mod options;
mod timer;
mod render;

use crate::options::Options;
use crate::timer::{Timer, TimerEvent};
use crate::render::render_time;

#[tokio::main]
async fn main() {
    let options = Options::from_args();
    let mut timer = Timer::new(&options);

    while let TimerEvent::Tick { time } = timer.get_next_event().await {
        render_time(time);
    }
}
