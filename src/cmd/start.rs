use std::process::Output;

use ansi_term::*;
use ansi_term::Colour::*;

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
        }.into()
    }

    pub fn branch_name(&self) -> String {
        format!("{}/{}-{}", self.branch_type(), self.issue_number, self.description)
    }
}


pub fn exec(args: &Options) {
    let output = Git::new()
        .create_branch(&Branch::new_by_name(args.branch_name().as_ref()))
        .expect("Create branch");

    // TODO: エラーハンドリング
    println!("{}:\t{}", paint("BranchType", Green),  paint(&args.branch_type(), Cyan));
    println!("{}:\t{}", paint("IssueNumber", Green), paint(&args.issue_number, Cyan));
    println!("{}:\t{}", paint("Description", Green), paint(&args.description, Cyan));
    println!("");

    print_output(&output);
}

fn print_output(output : &Output) {
    println!("{}:\n  {}", paint("status", Yellow), output.status);
    println!("{}:\n{}",   paint("stdout", Yellow), String::from_utf8_lossy(&output.stdout));
    println!("{}:\n{}",   paint("stderr", Yellow), String::from_utf8_lossy(&output.stderr));
}

fn paint(v: &str, color: Colour) -> ANSIString {
    Style::new().bold().fg(color).paint(v)
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
