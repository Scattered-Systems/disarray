job("Build and Publish Crates") {
    container(displayName = "Rust", image = "rust") {
        shellScript {
            interpreter = "/bin/bash"
            content = """
                rustup default nightly && rustup target add wasm32-unknown-unknown && apt-get install -y protobuf-compiler
                cargo login ${'$'}CARGO_REGISTRY_TOKEN
                cargo test --all
            """
        }
    }
}