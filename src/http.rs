use actix_web::{get, Responder};

#[get("/healthz")]
pub async fn healthz() -> impl Responder {
    format!("ok")
}
