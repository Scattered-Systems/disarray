job("Test (crates)") {
    startOn {
        gitPush { 
            branchFilter {
                +"refs/heads/main"
                +"refs/tags/v*.*.*"
            }
        }
        schedule { cron("0 8 * * *") }
    }
    container(displayName = "Rust", image = "rust") {
        shellScript {
            interpreter = "/bin/bash"
            content = """
                apt-get update -y && apt-get upgrade -y
                apt-get install -y protobuf-compiler
                rustup default nightly && rustup target add wasm32-unknown-unknown --toolchain nightly
                cargo login ${'$'}CARGO_REGISTRY_TOKEN
                cargo test --all
            """
        }
    }
}

job("Publish (crates)") {
    startOn {
        gitPush { 
            branchFilter {
                +"refs/tags/v*.*.*"
            }
        }
    }
    container(displayName = "Rust", image = "rust") {
        shellScript {
            interpreter = "/bin/bash"
            content = """
                apt-get update -y && apt-get upgrade -y
                apt-get install -y protobuf-compiler
                rustup default nightly && rustup target add wasm32-unknown-unknown --toolchain nightly
                cargo publish --all-features --color always --jobs 1 --token ${'$'}CARGO_REGISTRY_TOKEN --verbose -p disarray-ledger
                cargo publish --all-features --color always --jobs 1 --token ${'$'}CARGO_REGISTRY_TOKEN --verbose -p disarray-network
                cargo publish --all-features --color always --jobs 1 --token ${'$'}CARGO_REGISTRY_TOKEN --verbose -p disarray-runtime
                cargo publish --all-features --color always --jobs 1 --token ${'$'}CARGO_REGISTRY_TOKEN --verbose -p disarray
            """
        }
    }
}