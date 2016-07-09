extern crate gitx;
extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;

use std::process::Output;
use gitx::git::git::*;
use gitx::git::branch::*;

const USAGE: &'static str = "
Usage:
  git-start <type> <number> <comment>
  git-start (--help | --version)

Options:
  -h --help     Show this screen.
  --version     Show version.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    cmd_start: bool,
    arg_type: String,
    arg_number: String,
    arg_comment: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    println!("{:?}", args);
    if args.cmd_start {
        start(&args);
    }
}

fn start(args: &Args) {
    let git = Git::new();
    let arg_type = match args.arg_type.as_ref() {
        "f" => "feature",
        "s" => "spark",
        "h" => "hotfix",
        t   => t
    };
    let branch_name = format!("{}/{}-{}", arg_type, args.arg_number, args.arg_comment);
    let output = git.create_branch(&Branch::new(&branch_name))
        .unwrap_or_else(|e| panic!(e));
    print_stdout(&output);
}

fn print_stdout(output : &Output) {
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
}
