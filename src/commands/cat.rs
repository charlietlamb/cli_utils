use crate::cwd::get_cwd;
use std::fs;

pub fn cat(input: Vec<String>) {
    if input.len() < 2 {
        return println!("Please provide a file.");
    }
    let mut cwd = get_cwd();
    cwd.push(&input[1]);
    match fs::read_to_string(cwd) {
        Ok(string) => println!("{string}"),
        Err(e) => println!("Couldn't read file! {e}"),
    }
}
