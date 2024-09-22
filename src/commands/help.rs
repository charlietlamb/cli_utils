pub fn help() {
    println!("--RUST SHELL COMMANDS--");
    println!("  cd: Change directory as you would in any unix terminal");
    println!("  ls: List all files in current directory");
    println!("    -a: Lists dotfiles");
    println!("echo: Repeats input command");
    println!(" cat: Outputs contents of given file");
    println!("find: Finds a file whose name contains a string");
    println!("grep: Finds a string in a file");
    println!("    -c: Output as a count of finds");
    println!("    -i: Ignore case");
    println!("    -v: Reverse condition");
    println!("    -n: Add line numbers");
    println!("     *: Search all files in directory");
}
