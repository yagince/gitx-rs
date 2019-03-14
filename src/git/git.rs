use std::io;
use std::collections::HashMap;
use std::process::{Command, Output};
use std::ffi::*;
use git::branch::*;
use git::log::*;

pub struct Git {
    path: String,
}

impl Git {
    pub fn new() -> Git {
        Git{path: "git".to_string()}
    }

    fn command(&self) -> Command {
        Command::new(OsString::from(&self.path))
    }

    pub fn status(&self) -> io::Result<Output> {
        self.command()
            .arg("status")
            .output()
    }

    pub fn branches(&self) -> io::Result<Branches> {
        let output = self.command().arg("branch").arg("-vv").output()?;

        let mut current = String::new();

        let map = String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|l| l.trim_matches(' '))
            .map(|l| {
                let branch = Branch::new(l.replace("* ", "").as_ref());
                if l.starts_with('*') {
                    current = branch.name.clone();
                }
                branch
            })
            .fold(HashMap::new(), |mut branches, branch| {
                branches.insert(branch.name.clone(), branch);
                branches
            });
        Ok(Branches{branches: map, current: current})
    }

    pub fn logs(&self) -> Vec<Log> {
        let output = self.command()
            .arg("log")
            .arg("--oneline")
            .output()
            .unwrap();
        String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|l| {
                let (commit, msg) = l.split_at(8);
                Log::new(commit, msg)
            })
            .collect()
    }

    pub fn checkout(&self, branch : &Branch) -> io::Result<Output> {
        self.command().arg("checkout").arg(&branch.name).output()
    }

    pub fn checkout_prev(&self) -> io::Result<Output> {
        self.checkout(&Branch::new_by_name("-"))
    }

    pub fn create_branch(&self, branch : &Branch) -> io::Result<Output> {
        self.command().arg("checkout").arg("-b").arg(&branch.name).output()
    }

    pub fn delete_local_branch(&self, branch: &Branch) ->  io::Result<Output> {
        self.command().arg("branch").arg("-D").arg(&branch.name).output()
    }
}
