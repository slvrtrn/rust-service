use actix_web::http::StatusCode;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

use crate::globals::CONFIG;

pub async fn init_http_server() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(healthz))
        .bind(&CONFIG.http_addr)?
        .run()
        .await
}

#[get("/healthz")]
pub async fn healthz() -> impl Responder {
    HttpResponse::build(StatusCode::OK).body("healthy")
}
