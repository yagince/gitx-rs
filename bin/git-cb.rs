extern crate gitx;
#[macro_use]
extern crate serde_derive;
extern crate docopt;

use docopt::Docopt;
use std::process::*;

use gitx::cmd::cb;

const VERSION: &str = env!("CARGO_PKG_VERSION");

const USAGE: &str = "
Usage:
  git-cb
  git-cb (--help | --version)

Options:
  -h --help     Show this screen.
  --version     Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_version: bool,
}
fn main() -> Result<(), Box<std::error::Error>> {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if args.flag_version {
        println!("git cb {}", VERSION);
        exit(0);
    }
    cb::exec()
}
