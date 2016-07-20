extern crate regex;

use std::collections::HashMap;
use self::regex::{Regex, Captures};

#[derive(Debug)]
pub struct Branches {
    pub branches: HashMap<String, Branch>,
    pub current: String,
}

impl Branches {
    pub fn current_branch(&self) -> Branch {
        self.branches.get(&self.current).unwrap().clone()
    }

    pub fn is_current(&self, branch: &Branch) -> bool {
        self.current == branch.name
    }

    pub fn list(&self) -> Vec<Branch> {
        self.branches
            .iter()
            .map(|(_, branch)| branch.clone())
            .collect()
    }
}

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Branch {
    pub name: String,
    pub last_commit: String,
    pub last_commit_msg: String,
    pub remote_branch_name: String,
}

impl Branch {
    pub fn new_by_name(name: &str) -> Branch {
        Branch {
            name: name.to_string(),
            last_commit: "".to_string(),
            last_commit_msg: "".to_string(),
            remote_branch_name: "".to_string(),
        }
    }

    pub fn new(log: &str) -> Branch {
        lazy_static! {
            static ref LOG_REGEXP: Regex = Regex::new(r"(?P<name>\S+)\s+(?P<commit>[:alnum:]+)\s+(?:(?P<remote>\[.*?\])\s+)?(?P<msg>.*)").unwrap();
        }
        LOG_REGEXP.captures(log).map_or_else(|| panic!("invalid branch log"), |c: Captures| {
            Branch{
                name: c.name("name").unwrap_or("").to_string(),
                last_commit: c.name("commit").unwrap_or("").to_string(),
                last_commit_msg: c.name("msg").unwrap_or("").to_string(),
                remote_branch_name: c.name("remote").unwrap_or("").to_string(),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    mod branches {
        use std::collections::HashMap;
        use super::super::*;

        #[test]
        fn current_branch() {
            let mut map = HashMap::new();
            map.insert("first".to_string(), Branch::new_by_name("first"));
            let branch = Branch::new_by_name("second");
            map.insert("second".to_string(), branch.clone());

            let branches = Branches{ branches: map, current: "second".to_string()};

            assert_eq!(branch, branches.current_branch());
        }

        #[test]
        fn is_current() {
            let mut map = HashMap::new();
            let first = Branch::new_by_name("first");
            map.insert("first".to_string(), first.clone());

            let second = Branch::new_by_name("second");
            map.insert("second".to_string(), second.clone());

            let branches = Branches{ branches: map, current: second.name.clone()};

            assert_eq!(false, branches.is_current(&first));
            assert_eq!(true,  branches.is_current(&second));
        }
    }

    mod branch {
        use super::super::*;

        #[test]
        fn new_branch() {
            let log = "master                              2d6953d [origin/master: behind 8] Merged in feature/presentation-material-ceres (pull request #287)";
            let branch = Branch::new(log);

            assert_eq!(Branch{
                name: "master".to_string(),
                last_commit: "2d6953d".to_string(),
                last_commit_msg: "Merged in feature/presentation-material-ceres (pull request #287)".to_string(),
                remote_branch_name: "[origin/master: behind 8]".to_string(),
            }, branch);
        }
    }
}
