extern crate gitx;
extern crate rustc_serialize;
extern crate docopt;
extern crate rustbox;
extern crate regex;

use std::process::*;
use std::error::Error;
use std::default::Default;
use std::time::Duration;
use std::thread;
use std::collections::HashSet;

use docopt::Docopt;
use rustbox::{Color, RustBox};
use rustbox::Key;

use gitx::git::git::*;
use gitx::git::branch::*;

const VERSION: &'static str = "0.0.1";

const USAGE: &'static str = "
Usage:
  git-db
  git-db (--help | --version)

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
    selected_index: usize,
    delete_indexes: HashSet<usize>,
}

impl Context {
    fn up_selected(&mut self) {
        if self.selected_index == 0 {
            self.selected_index = self.branch_list().len() - 1;
        } else {
            self.selected_index -= 1;
        }
    }

    fn down_selected(&mut self) {
        if self.selected_index == self.branch_list().len() - 1 {
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

    fn selected_branch(&self) -> Option<Branch> {
        self.branch_list().get(self.selected_index).map(|b| b.clone())
    }

    fn mark_selected_to_delete(&mut self) {
        self.delete_indexes.insert(self.selected_index);
    }

    fn unmark_selected_to_delete(&mut self) {
        self.delete_indexes.remove(&self.selected_index);
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

        let text =
            if context.delete_indexes.contains(&i) {
                format!("D {}", text)
            } else {
                format!("  {}", text)
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

    thread::sleep(Duration::from_millis(2000));

    print(context);
}

fn exec() {
    let rustbox = RustBox::init(Default::default()).unwrap_or_else(|e| panic!(e));

    let git = Git::new();
    let branches = git.branches();

    let mut context = Context{
        rustbox: rustbox,
        branches: branches,
        selected_index: 0,
        delete_indexes: HashSet::new(),
    };

    print(&context);

    loop {
        match context.rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Esc | Key::Ctrl('c') => { break; },
                    Key::Char('c') => {
                        context.unmark_selected_to_delete();
                        context.down_selected();
                    },
                    Key::Char('d') | Key::Ctrl('h') | Key::Backspace | Key::Delete => {
                        context.mark_selected_to_delete();
                        context.down_selected();
                    },
                    Key::Ctrl('n') | Key::Down => {
                        context.down_selected();
                    },
                    Key::Ctrl('p') | Key::Up => {
                        context.up_selected();
                    },
                    Key::Enter => {
                        match context.selected_branch() {
                            Some(branch) => {
                                // let output = git.checkout(&branch).unwrap();

                                // if output.status.success() {
                                //     println!("{}", String::from_utf8_lossy(&output.stdout));
                                //     break;
                                // } else {
                                //     print_err(output, &context);
                                // }
                            },
                            _ => {},
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
