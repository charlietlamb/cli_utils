pub fn echo(input: Vec<String>) {
    for (i, inp) in input.iter().enumerate() {
        if i == 0 {
            continue;
        }
        print!("{inp} ");
    }
    print!("\n");
}
