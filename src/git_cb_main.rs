extern crate gitx;
extern crate rustc_serialize;
extern crate docopt;

use std::process::*;
use docopt::Docopt;

use gitx::cmd::cb;

const VERSION: &'static str = "0.0.1";

const USAGE: &'static str = "
Usage:
  git-cb
  git-cb (--help | --version)

Options:
  -h --help     Show this screen.
  --version     Show version.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_version: bool,
}
fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    if args.flag_version {
        println!("git cb {}", VERSION);
        exit(0);
    }
    cb::exec();
}
