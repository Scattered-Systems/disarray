#!/usr/bin/env zsh
cargo login $CARGO_REGISTRY_TOKEN
cargo publish --color always --jobs 1 --package disarray-core
cargo publish --color always --jobs 1 --package disarray-network
cargo publish --color always --jobs 1 --package disarray-runtime
cargo publish --color always --jobs 1 --package disarray