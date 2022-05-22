use std::{env, fs, process::Stdio};

use async_std::process::Command;
use compiler::compile;
use tide::{
    http::headers::HeaderValue,
    prelude::*,
    security::{CorsMiddleware, Origin},
    Body, Request, Response,
};

mod compiler;

#[derive(Deserialize)]
#[serde(default)]
struct CompileRequest {
    execute: bool,
}

impl Default for CompileRequest {
    fn default() -> Self {
        Self { execute: false }
    }
}

async fn compile_or_execute(mut req: Request<()>) -> tide::Result {
    let code = req.body_string().await?;
    let params: CompileRequest = req.query()?;
    match compile(&code, params.execute) {
        Ok(result) => {
            let mut response = Response::new(200);
            response.set_body(Body::from_json(&result)?);
            Ok(response)
        }
        Err(_) => todo!(),
    }
}

fn docker_exists() -> bool {
    if let Ok(paths) = env::var("PATH") {
        for path in paths.split(":") {
            if fs::metadata(format!("{}/docker", path)).is_ok() {
                return true;
            }
        }
    }
    false
}

async fn has_docker_image() -> bool {
    if let Ok(child) = Command::new("docker")
        .arg("images")
        .stdout(Stdio::piped())
        .spawn()
    {
        if let Ok(output) = child.output().await {
            if let Ok(string) = String::from_utf8(output.stdout) {
                return string.contains("jakt_sandbox");
            }
        }
    }
    false
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    if !docker_exists() {
        eprintln!("Docker not found! Please install it: https://docs.docker.com/get-docker/");
        return Ok(());
    }

    if !has_docker_image().await {
        eprintln!("Docker image jakt_sandbox is missing. Have you ran 'sh ./sandbox/setup.sh'?");
        return Ok(());
    }

    let mut app = tide::new();
    // FIXME: Let's not allow * origin
    app.with(
        CorsMiddleware::new()
            .allow_methods("POST".parse::<HeaderValue>().unwrap())
            .allow_origin(Origin::from("*"))
            .allow_credentials(false),
    );
    app.at("/compile").post(compile_or_execute);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
