pub fn split_input(input: &String) -> Vec<String>{
    input.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>()
}
