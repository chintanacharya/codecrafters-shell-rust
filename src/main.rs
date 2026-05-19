#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();

    let mut raw_cmd = String::new();
    io::stdin().read_line(&mut raw_cmd).unwrap();
    let trimmed_cmd = raw_cmd.trim_end();

    println!("{trimmed_cmd}: command not found");
}
