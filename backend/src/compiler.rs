use crate::utils::{execute_command, wrap_err};
use serde::{Deserialize, Serialize};
use std::{
    env::temp_dir,
    fs::{remove_file, File},
    io::Write,
    process::Command,
};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct CompilerResult {
    pub code: i32,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Debug)]
pub(crate) enum ProgramError {
    EnvMissing,
    FileCreate,
    FileRead,
    FileWrite,
    ChildSpawn,
    ChildWaitWithOutput,
    UTF8Conversion,
}

pub(crate) fn compile(code: &str, execute: bool) -> Result<CompilerResult, ProgramError> {
    // Get the paths for jakt file and cpp output
    let tmp_path = String::from(temp_dir().to_str().unwrap_or_else(|| "/tmp"));
    let filename = format!("{}/jakt-{}", tmp_path, Uuid::new_v4());
    let filename_jakt = format!("{}.jakt", filename);
    let filename_cpp = format!("{}.cpp", filename);
    let jakt_home = wrap_err!(std::env::var("JAKT_HOME"), ProgramError::EnvMissing)?;

    // Write jakt file to a temp directory
    let mut tmp_file = wrap_err!(File::create(&filename_jakt), ProgramError::FileCreate)?;
    wrap_err!(tmp_file.write(code.as_bytes()), ProgramError::FileWrite)?;

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
            ProgramError::FileRead
        )?;
        // It's fine if the removing file failed
        let _ = remove_file(filename_cpp);
        let _ = remove_file(filename_jakt);
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

    // It's fine if the removing file failed
    let _ = remove_file(filename_cpp);
    let _ = remove_file(filename_jakt);
    return result;
}
