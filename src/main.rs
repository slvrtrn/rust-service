use globals::CONFIG;

use crate::kafka::KafkaConsumers;

mod avro;
mod globals;
mod http;
mod kafka;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    lazy_static::initialize(&CONFIG);
    env_logger::init();
    log::info!("Starting the app in {} mode", &CONFIG.rust_env);
    let kafka_consumers = KafkaConsumers::new();
    tokio::try_join!(kafka_consumers.start(), http::init_http_server())?;
    Ok(())
}
