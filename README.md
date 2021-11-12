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

### Start the service

```
cargo run
```
