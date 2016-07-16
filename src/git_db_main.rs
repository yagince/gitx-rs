extern crate gitx;
extern crate rustc_serialize;
extern crate docopt;
extern crate rustbox;
extern crate regex;

use std::process::*;
use std::error::Error;
use std::default::Default;
use std::time::Duration;
use std::collections::HashSet;
use std::thread;
use std::sync::mpsc::*;

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
    remote_delete_indexes: HashSet<usize>,
}

enum Message {
    Quit,
    Result(Output),
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

    fn delete_marked_branches(&self) -> Vec<Branch> {
        self.branch_list().into_iter().filter(|b| {
            self.delete_indexes.contains(&self.index_of(b).unwrap())
        }).map(|b| b.clone()).collect()
    }

    fn mark_selected_to_delete(&mut self) {
        self.delete_indexes.insert(self.selected_index);
    }

    fn mark_selected_to_remote_delete(&mut self) {
        self.remote_delete_indexes.insert(self.selected_index);
    }

    fn unmark_selected_to_delete(&mut self) {
        self.delete_indexes.remove(&self.selected_index);
        self.remote_delete_indexes.remove(&self.selected_index);
    }

    fn decorate_branch_name(&self, branch: &Branch) -> String {
        let i = self.index_of(branch).unwrap();

        let text =
            if self.branches.is_current(branch) {
                format!("{:2}: * {}", i, branch.name)
            } else {
                format!("{:2}:  {}",  i, branch.name)
            };

        let text =
            if self.delete_indexes.contains(&i) {
                format!("D {}", text)
            } else {
                format!("  {}", text)
            };

        let text =
            if self.remote_delete_indexes.contains(&i) {
                format!("R{}", text)
            } else {
                format!(" {}", text)
            };

        text
    }

    fn index_of(&self, branch: &Branch) -> Option<usize> {
        for (i, b) in self.branch_list().iter().enumerate() {
            if branch == b {
                return Some(i)
            }
        }
        None
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

    let mut outputs: Vec<Output> = Vec::new();
    let rx = exec();
    loop {
        match rx.recv().unwrap() {
            Message::Quit => break,
            Message::Result(output) => {
                outputs.push(output);
            },
        }
    }

    thread::sleep(Duration::from_millis(100));

    for output in &outputs {
        let out = String::from_utf8_lossy(&output.stdout);
        if out.len() != 0 {
            println!("{}", out.trim());
        }
        let err = String::from_utf8_lossy(&output.stderr);
        if err.len() != 0 {
            println!("{}", err.trim());
        }
    }
}

fn print(context: &Context) {
    context.rustbox.clear();

    context.rustbox.print(0, 0, rustbox::RB_BOLD, Color::Green, Color::Default, "Press `a` to delete local and remote branch.");
    context.rustbox.print(0, 1, rustbox::RB_BOLD, Color::Green, Color::Default, "Press `d` to delete local branch.");
    context.rustbox.print(0, 2, rustbox::RB_BOLD, Color::Green, Color::Default, "Press `r` to delete remote branch.");
    context.rustbox.print(0, 3, rustbox::RB_BOLD, Color::Green, Color::Default, "Press ESC or Ctrl+c or `q` to exit.");
    context.rustbox.print(0, 4, rustbox::RB_BOLD, Color::Green, Color::Default, "Press Enter to execute delete branches");

    let list = context.branch_list();
    let horizontal_offset = 5;

    for (i, branch) in list.iter().enumerate() {

        let text = context.decorate_branch_name(&branch);

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

fn exec() -> Receiver<Message> {
    let (tx, rx): (Sender<Message>, Receiver<Message>) = channel();

    thread::spawn(move || {
        let rustbox = RustBox::init(Default::default()).unwrap_or_else(|e| panic!(e));

        let git = Git::new();
        let branches = git.branches();

        let mut context = Context{
            rustbox: rustbox,
            branches: branches,
            selected_index: 0,
            delete_indexes: HashSet::new(),
            remote_delete_indexes: HashSet::new(),
        };

        loop {
            print(&context);

            match context.rustbox.poll_event(false) {
                Ok(rustbox::Event::KeyEvent(key)) => {
                    match key {
                        Key::Char('q') | Key::Esc | Key::Ctrl('c') => { break; },
                        Key::Char('a') => {
                            context.mark_selected_to_delete();
                            context.mark_selected_to_remote_delete();
                            context.down_selected();
                        },
                        Key::Char('c') => {
                            context.unmark_selected_to_delete();
                            context.down_selected();
                        },
                        Key::Char('r') => {
                            context.mark_selected_to_remote_delete();
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
                            for branch in context.delete_marked_branches() {
                                let output = git.delete_local_branch(&branch).unwrap();
                                tx.send(Message::Result(output)).unwrap();
                            }
                            break;
                        },
                        _ => { },
                    }
                },
                Err(e) => panic!("{}", e.description()),
                _ => { }
            }
        }
        tx.send(Message::Quit).unwrap();
    });
    rx
}
