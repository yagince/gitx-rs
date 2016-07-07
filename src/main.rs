extern crate gitx_rs;

use std::process::Output;
use gitx_rs::git::git::*;
use gitx_rs::git::branch::*;

fn main() {
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

fn print_stdout(output : &Output) {
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
}
