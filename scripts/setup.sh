#!/usr/bin/env sh
sudo apt update -y && sudo apt upgrade -y && sudo apt install -y protobuf-compiler

rustup default nightly
rustup component add clippy rustfmt --toolchain nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
