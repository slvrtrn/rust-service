// See https://github.com/fede1024/rust-rdkafka/blob/6fb2c37/examples/asynchronous_processing.rs

use futures::stream::FuturesUnordered;
use futures::{StreamExt, TryStreamExt};
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::{ClientConfig, Message};

use crate::globals::CONFIG;

async fn run_inbox_consumer() {
    // Create the `StreamConsumer`, to receive the messages from the topic in form of a `Stream`.
    log::info!("Starting the inbox stream processing");
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", &CONFIG.kafka_inbox_group_id)
        .set("bootstrap.servers", &CONFIG.kafka_bootstrap_server)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "false")
        .create()
        .expect("Consumer creation failed");
    consumer
        .subscribe(&[&CONFIG.kafka_inbox_topic])
        .expect("Can't subscribe to specified topic");
    // Create the outer pipeline on the message stream.
    let stream_processor = consumer
        .stream()
        .try_for_each(|borrowed_message| {
            async move {
                // Process each message
                match borrowed_message.payload_view::<str>() {
                    Some(Ok(payload)) => {
                        log::info!("Got a new inbox message: {}", payload)
                    }
                    Some(Err(_)) => {
                        log::info!(
                            "Inbox message with offset {} payload is not a string",
                            borrowed_message.offset()
                        )
                    }
                    None => {
                        log::info!(
                            "Inbox message with offset {} without payload",
                            borrowed_message.offset()
                        )
                    }
                }
                Ok(())
            }
        })
        .await
        .expect("Inbox stream processing failed");
    log::info!("Inbox stream processing terminated");
}

pub async fn init_kafka_consumers() -> () {
    (0..CONFIG.kafka_inbox_num_workers)
        .map(|_| tokio::spawn(run_inbox_consumer()))
        .collect::<FuturesUnordered<_>>()
        .for_each(|_| async { () })
        .await
}
