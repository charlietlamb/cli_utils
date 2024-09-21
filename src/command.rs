pub fn command(input: String){
    match input.trim(){
        "exit" => std::process::exit(0),
        _ => println!("Please enter a valid command.\nRun help to learn more.")
    };
}
