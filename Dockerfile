FROM rust:stretch

WORKDIR /usr/local/byzan

COPY . .

ENV PATH $PATH:/usr/local/byzan/target/release

RUN apt-get update -y && apt-get install cmake golang -y

RUN rustup default stable
RUN cargo install protobuf-codegen
RUN cargo install grpcio-compiler
RUN cargo build --release
