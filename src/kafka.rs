// See https://github.com/fede1024/rust-rdkafka/blob/6fb2c37/examples/asynchronous_processing.rs
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::{ClientConfig, Message};

use crate::globals::CONFIG;

pub async fn run_consumers() -> anyhow::Result<()> {
    (0..CONFIG.kafka_inbox_num_workers)
        .map(|n| tokio::spawn(run_outbox_consumer(n + 1)))
        .collect::<FuturesUnordered<_>>()
        .for_each(|_| async { () })
        .await;
    Ok(())
}

async fn run_outbox_consumer(n: u8) -> () {
    log::info!("Starting the outbox stream consumer #{}", n);
    let consumer: StreamConsumer = ClientConfig::new()
        .set("client.id", &CONFIG.kafka_client_id)
        .set("debug", "all")
        .set("group.id", &CONFIG.kafka_inbox_group_id)
        .set("bootstrap.servers", &CONFIG.kafka_bootstrap_server)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "false")
        .set("reconnect.backoff.max.ms", "200")
        .create()
        .expect("Consumer creation failed");
    consumer
        .subscribe(&[&CONFIG.kafka_inbox_topic])
        .expect("Can't subscribe to specified topic");
    let mut message_stream = consumer.stream();
    // inspired by https://github.com/fede1024/rust-rdkafka/issues/304#issuecomment-747691813
    // graceful shutdown of the consumer
    while let Some(res) = {
        tokio::select! {
            msg = message_stream.next() => msg,
            _ = tokio::signal::ctrl_c() => None,
        }
    } {
        match res {
            Ok(borrowed_message) => {
                // Process each message
                match borrowed_message.payload_view::<str>() {
                    Some(Ok(payload)) => {
                        log::info!("Got a new message: {}", payload)
                    }
                    Some(Err(_)) => {
                        log::info!(
                            "Message with offset {} payload is not a string",
                            borrowed_message.offset()
                        )
                    }
                    None => {
                        log::info!(
                            "Message with offset {} without payload",
                            borrowed_message.offset()
                        )
                    }
                }
            }
            Err(e) => log::error!("Failed to process the message: {}", e),
        }
    }
    consumer.unsubscribe();
    log::info!("Unsubscribed outbox stream consumer #{}", n);
}
