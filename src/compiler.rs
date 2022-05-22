use std::{
    env::temp_dir,
    fs::{remove_file, File},
    io::{Result, Write},
    process::{Command, Stdio},
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct CompilerResult {
    pub success: bool,
    pub output: String,
    pub error: String,
}

pub fn compile(code: &str, execute: bool) -> Result<CompilerResult> {
    let temp_file_path = format!(
        "{}/jakt-{}.jakt",
        temp_dir().to_str().unwrap(),
        Uuid::new_v4()
    );

    let mut temp_file = File::create(&temp_file_path)?;
    temp_file.write(code.as_bytes())?;

    let result = Command::new("docker")
        .arg("run")
        .arg("--rm")
        .arg("--net=none")
        .arg("--memory=100m")
        .arg("--memory-swap=100m")
        .arg("--cpus=0.8")
        .arg(format!("--env=CODEGEN={}", if execute { 0 } else { 1 }))
        .arg(format!(
            "--volume={}:/playground/input.jakt",
            temp_file_path
        ))
        .arg("jakt_sandbox")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let output = result.wait_with_output()?;
    remove_file(temp_file_path)?;
    Ok(CompilerResult {
        success: output.status.success(),
        output: String::from_utf8(output.stdout).unwrap(),
        error: String::from_utf8(output.stderr).unwrap(),
    })
}
