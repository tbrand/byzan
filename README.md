<h1 align="center">
  <img src="https://user-images.githubusercontent.com/3483230/45586645-ee774800-b935-11e8-971e-3a72e71db9ba.jpg" />
</h1>

<p align="center">
  <i>Distributed blockchain KVS with high availability written in Rust.</i>
</p>

## Quick start

<i>TODO</i>

## How to use blockchain?

Core technology of blockchain is not for the currency but for the consensus algorithm.
Byzan uses the blockchain feature.

### Why no miners?

Because Byzan is assumed to be used in system internally. (not publically.)
So it's not needed to make some intervals but just verifying the block hash is enough for the purpose.

## API

All APIs are defined as gRPC. You can find the definition at `/proto` directory.

<i>TODO</i>

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
