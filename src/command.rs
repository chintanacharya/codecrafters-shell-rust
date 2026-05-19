use std::{env, fs::File, os::unix::fs::PermissionsExt, path::PathBuf};

use crate::builtin::{Builtin, parse_builtin};

pub enum ResolveResult {
    Builtin(Builtin),
    Command(PathBuf),
    NotFound,
}

pub fn resolve_command(cmd: &str) -> ResolveResult {
    let builtin_opt = parse_builtin(cmd);

    match builtin_opt {
        Some(builtin) => ResolveResult::Builtin(builtin),
        None => {
            let mut found_candidate = None;
            for candidate in path_candidates() {
                let target_path = candidate.join(cmd);
                let result = find_exe_at_path(&target_path);
                if result.is_some_and(|val: bool| val) {
                    found_candidate = Some(target_path);
                    break;
                }
            }

            match found_candidate {
                None => ResolveResult::NotFound,
                Some(exe_path) => ResolveResult::Command(exe_path),
            }
        }
    }
}

fn path_candidates() -> Vec<PathBuf> {
    let path_res = env::var("PATH");
    match path_res {
        Err(_) => panic!("PATH not found"),
        Ok(path_var) => env::split_paths(&path_var).collect(),
    }
}

fn find_exe_at_path(target_path: &PathBuf) -> Option<bool> {
    // TODO: make this OS independent using the is_executable crate
    let file = File::open(target_path).ok()?;
    let permissions = file.metadata().ok()?.permissions();

    Some(permissions.mode() & 0o111 != 0)
}
