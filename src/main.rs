use std::io::{self, Write};

use crate::{
    builtin::process_builtin,
    command::{ResolveResult, process_exe, resolve_command},
};

pub mod builtin;
pub mod command;
pub mod parser;

fn main() {
    repl();
}

fn repl() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut raw_line = String::new();
        io::stdin().read_line(&mut raw_line).unwrap();
        let trimmed_line = raw_line.trim();

        // Do not try to process blank lines
        if trimmed_line.len() > 0 {
            process_line(&trimmed_line);
        }
    }
}

fn process_line(line: &str) {
    let split_result = line.split_once(|c: char| c.is_whitespace());
    match split_result {
        None => process_cmd(line, Vec::new()),
        Some((command, line)) => process_cmd(
            command.trim(),
            line.trim()
                .split(|c: char| c.is_whitespace())
                .filter(|s| s.len() > 0)
                .collect(),
        ),
    }
}

fn process_cmd(cmd: &str, args: Vec<&str>) {
    let resolved = resolve_command(cmd);
    match resolved {
        ResolveResult::Builtin(builtin) => process_builtin(&builtin, args),
        ResolveResult::Command(command_path) => process_exe(&command_path, args),
        ResolveResult::NotFound => println!("{cmd}: command not found"),
        ResolveResult::InvalidPath => println!("{cmd}: invalid or blank command"),
    }
}
