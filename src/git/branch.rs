use std::collections::HashMap;

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
}

impl Branch {
    pub fn new(name : &str) -> Branch {
        Branch{name: name.to_string()}
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
            map.insert("first".to_string(), Branch{name: "first".to_string()});
            let branch = Branch{name: "second".to_string()};
            map.insert("second".to_string(), branch.clone());

            let branches = Branches{ branches: map, current: "second".to_string()};

            assert_eq!(branch, branches.current_branch());
        }

        #[test]
        fn is_current() {
            let mut map = HashMap::new();
            let first = Branch{name: "first".to_string()};
            map.insert("first".to_string(), first.clone());

            let second = Branch{name: "second".to_string()};
            map.insert("second".to_string(), second.clone());

            let branches = Branches{ branches: map, current: second.name.clone()};

            assert_eq!(false, branches.is_current(&first));
            assert_eq!(true,  branches.is_current(&second));
        }
    }
}
