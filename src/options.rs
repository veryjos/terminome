use std::time::Duration;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt()]
struct CliOptions {
    #[structopt(default_value = "Duration::from_secs(0)")]
    time: Duration,
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
        Options {
            time: opt.time,
            countdown: opt.time > Duration::from_secs(0),
        }
    }
}
