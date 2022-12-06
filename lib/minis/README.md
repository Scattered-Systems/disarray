# acme

## About

Acme advances the scsys crate by carefully implementing a number of useful networking utilities designed to eventually
mimic libraries like Python's FastAPI, enabling developers to quickly spin up cloud-native applications written in Rust. 

## Contributors

### _Building from the Source_

#### Clone the repository

    git clone https://gitlab.com/FL03/acme

#### Crate

    cargo build --color always --release --workspace
    cargo test --all-features --color always --release --verbose --workspace

## Developers

### _Examples_

#### netaddr
    fn main() -> scsys::BoxResult {
        println!("{:?}", netaddr![([0, 0, 0, 0), 8080]);
        Ok(())
    }
