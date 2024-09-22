use crate::commands::cd;
pub fn command(input: Vec<String>){
    match input[0].trim(){
        "cd" => cd::cd(input),
        "exit" => std::process::exit(0),
        _ => println!("Please enter a valid command.\nRun help to learn more.")
    };
}
