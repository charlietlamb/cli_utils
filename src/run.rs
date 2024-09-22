use std::io;
use crate::command::command;
use crate::utils;
use crate::cwd;

pub fn run(){
    loop{
        let input = get_next_input();
        command(input);
    }
}


fn get_next_input() -> Vec<String>{
    let mut input = String::new();
    cwd::print_cwd();
    io::stdin().read_line(&mut input).expect("Failed tor read input");
    utils::split_input(&input)

}
