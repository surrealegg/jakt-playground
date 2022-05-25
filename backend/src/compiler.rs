use std::{
    env::temp_dir,
    fs::{remove_file, File},
    io::Write,
    process::{Command, Stdio},
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct CompilerResult {
    pub code: i32,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Debug)]
pub enum CompileError {
    FileCreate,
    FileRead,
    FileWrite,
    ChildSpawn,
    ChildWaitWithOutput,
    UTF8Conversion,
}

macro_rules! wrap_err {
    ($value: expr, $err: expr) => {
        $value.or_else(|_| Err($err))
    };
}

fn execute_command(command: &mut Command) -> Result<CompilerResult, CompileError> {
    let child = command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .or_else(|_| Err(CompileError::ChildSpawn))?;
    let output = wrap_err!(child.wait_with_output(), CompileError::ChildWaitWithOutput)?;
    Ok(CompilerResult {
        code: output.status.code().unwrap_or(0),
        stdout: wrap_err!(
            String::from_utf8(output.stdout),
            CompileError::UTF8Conversion
        )?,
        stderr: wrap_err!(
            String::from_utf8(output.stderr),
            CompileError::UTF8Conversion
        )?,
    })
}

pub fn compile(code: &str, execute: bool) -> Result<CompilerResult, CompileError> {
    let tmp_path = String::from(temp_dir().to_str().unwrap_or_else(|| "/tmp"));
    let filename = format!("{}/jakt-{}", tmp_path, Uuid::new_v4());
    let filename_jakt = format!("{}.jakt", filename);
    let filename_cpp = format!("{}.cpp", filename);
    let jakt_home = std::env::var("JAKT_HOME").unwrap();

    let mut tmp_file = wrap_err!(File::create(&filename_jakt), CompileError::FileCreate)?;
    wrap_err!(tmp_file.write(code.as_bytes()), CompileError::FileWrite)?;

    // Transpile jakt code
    execute_command(
        Command::new("timeout")
            .arg("5")
            .arg("jakt")
            .arg("-o")
            .arg(&tmp_path)
            .arg(&filename_jakt),
    )?;

    // If only thing needed is showing the c++ code, just output the result
    if !execute {
        let result = wrap_err!(
            std::fs::read_to_string(&filename_cpp),
            CompileError::FileRead
        )?;
        let _ = remove_file(&filename_cpp);
        return Ok(CompilerResult {
            code: 0,
            stdout: result,
            stderr: "".to_string(),
        });
    }

    // Run clang on docker
    let result = execute_command(
        Command::new("docker")
            .arg("run")
            .arg("--rm")
            .arg("--net=none")
            .arg("--memory=100m")
            .arg("--memory-swap=100m")
            .arg("--cpus=0.8")
            .arg(format!(
                "--volume={}:/playground/input.cpp:ro",
                filename_cpp
            ))
            .arg(format!(
                "--volume={}/runtime:/usr/local/include/runtime:ro",
                jakt_home
            ))
            .arg("jakt_sandbox"),
    );

    let _ = remove_file(filename_cpp);
    let _ = remove_file(filename_jakt);
    return result;
}
