use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use super::cmdline::CmdLine;

pub struct Environ {
    name: String,
    config_opts: Vec<String>,
    proc_file: String,
}

impl Environ {
    fn open_file(file_path: &String) -> File {
        let f = File::open(file_path);
        match f.ok() {
            Some(file) => file,
            None => panic!("Could not open file: {:?}", file_path)
        }
    }

    fn open_proc(&self) -> File {
        Environ::open_file(&self.proc_file)
    }

    fn load_cmds(reader: BufReader<File>) -> Vec<String> {
        let mut lines = Vec::new();
        for line in reader.lines() {
            if let io::Result::Ok(l) = line {
                if !l.is_empty() {
                    lines.push(l);
                }
            }
        }
        lines
    }

    fn strs_to_cmds(cmd_strs: Vec<String>) -> Vec<CmdLine> {
        let mut cmds = Vec::new();
        for s in cmd_strs.iter() {
            cmds.push(CmdLine::parse_str(s));
        }
        cmds
    }

    pub fn read_proc(&self) -> Vec<CmdLine> {
        let f = self.open_proc();
        let reader = BufReader::new(f);
        let cmds = Environ::load_cmds(reader);
        Environ::strs_to_cmds(cmds)
    }
}