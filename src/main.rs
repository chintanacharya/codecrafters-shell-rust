#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage
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
    match cmd {
        "exit" => std::process::exit(0),
        "echo" => println!("{}", line),
        _ => println!("{cmd}: command not found"),
    }
}
