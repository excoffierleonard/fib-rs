use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, Responder,
    body::BoxBody,
    middleware::{Compress, Logger},
    post,
    web::Json,
};
use env_logger::{self, Env};
use serde::{Deserialize, Serialize};
use std::{env, io::Result};

#[actix_web::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let port = env::var("PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(8080);

    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .wrap(Compress::default())
            .wrap(Logger::default())
            .service(fib_handler)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

#[derive(Debug, Deserialize)]
struct FibArgs {
    n: u128,
}

#[derive(Debug, Serialize)]
struct FibResponse {
    #[serde(rename = "F")]
    f: String,
}

impl Responder for FibResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}

#[post("/fib")]
async fn fib_handler(args: Json<FibArgs>) -> FibResponse {
    FibResponse {
        f: fib::fib(args.n).to_string(),
    }
}
