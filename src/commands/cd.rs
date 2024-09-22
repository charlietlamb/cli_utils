use crate::cwd;
use std::path::Path;

pub fn cd(input: Vec<String>) {
    if input.len() < 2 {
        cd_root();
    } else {
        cd_new(&input[1]);
    }
}

fn cd_root() {
    let root_path = Path::new("/");
    cwd::set_cwd(root_path.to_path_buf());
}
fn cd_new(new_dir: &String) {
    let mut path = cwd::get_cwd();
    path.push(new_dir);
    cwd::set_cwd(path);
}
