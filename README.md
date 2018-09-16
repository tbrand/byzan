<h1 align="right">
  <img src="https://user-images.githubusercontent.com/3483230/45586645-ee774800-b935-11e8-971e-3a72e71db9ba.jpg" width="600px" />
</h1>

<p align="right">
  <i>Distributed blockchain KVS with high availability written in Rust.</i>
</p>

## Quick start

You need rust (cargo) environment for building.

After cloning this repo, build it by below.
```bash
cargo build --release
```

Launch a node with binding a gPRC port on 8000 and websocket port on 8001.
```bash
./target/release/byzd --bind_port=8000 --peer_port=8001
```

Upsert data and get it from it.
```bash
# Upsert data
./target/release/byz --connect_port=8000 upsert -k mykey -v myvalue

# Get it from the node
./target/release/byz --connect_port=8000 get -k mykey
```

## How to use blockchain?

Core technology of blockchain is not for the currency but for the consensus algorithm.
Byzan uses the blockchain feature for keeping data consisntency between distributed nodes.

### Why no miners?

Because Byzan is assumed to be used in system internally. (not publically.)
So it's not needed to make some intervals but just verifying the block hash is enough for the purpose.

## API

All APIs are defined as gRPC. You can find the definition at [/proto](https://github.com/tbrand/byzan/tree/master/proto) directory.

## Development

### Build
```bash
cargo build --release
```

### Test
```bash
cargo test
```

### Benchmark (nightly only)
```bash
cargo bench
```

### Generate gRPC fines

Byzan uses [pingcap/grpc-rs](https://github.com/pingcap/grpc-rs) as a gRPC library.
Follow the instraction on it to make a required environment.

```bash
protoc --rust_out=./src/proto/ --grpc_out=./src/proto/ --plugin=protoc-gen-grpc=`which grpc_rust_plugin` proto/byzan.proto
```

## Contributors
- [tbrand](https://github.com/tbrand) Taichiro Suzuki - creator, maintainer
