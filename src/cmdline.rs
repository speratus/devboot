
pub struct CmdLine {
    command: String,
    args: Vec<String>,
}

impl CmdLine {
    pub fn parse_str(cmd_line: &String) -> Self {
        let mut parts = Vec::new();
        for p in cmd_line.split(' ') {
            parts.push(String::from(p));
        }
        let cmd = parts.remove(0);

        CmdLine {
            command: String::from(cmd),
            args: parts
        }
    }
}