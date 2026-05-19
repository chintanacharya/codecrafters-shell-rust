use std::str::FromStr;

pub enum Builtin {
    Echo,
    Exit,
    Type,
    PWD,
}

impl FromStr for Builtin {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "echo" => Ok(Builtin::Echo),
            "exit" => Ok(Builtin::Exit),
            "type" => Ok(Builtin::Type),
            "pwd" => Ok(Builtin::PWD),
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

pub fn is_builtin(cmd: &str) -> bool {
    parse_builtin(cmd).is_some()
}
