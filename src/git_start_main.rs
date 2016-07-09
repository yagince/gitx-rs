extern crate gitx;
extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;
use std::process::*;

use gitx::cmd::*;

const VERSION: &'static str = "0.0.1";

const USAGE: &'static str = "
Usage:
  git-start <type> <number> <description>
  git-start (--help | --version)

Options:
  -h --help     Show this screen.
  -v --version     Show version.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_type: String,
    arg_number: String,
    arg_description: String,
    flag_version: bool,
}

impl Args {
    fn to_options(&self) -> start::Options {
        start::Options::new(
            self.arg_type.as_ref(),
            self.arg_number.as_ref(),
            self.arg_description.as_ref(),
        )
    }
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    // println!("{:?}", args);

    if args.flag_version {
        println!("git start {}", VERSION);
        exit(0);
    }

    start::exec(&args.to_options());
}
