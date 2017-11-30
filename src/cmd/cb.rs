extern crate rustbox;
extern crate regex;

use std::process::*;
use std::error::Error;
use std::default::Default;
use std::time::Duration;
use std::thread;

use self::rustbox::{Color, RustBox};
use self::rustbox::Key;
use self::regex::Regex;

use git::git::*;
use git::branch::*;

struct Context {
    rustbox: RustBox,
    branches: Branches,
    input: String,
    selected_index: usize,
}

impl Context {
    fn input(&mut self, c: char) {
        self.input.push(c);
        self.adjust_selected_index();
    }

    fn pop(&mut self) {
        self.input.pop();
        self.adjust_selected_index();
    }

    fn adjust_selected_index(&mut self) {
        let list_size = self.branch_list().len();
        if list_size <= 1 {
            self.selected_index = 0;
            return
        }

        let max_index = list_size - 1;
        if self.selected_index > max_index {
            self.selected_index = max_index;
        }
    }

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
        if self.input.len() == 0 {
            return list
        }

        match Regex::new(format!("(?i){}", self.input).as_ref()) {
            Ok(regex) => {
                list.into_iter().filter(|x| {
                    regex.is_match(x.name.as_ref())
                }).collect()
            },
            Err(_) => list,
        }
    }

    fn selected_branch(&self) -> Option<Branch> {
        self.branch_list().get(self.selected_index).map(|b| b.clone())
    }
}


pub fn exec() {
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
                        match context.selected_branch() {
                            Some(branch) => {
                                let output = git.checkout(&branch).unwrap();

                                if output.status.success() {
                                    println!("{}", String::from_utf8_lossy(&output.stdout));
                                    break;
                                } else {
                                    print_err(output, &context);
                                }
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

    thread::sleep(Duration::from_millis(2000));

    print(context);
}
