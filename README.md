# Light Control
![badge](https://framagit.org/adjivas/light_control/badges/master/pipeline.svg)

This service will captures a MQTT motion stream controls a light.

## Play with it!
How to configure the environment:
```shell
cp env.example.sh env.sh
$EDITOR env.sh
source env.sh
```

How to compile:
```shell
cargo build --release
```

How to run the service:
```shell
cargo run --features
```

How to cross compile:
```shell
cross build --target x86_64-unknown-linux-gnu --release
cross build --target armv7-unknown-linux-gnueabihf --release
```
