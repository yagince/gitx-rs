extern crate gitx;
extern crate rustc_serialize;
extern crate docopt;
extern crate rustbox;

use std::process::*;
use std::error::Error;
use std::default::Default;

use docopt::Docopt;
use rustbox::{Color, RustBox};
use rustbox::Key;

use gitx::git::git::*;
use gitx::git::branch::*;

const VERSION: &'static str = "0.0.1";

const USAGE: &'static str = "
Usage:
  git-cb
  git-cb (--help | --version)

Options:
  -h --help     Show this screen.
  --version     Show version.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_version: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    if args.flag_version {
        println!("git cb {}", VERSION);
        exit(0);
    }
    exec();
}

fn exec() {
    let rustbox = RustBox::init(Default::default()).unwrap_or_else(|e| panic!(e));

    let git = Git::new();
    let branches = git.branches();

    let mut list = branches.list();
    list.sort();

    for (i, branch) in list.iter().enumerate() {
        if branches.is_current(branch) {
            rustbox.print(1, i+1, rustbox::RB_BOLD, Color::Green, Color::Default, format!("* {}", branch.name).as_ref());
        } else {
            rustbox.print(1, i+1, rustbox::RB_BOLD, Color::White, Color::Default, branch.name.as_ref());
        }
    }

    rustbox.present();

    let mut v = String::new();

    loop {
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char('q') | Key::Ctrl('c') => { break; },
                    Key::Char(c) => {
                        v.push(c);
                        rustbox.print(1, 0, rustbox::RB_BOLD, Color::White, Color::Default, v.as_ref());
                        rustbox.present();
                    },
                    Key::Backspace | Key::Delete => {
                        // TODO: clear
                        v.pop();
                        rustbox.print(1, 0, rustbox::RB_BOLD, Color::White, Color::Default, v.as_ref());
                        rustbox.present();
                    },
                    _ => { },
                }
            },
            Err(e) => panic!("{}", e.description()),
            _ => { }
        }
    }
}
