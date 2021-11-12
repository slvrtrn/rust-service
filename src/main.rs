use globals::CONFIG;

mod globals;
mod http;
mod kafka;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    lazy_static::initialize(&CONFIG);
    env_logger::init();
    log::info!("Starting the app in {} mode", &CONFIG.rust_env);
    kafka::init_kafka_consumer();
    http::init_http_server().await
}
