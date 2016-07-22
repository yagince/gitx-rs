extern crate gitx;
extern crate rustc_serialize;
extern crate docopt;

use std::process::*;
use std::time::Duration;
use std::thread;

use docopt::Docopt;

use gitx::cmd::*;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

const USAGE: &'static str = "
Usage:
  git-db
  git-db (--help | --version)

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
        println!("git db {}", VERSION);
        exit(0);
    }

    let mut outputs: Vec<Output> = Vec::new();
    let rx = db::exec();
    loop {
        match rx.recv().unwrap() {
            db::Message::Quit => break,
            db::Message::Result(output) => {
                outputs.push(output);
            },
        }
    }

    thread::sleep(Duration::from_millis(100));

    for output in &outputs {
        let out = String::from_utf8_lossy(&output.stdout);
        if out.len() != 0 {
            println!("{}", out.trim());
        }
        let err = String::from_utf8_lossy(&output.stderr);
        if err.len() != 0 {
            println!("{}", err.trim());
        }
    }
}
