use std::process::Command;
use std::{env, fs, process::Stdio};

use crate::compiler::{CompilerResult, ProgramError};

macro_rules! wrap_err {
    ($value: expr, $err: expr) => {
        $value.or_else(|_| Err($err))
    };
}

macro_rules! env_get {
    ($name: expr, $default: expr) => {
        std::env::var($name).unwrap_or_else(|_| {
            warn!("Warning: {} is not set, using default {}", $name, $default);
            String::from($default)
        })
    };
}

pub(crate) use env_get;
pub(crate) use wrap_err;

pub(crate) fn program_exists(program: &str) -> bool {
    if let Ok(paths) = env::var("PATH") {
        for path in paths.split(":") {
            if fs::metadata(format!("{}/{}", path, program)).is_ok() {
                return true;
            }
        }
    }
    false
}

pub(crate) fn execute_command(command: &mut Command) -> Result<CompilerResult, ProgramError> {
    let child = command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .or_else(|_| Err(ProgramError::ChildSpawn))?;
    let output = wrap_err!(child.wait_with_output(), ProgramError::ChildWaitWithOutput)?;
    Ok(CompilerResult {
        code: output.status.code().unwrap_or(0),
        stdout: wrap_err!(
            String::from_utf8(output.stdout),
            ProgramError::UTF8Conversion
        )?,
        stderr: wrap_err!(
            String::from_utf8(output.stderr),
            ProgramError::UTF8Conversion
        )?,
    })
}

pub(crate) fn has_docker_image() -> bool {
    if let Ok(child) = Command::new("docker")
        .arg("images")
        .stdout(Stdio::piped())
        .spawn()
    {
        if let Ok(output) = child.wait_with_output() {
            if let Ok(string) = String::from_utf8(output.stdout) {
                return string.contains("jakt_sandbox");
            }
        }
    }
    false
}
