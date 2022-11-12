FROM rust:latest as builder-base

RUN apt-get update -y && apt-get upgrade -y && rustup update

RUN apt-get install -y \
    apt-utils \
    protobuf-compiler

RUN rustup default nightly && \
    rustup target add wasm32-unknown-unknown

FROM builder-base as builder

ADD .config /config

ADD . /app
WORKDIR /app

COPY . .
RUN cargo build --release --workspace

FROM builder as testing

RUN cargo test --all --all-features --release --verbose

FROM debian:buster-slim as runner-base

COPY --from=builder /app/target/release/conduit /bin/conduit



CMD [ "conduit" ]