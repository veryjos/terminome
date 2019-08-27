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
    let timer = Timer::new(&options);

    if let TimerEvent::Tick { time } = timer.get_next_event(Some(0)).await {
        render_time(time);
    }

    while let TimerEvent::Tick { time } = timer.get_next_event(None).await {
        render_time(time);
    }
}
