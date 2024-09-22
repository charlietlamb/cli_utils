use crate::commands::find::get_items;
use crate::commands::grep::{print_grep, print_grep_count};
use crate::commands::grep_flags::GrepFlags;
use std::fs;
use std::path::PathBuf;

pub fn grep_all(inp: &String, path: &PathBuf, grep_flags: &GrepFlags) {
    let items = get_items(path.to_path_buf());
    let dirs: Vec<&PathBuf> = items.iter().filter(|i| i.is_dir()).collect();
    let files: Vec<&PathBuf> = items.iter().filter(|i| !i.is_dir()).collect();
    for f in files {
        grep_file(inp, f, grep_flags);
    }
    for d in dirs {
        grep_all(inp, d, grep_flags);
    }
}

fn grep_file(inp: &String, f: &PathBuf, grep_flags: &GrepFlags) {
    let file_path = f.display().to_string();
    let file_path = file_path.split('/').last().unwrap_or_else(|| "");
    let file_str: String;
    match fs::read_to_string(f) {
        Ok(string) => file_str = string,
        Err(e) => return eprintln!("{e}"),
    }
    let file_lines: Vec<&str> = file_str.split('\n').collect();
    if grep_flags.count {
        print_grep_count(inp, &file_lines, grep_flags, file_path);
    } else {
        print_grep(inp, &file_lines, grep_flags, file_path);
    }
}
