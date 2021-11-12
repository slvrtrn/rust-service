### Prerequisites

* Docker-compose
* [rdkafka](https://github.com/fede1024/rust-rdkafka#installation) compile dependencies

### Prepare env variables

```
cp .env.example .env
```

### Start the docker containers

```
./start.sh
```

### Create the inbox topic

```
docker-compose exec kafka kafka-topics --create \
  --topic InboxTopic \
  --bootstrap-server localhost:9092 \
  --partitions 32 \
  --replication-factor 1
```

### Start the service

```
cargo run
```

### Produce some messages to Kafka

```
docker-compose exec kafka kafka-console-producer \
  --topic InboxTopic \
  --bootstrap-server localhost:9092
```

The expected output should be something like this:

```
[2021-11-12T18:23:13Z INFO  rust_service] Starting the app in development mode
[2021-11-12T18:23:13Z INFO  rust_service::kafka] Starting the inbox stream processing
[2021-11-12T18:23:13Z INFO  rust_service::kafka] Starting the inbox stream processing
[2021-11-12T18:23:13Z INFO  rust_service::kafka] Starting the inbox stream processing
[2021-11-12T18:23:13Z INFO  rust_service::kafka] Starting the inbox stream processing
[2021-11-12T18:23:25Z INFO  rust_service::kafka] Got a new inbox message: hello kafka
```