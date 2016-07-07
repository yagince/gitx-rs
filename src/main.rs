extern crate gitx_rs;

use gitx_rs::git::git::*;

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

    println!("== Checkout ==============================");

    let output = git.checkout(&branches.current_branch());
    println!("{}", String::from_utf8_lossy(&output.unwrap().stdout));
}
