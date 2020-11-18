use std::path::PathBuf;

mod environ;
mod cmdline;

use dirs_next;

use clap::{App, Arg, SubCommand};

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
    let matches = App::new("devboot")
        .version("0.1.0")
        .author("Andrew Luchuk")
        .about("Boots your development environment with a single command")
        .subcommand(
            SubCommand::with_name("new")
            .about("creates a new project")
            .arg(
                Arg::with_name("project-name")
                .required(true)
                .takes_value(true)
                .value_name("NAME")
                .help("the name of the new project")
            )
            .arg(
                Arg::with_name("proc-file")
                .required(true)
                .takes_value(true)
                .value_name("PROCEDURE NAME")
                .help("the path of the procedure file")
            )
        )
        .arg(
            Arg::with_name("project")
                .takes_value(true)
                .value_name("PROJECT")
                .help("the name of the project to boot")
        )
        .get_matches();
}
