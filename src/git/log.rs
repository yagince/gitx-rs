use std::fmt;

#[derive(Debug)]
pub struct Log {
    pub commit: String,
    pub message: String,
}

impl Log {
    pub fn new(commit: &str, message: &str) -> Log {
        Log{commit: commit.trim().to_string(), message: message.trim().to_string()}
    }
}

impl fmt::Display for Log {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.commit, self.message)
    }
}
