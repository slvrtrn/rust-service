// See https://github.com/fede1024/rust-rdkafka/blob/6fb2c37/examples/asynchronous_processing.rs

use rdkafka::consumer::StreamConsumer;
use rdkafka::ClientConfig;

use crate::globals::CONFIG;

pub fn init_kafka_consumer() {
    // Create the `StreamConsumer`, to receive the messages from the topic in form of a `Stream`.
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", &CONFIG.kafka_group_id)
        .set("bootstrap.servers", &CONFIG.kafka_bootstrap_server)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "false")
        .create()
        .expect("Consumer creation failed");
}
