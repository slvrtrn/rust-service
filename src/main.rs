use globals::CONFIG;

mod db;
mod globals;
mod http;
mod kafka;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    lazy_static::initialize(&CONFIG);
    env_logger::init();
    log::info!("Starting the app in {} mode", &CONFIG.rust_env);
    let pool = db::connect().await?;
    sqlx::query!("SELECT 1 as test").fetch_one(&*pool).await?;
    tokio::try_join!(kafka::run_consumers(), http::init_http_server())?;
    Ok(())
}
