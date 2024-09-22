use std::io::Write;
use std::path::PathBuf;
use std::{env, io};

pub fn get_cwd() -> PathBuf {
    match env::current_dir() {
        Ok(current_path) => current_path,
        Err(e) => {
            eprintln!("Failed to get cwd: {e}");
            std::process::exit(1);
        }
    }
}

pub fn set_cwd(path: PathBuf) {
    if !path.is_dir() {
        return eprintln!("Directory not found ...");
    }
    match env::set_current_dir(path) {
        Ok(_) => (),
        Err(e) => eprintln!("Directory not found ... :{e}"),
    }
}

pub fn print_cwd() {
    print!("{}:", get_cwd().display());
    match io::stdout().flush() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Failed to flush stdout(): {e}");
            std::process::exit(1);
        }
    }
}
