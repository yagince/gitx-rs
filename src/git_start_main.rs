extern crate gitx;
extern crate rustc_serialize;
extern crate docopt;
extern crate ansi_term;

use std::process::Output;

use docopt::Docopt;
use ansi_term::Style;
use ansi_term::Colour::*;

use gitx::git::git::*;
use gitx::git::branch::*;

const USAGE: &'static str = "
Usage:
  git-start <type> <number> <description>
  git-start (--help | --version)

Options:
  -h --help     Show this screen.
  --version     Show version.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_type: String,
    arg_number: String,
    arg_description: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    // println!("{:?}", args);
    start(&args);
}

fn start(args: &Args) {
    let git = Git::new();
    let arg_type = match args.arg_type.as_ref() {
        "f" => "feature",
        "s" => "spark",
        "h" => "hotfix",
        t   => t
    };
    let branch_name = format!("{}/{}-{}", arg_type, args.arg_number, args.arg_description);
    let output = git.create_branch(&Branch::new(&branch_name))
        .unwrap_or_else(|e| panic!(e));

    println!("{}:\t{}", Style::new().bold().fg(Green).paint("BranchType"), arg_type);
    println!("{}:\t{}", Style::new().bold().fg(Green).paint("IssueNumber"), args.arg_number);
    println!("{}:\t{}", Style::new().bold().fg(Green).paint("Description"), args.arg_description);
   // TODO: エラーハンドリング
    print_stdout(&output);
}

fn print_stdout(output : &Output) {
    println!("status: {}", output.status);
    println!("stdout: \n{}", String::from_utf8_lossy(&output.stdout));
}
