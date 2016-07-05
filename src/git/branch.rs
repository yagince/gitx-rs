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
}

#[derive(Debug, Clone)]
pub struct Branch {
    pub name: String,
}
