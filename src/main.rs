extern crate gitx_rs;

use gitx_rs::git::git::*;

fn main() {
    let git = Git::new();
    let output = git
        .status()
        .unwrap_or_else(|e| { panic!("Failed to executte git: {}", e) });
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    // println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    println!("================================");

    let branches = git.branches();
    println!("branches: {:?}", branches.branches);
    println!("current: {:?}", branches.current_branch());

    println!("================================");

    let logs = git.logs();
    println!("logs: {:?}", logs);
    for l in &logs {
        println!("{}", l);
    }
}
