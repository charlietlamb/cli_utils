use crate::cwd::get_cwd;
use std::fs;
use std::path::PathBuf;

pub fn find(input: Vec<String>) {
    let path = get_cwd();
    find_files(&input[1], &path);
}

fn find_files(inp: &String, path: &PathBuf) {
    let items = get_items(path.to_path_buf());
    let dirs: Vec<&PathBuf> = items.iter().filter(|i| i.is_dir()).collect();
    let files: Vec<&PathBuf> = items.iter().filter(|i| !i.is_dir()).collect();
    for f in files {
        if let Some(file_str) = f.to_str() {
            if file_str.contains(inp) {
                println!("{file_str}");
            }
        }
    }
    for d in dirs {
        find_files(inp, d);
    }
}

pub fn get_items(path: PathBuf) -> Vec<PathBuf> {
    let mut items: Vec<PathBuf> = Vec::new();
    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        items.push(path);
                    }
                    Err(e) => eprintln!("Failed to read entry: {e}"),
                }
            }
        }
        Err(e) => println!("Failed to read div: {e}"),
    }
    items
}
