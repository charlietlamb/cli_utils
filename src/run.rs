use crate::commands::commands::command;
use crate::cwd;
use crate::utils;
use std::io;

pub fn run() {
    loop {
        command(get_next_input());
    }
}

fn get_next_input() -> Vec<String> {
    let mut input = String::new();
    let mut cont = true;
    while cont {
        cwd::print_cwd();
        match io::stdin().read_line(&mut input) {
            Ok(inp) => cont = inp == 1,
            Err(_) => cont = true,
        }
    }
    utils::split_input(&input)
}
