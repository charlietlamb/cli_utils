use std::io::{self, Write};
use std::env;
use crate::command::command;

pub fn run(){
    loop{
        let input = get_next_input();
        command(input);
    }
}

fn print_cwd(){
    match env::current_dir(){
        Ok(path) => {
            print!("{}: ",path.display());
            match io::stdout().flush(){
                Ok(_) => (),
                Err(e) => {
                    println!("Failed to flush stdout(): {e}");
                    std::process::exit(1);
                }

            }
        },
        Err(e) => {
            println!("{e}");
            std::process::exit(1);
        }

    };
}

fn get_next_input() -> String{
    let mut input = String::new();
    print_cwd();
    io::stdin().read_line(&mut input).expect("Failed tor read input");
    input

}
