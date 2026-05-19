use crate::builtin::{Builtin, parse_builtin};

pub enum ResolveResult<'a> {
    Builtin(Builtin),
    Command(&'a str),
    NotFound,
}

pub fn resolve_command<'a>(cmd: &'a str) -> ResolveResult<'a> {
    let builtin_opt = parse_builtin(cmd);

    match builtin_opt {
        Some(builtin) => ResolveResult::Builtin(builtin),
        None => {
            // TODO: resolve command from PATH
            ResolveResult::NotFound
        }
    }
}
