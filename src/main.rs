pub mod options;

use crate::options::Options;

fn main() {
    let options = Options::from_args();

    println!("{:?}", options);
}
