use std::{env, io::ErrorKind, str::FromStr};

use crate::command::{ResolveResult, resolve_command};

pub enum Builtin {
    Echo,
    Exit,
    Type,
    PWD,
    Cd,
}

impl FromStr for Builtin {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "echo" => Ok(Builtin::Echo),
            "exit" => Ok(Builtin::Exit),
            "type" => Ok(Builtin::Type),
            "pwd" => Ok(Builtin::PWD),
            "cd" => Ok(Builtin::Cd),
            _ => Err(()),
        }
    }
}

pub fn parse_builtin(cmd: &str) -> Option<Builtin> {
    let result: Result<Builtin, _> = cmd.parse();
    match result {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}

pub fn process_builtin(builtin: &Builtin, args: Vec<&str>) {
    match builtin {
        Builtin::Echo => println!("{}", args.join(" ")),
        Builtin::Exit => std::process::exit(0),
        Builtin::Type => {
            let command = args[0];
            let resolved = resolve_command(command);
            match resolved {
                ResolveResult::Builtin(_) => println!("{command} is a shell builtin"),
                ResolveResult::Command(command_path) => {
                    println!("{} is {}", command, command_path.display())
                }
                ResolveResult::NotFound => println!("{command}: not found"),
                ResolveResult::InvalidPath => println!("{command}: invalid or blank command"),
            }
        }
        Builtin::PWD => {
            let cwd_result = env::current_dir();
            match cwd_result {
                Ok(dir) => println!("{}", dir.display()),
                Err(_) => {
                    // TODO: error handling is not specified
                }
            }
        }
        Builtin::Cd => {
            if args.len() > 1 {
                eprintln!("cd: too many args")
            }

            let dir = args[0];

            if dir == "~" {
                let env_var_result = env::var("HOME");
                match env_var_result {
                    Ok(home_dir) => switch_to_dir(&home_dir),
                    Err(_) => {
                        eprintln!("cd: cannot switch to ~: HOME not set");
                        return;
                    }
                }
            } else {
                switch_to_dir(dir)
            }
        }
    }
}

fn switch_to_dir(target_dir: &str) {
    let cd_result = env::set_current_dir(target_dir);
    match cd_result {
        Ok(_) => {}
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                eprintln!("cd: {}: No such file or directory", target_dir)
            } else {
                eprintln!("cd: failed to switch to {}: {}", target_dir, e)
            }
        }
    }
}
