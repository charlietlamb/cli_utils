use crate::cwd::get_cwd;
use std::fs;

struct LsFlags {
    all: bool,
}

impl LsFlags {
    fn new() -> Self {
        Self { all: false }
    }
}

pub fn ls(input: Vec<String>) {
    let cwd = get_cwd();
    let flags = set_ls_flags(&input);
    match fs::read_dir(cwd) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        let dsp = path.display().to_string();
                        let entry_name = dsp.split('/').last().unwrap_or_else(|| "");
                        if !flags.all && entry_name.starts_with('.') {
                            continue;
                        }
                        if path.is_dir() {
                            print!("D: {}, ", entry_name);
                        } else {
                            print!("F: {}, ", entry_name);
                        }
                    }
                    Err(e) => eprintln!("Failed to read entry: {e}"),
                }
            }
            print!("\n");
        }
        Err(e) => println!("Failed to read div: {e}"),
    }
}

fn set_ls_flags(input: &Vec<String>) -> LsFlags {
    let mut flags = LsFlags::new();
    for inp in input {
        match inp.trim() {
            "-a" => flags.all = true,
            _ => (),
        }
    }
    flags
}
