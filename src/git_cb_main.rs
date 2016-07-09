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

struct Context {
    rustbox: RustBox,
    branches: Branches,
    input: String,
}

impl Context {
    fn input(&mut self, c: char) {
        self.input.push(c);
    }

    fn pop(&mut self) {
        self.input.pop();
    }
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

fn print(context: &Context) {
    context.rustbox.clear();

    context.rustbox.print(0, 0, rustbox::RB_BOLD, Color::White, Color::Default, format!("QUERY > {}", context.input).as_ref());
    context.rustbox.print(0, 1, rustbox::RB_BOLD, Color::Green, Color::Default, "Press ESC or Ctrl+c to exit.");

    let mut list = context.branches.list();
    list.sort();

    let horizontal_offset = 2;

    for (i, branch) in list.iter().enumerate() {
        if context.branches.is_current(branch) {
            context.rustbox.print(1, i+horizontal_offset, rustbox::RB_BOLD, Color::Green, Color::Default, format!("{:2}: * {}", i, branch.name).as_ref());
        } else {
            context.rustbox.print(1, i+horizontal_offset, rustbox::RB_BOLD, Color::White, Color::Default, format!("{:2}:  {}", i, branch.name).as_ref());
        }
    }

    context.rustbox.present();
}

fn exec() {
    let rustbox = RustBox::init(Default::default()).unwrap_or_else(|e| panic!(e));

    let git = Git::new();
    let branches = git.branches();

    let mut context = Context{
        rustbox: rustbox,
        branches: branches,
        input: String::new(),
    };

    print(&context);

    loop {
        match context.rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Esc | Key::Ctrl('c') => { break; },
                    Key::Char(c) => {
                        context.input(c);
                        print(&context);
                    },
                    Key::Ctrl('h') | Key::Backspace | Key::Delete => {
                        context.pop();
                        print(&context);
                    },
                    _ => { },
                }
            },
            Err(e) => panic!("{}", e.description()),
            _ => { }
        }
    }
}
