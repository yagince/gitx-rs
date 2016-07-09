extern crate rustc_serialize;
extern crate ansi_term;

use std::process::Output;

use self::ansi_term::Style;
use self::ansi_term::Colour::*;

use git::git::*;
use git::branch::*;

pub struct Options {
    branch_type: String,
    issue_number: String,
    description: String,
}

impl Options {
    pub fn new(branch_type: &str, issue_number: &str, description: &str) -> Options {
        Options {
            branch_type: branch_type.to_string(),
            issue_number: issue_number.to_string(),
            description: description.to_string(),
        }
    }

    pub fn branch_type(&self) -> String {
        match self.branch_type.as_ref() {
            "f" => "feature",
            "s" => "spark",
            "h" => "hotfix",
            t   => t
        }.to_string()
    }

    pub fn branch_name(&self) -> String {
        format!("{}/{}-{}", self.branch_type(), self.issue_number, self.description)
    }
}


pub fn exec(args: &Options) {
    let output = Git::new()
        .create_branch(&Branch::new(args.branch_name().as_ref()))
        .unwrap_or_else(|e| panic!(e));

    // TODO: エラーハンドリング
    println!("{}:\t{}", Style::new().bold().fg(Green).paint("BranchType"), args.branch_type());
    println!("{}:\t{}", Style::new().bold().fg(Green).paint("IssueNumber"), args.issue_number);
    println!("{}:\t{}", Style::new().bold().fg(Green).paint("Description"), args.description);

    print_stdout(&output);
}

fn print_stdout(output : &Output) {
    println!("status: {}", output.status);
    println!("stdout: \n{}", String::from_utf8_lossy(&output.stdout));
}

#[cfg(test)]
mod tests {
    mod options {
        use super::super::*;

        #[test]
        fn branch_type() {
            let opt = Options::new("f", "hoge", "foo");
            assert_eq!("feature", opt.branch_type());

            let opt = Options::new("h", "hoge", "foo");
            assert_eq!("hotfix", opt.branch_type());

            let opt = Options::new("s", "hoge", "foo");
            assert_eq!("spark", opt.branch_type());

            let opt = Options::new("test", "hoge", "foo");
            assert_eq!("test", opt.branch_type());
        }

        #[test]
        fn branch_name() {
            let opt = Options::new("test", "hoge", "foo");
            assert_eq!("test/hoge-foo", opt.branch_name());
        }
    }
}
