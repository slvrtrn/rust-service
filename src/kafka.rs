// See https://github.com/fede1024/rust-rdkafka/blob/6fb2c37/examples/asynchronous_processing.rs

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use futures::future;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::{ClientConfig, Message};

use crate::globals::CONFIG;

pub struct KafkaConsumers {
    is_running: Arc<AtomicBool>,
}

impl KafkaConsumers {
    pub fn new() -> Self {
        Self {
            is_running: Arc::new(AtomicBool::new(false)),
        }
    }

    pub async fn start(&self) -> anyhow::Result<()> {
        let consumers_future = (0..CONFIG.kafka_inbox_num_workers)
            .map(|_| tokio::spawn(KafkaConsumers::run_inbox_consumer(self.is_running.clone())))
            .collect::<FuturesUnordered<_>>()
            .for_each(|_| async { () });
        let shutdown_handle_future = tokio::signal::ctrl_c();
        tokio::select! {
            _ = shutdown_handle_future => {
                log::info!("Stopping Kafka consumers");
                self.is_running.swap(false, Ordering::Relaxed)
            },
            _ = consumers_future => unreachable!("Consumer stream is not expected to finish"),
        };
        Ok(())
    }

    async fn run_inbox_consumer(is_running: Arc<AtomicBool>) {
        log::info!("Starting the inbox stream processing");
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
        let mut message_stream = consumer
            .stream()
            .take_while(|_| future::ready(is_running.load(Ordering::Relaxed)));
        while let Some(res) = message_stream.next().await {
            match res {
                Ok(borrowed_message) => {
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
                }
                Err(e) => log::error!("Failed to process the message: {}", e),
            }
        }
    }
}
