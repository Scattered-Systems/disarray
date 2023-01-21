FROM rust:latest as builder-base

RUN apt-get update -y && apt-get upgrade -y && rustup update

RUN apt-get install -y \
    protobuf-compiler

RUN rustup install nightly && \
    rustup target add wasm32-unknown-unknown wasm32-wasi --toolchain nightly

FROM builder-base as builder

ADD . /workspace
WORKDIR /workspace

COPY . .
RUN cargo build --release -v --workspace

FROM debian:buster-slim as runner-base

RUN apt-get update -y && apt-get upgrade -y

RUN apt-get install -y \
    protobuf-compiler

FROM runner-base as runner

COPY --chown=55 .config /config
VOLUME [ "/config" ]

COPY --from=builder /workspace/target/release/disarray /bin/disarray

FROM runner

ENV MAINNET_PORT=9090 \
    RUST_LOG="info"

EXPOSE 80
EXPOSE ${MAINNET_PORT}

ENTRYPOINT [ "disarray" ]
