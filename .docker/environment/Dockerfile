FROM rust:latest as builder-base

RUN apt-get update -y && apt-get upgrade -y && rustup update

RUN apt-get install -y protobuf-compiler

FROM builder-base as builder

ADD . /app
WORKDIR /app

RUN cargo build --workspace

