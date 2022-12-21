FROM rust:latest as builder-base

RUN apt-get update -y && apt-get upgrade -y && rustup update

RUN apt-get install -y \
    protobuf-compiler

RUN rustup install nightly && \
    rustup target add wasm32-unknown-unknown wasm32-wasi --toolchain nightly
