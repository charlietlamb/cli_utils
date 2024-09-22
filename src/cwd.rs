use std::{env, io};
use std::io::Write;
use std::path::PathBuf;

pub fn get_cwd() -> PathBuf{
    match env::current_dir(){
        Ok(current_path) => {
            current_path
        },
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    }
}

pub fn set_cwd(path: PathBuf){
    if !path.is_dir(){
        return eprintln!("Directory not found ...")
    }
    match env::set_current_dir(path){
        Ok(_) => (),
        Err(e) => eprintln!("Directory not found ... :{e}")
    }
}

pub fn print_cwd(){
    let cwd = get_cwd();
    print!("{}: ",cwd.display());
    match io::stdout().flush(){
        Ok(_) => (),
        Err(e) => {
            println!("Failed to flush stdout(): {e}");
            std::process::exit(1);
        }
    }
}
