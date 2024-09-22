use crate::commands::cat;
use crate::commands::cd;
use crate::commands::echo;
use crate::commands::find;
use crate::commands::grep;
use crate::commands::ls;
pub fn command(input: Vec<String>) {
    match input[0].trim() {
        "cd" => cd::cd(input),
        "ls" => ls::ls(input),
        "echo" => echo::echo(input),
        "find" => find::find(input),
        "cat" => cat::cat(input),
        "grep" => grep::grep(input),
        "exit" => std::process::exit(0),
        _ => println!("Please enter a valid command.\nRun help to learn more."),
    };
}
