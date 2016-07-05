extern crate gitx_rs;

use gitx_rs::git::git::*;

fn main() {
    let git = Git::new();
    let output = git
        .status()
        .unwrap_or_else(|e| { panic!("Failed to executte git: {}", e) });
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    let branches = git.branches();
    println!("branches: {:?}", branches.branches);
}
