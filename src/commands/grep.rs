use crate::commands::grep_all::grep_all;
use crate::commands::grep_flags::GrepFlags;
use crate::cwd::get_cwd;
use std::fs;

pub fn grep(input: Vec<String>) {
    if input.len() < 3 {
        return println!("Please specify a string and a file! ");
    }
    let mut cwd = get_cwd();
    let grep_flags = get_grep_flags(&input);
    if grep_flags.all_files {
        return grep_all(&input[1], &cwd, &grep_flags);
    }
    cwd.push(&input[2]);
    let file_contents: String;
    match fs::read_to_string(cwd) {
        Ok(string) => file_contents = string,
        Err(e) => return println!("Couldn't read file! {e}"),
    }
    let file_lines: Vec<&str> = file_contents.split('\n').collect();
    if grep_flags.count {
        return print_grep_count(&input[1], &file_lines, &grep_flags, "");
    }
    print_grep(&input[1], &file_lines, &grep_flags, "")
}

fn get_grep_flags(inp: &Vec<String>) -> GrepFlags {
    let mut grep_flags = GrepFlags::new();
    for i in inp {
        match i.trim() {
            "-v" => grep_flags.reverse = true,
            "-n" => grep_flags.lines = true,
            "-c" => grep_flags.count = true,
            "-i" => grep_flags.ignore_case = true,
            "*" => grep_flags.all_files = true,
            _ => (),
        }
    }
    grep_flags
}

pub fn print_grep(inp: &String, file_lines: &Vec<&str>, grep_flags: &GrepFlags, path: &str) {
    for (i, line) in file_lines.iter().enumerate() {
        if grep_match(line, &inp, &grep_flags) {
            if grep_flags.all_files {
                print!("{path}:");
            }
            if grep_flags.lines {
                print!("{i}:");
            }
            println!("{line}");
        }
    }
}

pub fn print_grep_count(inp: &String, file_lines: &Vec<&str>, grep_flags: &GrepFlags, path: &str) {
    let mut count = 0;
    for line in file_lines {
        if grep_match(&line, inp, &grep_flags) {
            count += 1;
        }
    }
    if grep_flags.all_files {
        print!("{path}:");
    }
    if grep_flags.all_files && count > 0 || !grep_flags.all_files {
        println!("{count}")
    }
}

pub fn grep_match(line: &&str, inp: &String, grep_flags: &GrepFlags) -> bool {
    let inp = inp.trim();
    if grep_flags.ignore_case {
        if line.to_lowercase().contains(&inp.to_lowercase()) && !grep_flags.reverse {
            return true;
        }
        if !line.to_lowercase().contains(&inp.to_lowercase()) && grep_flags.reverse {
            return true;
        }
    }
    if line.contains(inp) && !grep_flags.reverse {
        return true;
    }
    if !line.contains(inp) && grep_flags.reverse {
        return true;
    }
    false
}
