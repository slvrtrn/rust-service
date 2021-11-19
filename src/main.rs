use globals::CONFIG;

mod globals;
mod http;
mod kafka;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    lazy_static::initialize(&CONFIG);
    env_logger::init();
    log::info!("Starting the app in {} mode", &CONFIG.rust_env);
    tokio::try_join!(kafka::run_consumers(), http::init_http_server())?;
    Ok(())
}
