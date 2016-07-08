extern crate gitx_rs;
extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;

use std::process::Output;
use gitx_rs::git::git::*;
use gitx_rs::git::branch::*;

const USAGE: &'static str = "
@yagince

Usage:
  git-x start <type> <number> <comment>
  git-x (--help | --version)

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
    debug();

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

fn debug() {
    println!("== Status ==============================");
    let git = Git::new();
    let output = git
        .status()
        .unwrap_or_else(|e| { panic!("Failed to executte git: {}", e) });
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    // println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    println!("== Branch List ==============================");

    let branches = git.branches();
    println!("branches: {:?}", branches.branches);
    println!("current: {:?}", branches.current_branch());

    println!("== Logs ==============================");

    let logs = git.logs();
    println!("logs: {:?}", logs);
    for l in &logs {
        println!("{}", l);
    }

    println!("== Checkout Foo ==============================");

    let default = Branch::new("-");
    let branch = branches.branches.get(&"foo".to_string()).unwrap_or(&default);
    let output = git.checkout(&branch);
    print_stdout(&output.unwrap());

    println!("== Checkout Prev ==============================");

    let output = git.checkout_prev();
    print_stdout(&output.unwrap());
}
