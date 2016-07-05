use std::io;
use std::collections::HashMap;
use std::process::{Command, Output};
use std::ffi::*;
use git::branch::*;

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

    pub fn branches(&self) -> Branches {
        let output = self.command().arg("branch").output().unwrap();

        let mut current = String::new();

        let map = String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|l| l.trim_matches(' '))
            .map(|l| {
                let branch = Branch{name: l.replace("* ", "").to_string()};
                if l.starts_with('*') {
                    current = branch.name.clone();
                }
                branch
            })
            .fold(HashMap::new(), |mut branches, branch| {
                branches.insert(branch.name.clone(), branch);
                branches
            });
        Branches{branches: map, current: current}
    }
}
