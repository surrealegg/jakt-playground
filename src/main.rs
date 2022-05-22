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

#[async_std::main]
async fn main() -> tide::Result<()> {
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
