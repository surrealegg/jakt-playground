use compiler::compile;
use tide::{
    http::headers::HeaderValue,
    log::{self, error, warn, LogMiddleware},
    prelude::*,
    security::{CorsMiddleware, Origin},
    Body, Request, Response,
};
use tide_governor::GovernorMiddleware;
use utils::{env_get, has_docker_image, program_exists};

mod compiler;
mod utils;

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
    let code = String::from(req.body_string().await?.trim());
    if code.len() == 0 {
        return Ok(Response::new(400));
    }

    let params: CompileRequest = req.query()?;
    match compile(&code, params.execute) {
        Ok(result) => {
            let mut response = Response::new(200);
            response.set_body(Body::from_json(&result)?);
            Ok(response)
        }
        Err(err) => {
            error!("{:#?}", err);
            Ok(Response::new(500))
        }
    }
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    log::start();

    if !program_exists("jakt") {
        error!("Jakt not found. Have you ran 'cargo install --path . inside jakt repository?'");
        return Ok(());
    }

    if !program_exists("docker") {
        error!("Docker not found. Please install it: https://docs.docker.com/get-docker/");
        return Ok(());
    }

    if !has_docker_image() {
        error!("Docker image jakt_sandbox is missing. Have you ran 'sh ./sandbox/setup.sh'?");
        return Ok(());
    }

    if std::env::var("JAKT_HOME").is_err() {
        error!("JAKT_HOME is not set");
        return Ok(());
    }

    let mut app = tide::new();
    app.with(
        CorsMiddleware::new()
            .allow_methods("POST".parse::<HeaderValue>().unwrap())
            .allow_origin(Origin::from(env_get!("ALLOW_ORIGIN", "*")))
            .allow_credentials(false),
    );
    app.with(LogMiddleware::new());

    // If needed serve static page at main directory
    if let Ok(static_page) = std::env::var("STATIC_PAGE") {
        app.at("/").serve_dir(static_page)?;
    }

    app.at("/compile")
        .with(GovernorMiddleware::per_minute(4)?)
        .post(compile_or_execute);
    app.listen(format!("127.0.0.1:{}", env_get!("PORT", "8080")))
        .await?;
    Ok(())
}
