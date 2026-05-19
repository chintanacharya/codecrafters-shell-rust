#[allow(unused_imports)]
use std::io::{self, Write};
use std::{os::unix::process::CommandExt, path::PathBuf, process::Command};

use crate::{
    builtin::Builtin,
    command::{ResolveResult, resolve_command},
};

pub mod builtin;
pub mod command;

fn main() {
    repl();
}

fn repl() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut raw_line = String::new();
        io::stdin().read_line(&mut raw_line).unwrap();
        let trimmed_line = raw_line.trim_end();

        process_line(&trimmed_line);
    }
}

fn process_line(line: &str) {
    let cmd_term_index = line.find(|c: char| c.is_whitespace());
    match cmd_term_index {
        Some(index) => process_cmd(&line[0..index], &line[index..].trim()),
        None => process_cmd(line, ""),
    }
}

fn process_cmd(cmd: &str, line: &str) {
    let resolved = resolve_command(cmd);
    match resolved {
        ResolveResult::Builtin(builtin) => process_builtin(&builtin, line),
        ResolveResult::Command(command_path) => process_exe(&command_path, line),
        ResolveResult::NotFound => println!("{cmd}: command not found"),
        ResolveResult::InvalidPath => println!("{cmd}: invalid or blank command"),
    }
}

fn process_exe(target_path: &PathBuf, line: &str) {
    let _ = Command::new(target_path)
        .arg0(target_path.file_name().unwrap_or(target_path.as_os_str()))
        .args(line.split_ascii_whitespace())
        .stdout(io::stdout())
        .stderr(io::stderr())
        .output()
        .expect(
            format!(
                "Failed to run executable: {}",
                target_path.to_string_lossy()
            )
            .as_str(),
        );
}

fn process_builtin(builtin: &Builtin, line: &str) {
    match builtin {
        Builtin::Echo => println!("{}", line),
        Builtin::Exit => std::process::exit(0),
        Builtin::Type => {
            let resolved = resolve_command(line);
            match resolved {
                ResolveResult::Builtin(_) => println!("{line} is a shell builtin"),
                ResolveResult::Command(command_path) => {
                    println!("{} is {}", line, command_path.to_string_lossy())
                }
                ResolveResult::NotFound => println!("{line}: not found"),
                ResolveResult::InvalidPath => println!("{line}: invalid or blank command"),
            }
        }
    }
}
