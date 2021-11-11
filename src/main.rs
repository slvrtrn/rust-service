use actix_web::{App, HttpServer};

mod http;

const ADDR: &str = "127.0.0.1:3003";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(http::healthz))
        .bind(ADDR)?
        .run()
        .await
}
