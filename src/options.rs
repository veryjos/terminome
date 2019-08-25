use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt()]
struct CliOptions {
    #[structopt(default_value = "0")]
    time: f64,
}

#[derive(Debug)]
pub struct Options {
    time: f64,
    countdown: bool,
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
            countdown: opt.time != 0.0,
        }
    }
}
