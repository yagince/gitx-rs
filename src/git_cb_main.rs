extern crate gitx;
extern crate rustc_serialize;
extern crate docopt;
extern crate rustbox;

use std::process::*;
use std::error::Error;
use std::default::Default;
use std::thread;

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
    selected_index: usize,
}

impl Context {
    fn input(&mut self, c: char) {
        self.input.push(c);
    }

    fn pop(&mut self) {
        self.input.pop();
    }

    fn up_selected(&mut self) {
        if self.selected_index == 0 {
            self.selected_index = self.branches.list().len() - 1;
        } else {
            self.selected_index -= 1;
        }
    }

    fn down_selected(&mut self) {
        if self.selected_index == self.branches.list().len() - 1 {
            self.selected_index = 0;
        } else {
            self.selected_index += 1;
        }
    }

    fn branch_list(&self) -> Vec<Branch> {
        let mut list = self.branches.list();
        list.sort();
        list
    }

    fn selected_branch(&self) -> Branch {
        self.branch_list().get(self.selected_index).map(|b| b.clone()).unwrap()
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

    let list = context.branch_list();
    let horizontal_offset = 2;

    for (i, branch) in list.iter().enumerate() {

        let text =
            if context.branches.is_current(branch) {
                format!("{:2}: * {}", i, branch.name)
            } else {
                format!("{:2}:  {}",  i, branch.name)
            };

        if i == context.selected_index {
            context.rustbox.print(1, i+horizontal_offset, rustbox::RB_BOLD, Color::Green, Color::Magenta, text.as_ref());
        } else if context.branches.is_current(branch) {
            context.rustbox.print(1, i+horizontal_offset, rustbox::RB_BOLD, Color::Green, Color::Default, text.as_ref());
        } else {
            context.rustbox.print(1, i+horizontal_offset, rustbox::RB_BOLD, Color::White, Color::Default, text.as_ref());
        }
    }

    context.rustbox.present();
}

fn print_err(output: Output, context: &Context) {
    context.rustbox.clear();

    context.rustbox.print(
        0,
        0,
        rustbox::RB_BOLD,
        Color::Magenta,
        Color::Default,
        String::from_utf8_lossy(&output.stderr).as_ref(),
    );
    context.rustbox.present();

    thread::sleep(2000);

    print(context);
}

fn exec() {
    let rustbox = RustBox::init(Default::default()).unwrap_or_else(|e| panic!(e));

    let git = Git::new();
    let branches = git.branches();

    let mut context = Context{
        rustbox: rustbox,
        branches: branches,
        input: String::new(),
        selected_index: 0,
    };

    print(&context);

    loop {
        match context.rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Esc | Key::Ctrl('c') => { break; },
                    Key::Char(c) => {
                        context.input(c);
                    },
                    Key::Ctrl('h') | Key::Backspace | Key::Delete => {
                        context.pop();
                    },
                    Key::Ctrl('n') | Key::Down => {
                        context.down_selected();
                    },
                    Key::Ctrl('p') | Key::Up => {
                        context.up_selected();
                    },
                    Key::Enter => {
                        let branch = context.selected_branch();
                        let output = git.checkout(&branch).unwrap();

                        if output.status.success() {
                            println!("{}", String::from_utf8_lossy(&output.stdout));
                            break;
                        } else {
                            print_err(output, &context);
                        }
                    },
                    _ => { },
                }
            },
            Err(e) => panic!("{}", e.description()),
            _ => { }
        }
        print(&context);
    }
}
