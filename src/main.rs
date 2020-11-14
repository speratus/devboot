use std::path::PathBuf;

mod environ;
mod cmdline;

use dirs_next;

pub fn get_config_dir() -> PathBuf {
    let mut home_dir;
    if let Some(dir) = dirs_next::home_dir() {
        home_dir = dir;
    } else {
        panic!("User home directory doesn't exist!")
    }
    home_dir.push(".devboot");
    home_dir
}

fn main() {
    println!("Hello, world!");
}
