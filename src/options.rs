use std::time::Duration as Duration;

use humantime::Duration as HumanDuration;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt()]
struct CliOptions {
    #[structopt(default_value = "0s")]
    time: HumanDuration,
}

#[derive(Debug)]
pub struct Options {
    pub time: Duration,
    pub countdown: bool,
}

impl Options {
    pub fn from_args() -> Self {
        CliOptions::from_args().into()
    }
}

impl From<CliOptions> for Options {
    fn from(opt: CliOptions) -> Options {
        let time_std_duration = opt.time.into();

        Options {
            time: time_std_duration,
            countdown: time_std_duration > Duration::from_secs(0),
        }
    }
}
