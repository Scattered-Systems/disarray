#!/usr/bin/env sh
rustup default nightly
rustup component add clippy rustfmt --toolchain nightly
rustup target add wasm32-unknown-unknown wasm32-wasi --toolchain nightly
sudo apt install -y protobuf-compiler