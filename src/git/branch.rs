use std::collections::HashMap;

#[derive(Debug)]
pub struct Branches {
    pub branches: HashMap<String, Branch>,
    pub current_commit: String,
}

#[derive(Debug)]
pub struct Branch {
    pub name: String,
}
